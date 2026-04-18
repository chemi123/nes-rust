use crate::cartridge::{PRG_ROM_PAGE_SIZE, Rom};
use crate::cpu::Clock;
use crate::cpu::bus_access::Bus;
use crate::joypad::Joypad;
use crate::memory::Memory;
use crate::ppu::Ppu;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const RAM_MIRROR_MASK: u16 = 0x07FF;

const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;
const PPU_MIRROR_MASK: u16 = 0x0007;

const APU_IO_START: u16 = 0x4000;
const APU_IO_END: u16 = 0x401F;

// APU_IO 範囲内だが個別ハンドリング
const OAM_DMA: u16 = 0x4014;
const JOYPAD_1: u16 = 0x4016;
const JOYPAD_2: u16 = 0x4017;

pub(crate) const CARTRIDGE_START: u16 = 0x8000;
const CARTRIDGE_END: u16 = 0xFFFF;

pub struct NESBus {
    memory: Memory,
    rom: Rom,
    ppu: Ppu,
    joypad1: Joypad,
}

impl NESBus {
    pub fn new(mut rom: Rom) -> Self {
        let chr_rom = std::mem::take(&mut rom.chr_rom);
        let ppu = Ppu::new(chr_rom, rom.screen_mirroring.clone());
        NESBus {
            memory: Memory::new(),
            rom,
            ppu,
            joypad1: Joypad::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_program(program: &[u8]) -> Self {
        let mut rom = Rom::with_program(program);
        let chr_rom = std::mem::take(&mut rom.chr_rom);
        let ppu = Ppu::new(chr_rom, rom.screen_mirroring.clone());
        NESBus {
            memory: Memory::new(),
            rom,
            ppu,
            joypad1: Joypad::new(),
        }
    }

    pub fn ppu(&self) -> &Ppu {
        &self.ppu
    }

    pub fn joypad1_mut(&mut self) -> &mut Joypad {
        &mut self.joypad1
    }

    fn read_cartridge(&self, mut addr: u16) -> u8 {
        addr -= CARTRIDGE_START;
        if self.rom.prg_rom.len() == PRG_ROM_PAGE_SIZE && addr >= PRG_ROM_PAGE_SIZE as u16 {
            addr %= PRG_ROM_PAGE_SIZE as u16;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl Bus for NESBus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => self.memory.read(addr & RAM_MIRROR_MASK),
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                self.ppu.read_register(addr & PPU_MIRROR_MASK)
            }
            JOYPAD_1 => self.joypad1.read(),
            JOYPAD_2 => 0,                  // 2P 未対応
            APU_IO_START..=APU_IO_END => 0, // TODO: APU
            CARTRIDGE_START..=CARTRIDGE_END => self.read_cartridge(addr),
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            RAM_START..=RAM_END => self.memory.write(addr & RAM_MIRROR_MASK, value),
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                self.ppu.write_to_register(addr & PPU_MIRROR_MASK, value)
            }
            OAM_DMA => {
                // CPU メモリの (value << 8) から 256 byte を PPU OAM へ転送。
                // 実機ではここで CPU が ~513 サイクル停止するが、現状はサイクル未計上。
                let mut buffer = [0u8; 256];
                let base = (value as u16) << 8;
                for offset in 0..256u16 {
                    buffer[offset as usize] = self.read(base + offset);
                }
                self.ppu.write_oam_dma(&buffer);
            }
            JOYPAD_1 => self.joypad1.write(value),
            JOYPAD_2 => {}                  // APUフレームカウンタ兼用、未対応
            APU_IO_START..=APU_IO_END => {} // TODO: APU
            CARTRIDGE_START..=CARTRIDGE_END => {
                log::warn!(
                    "ignoring write to Cartridge ROM space: addr={:#06X} value={:#04X}",
                    addr,
                    value
                );
            }
            _ => {}
        }
    }
}

impl Clock for NESBus {
    // 戻り値は NMI エッジ (VBlank 開始) の立ち上がりを示す。
    // CPU 側はこれを拾って on_frame コールバックに制御を渡す。
    fn tick(&mut self, cycles: u8) -> bool {
        let nmi_before = self.ppu.nmi_interrupt;
        self.ppu.tick(cycles as u16 * 3);
        let nmi_after = self.ppu.nmi_interrupt;
        !nmi_before && nmi_after
    }

    fn poll_nmi_status(&mut self) -> bool {
        self.ppu.poll_nmi_interrupt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::Clock;
    use crate::cpu::bus_access::Bus;
    use crate::ppu::register::controller::ControllerRegister;

    #[test]
    fn test_poll_nmi_status_returns_true_and_clears() {
        let mut bus = NESBus::with_program(&[0x00]);
        // Bus経由でNMI有効化 (0x2000 = PPU_CTRL)
        bus.write(0x2000, ControllerRegister::GENERATE_NMI.bits());

        // VBlankに到達させる (scanline 241)
        // 1 CPUサイクル = 3 PPUサイクル、1スキャンライン = 341 PPUサイクル
        // 241スキャンライン × 341 PPUサイクル = 82,181 PPUサイクル
        // 82,181 / 3 = 27,393.67 → 27,394 CPUサイクルでscanline 241に到達
        //
        // ただしフレーム完了(scanline 262)まで進めるとNMIがクリアされるので、
        // VBlank開始直後で止める必要がある。
        // 1回のtickで1スキャンライン分(341 PPUサイクル = 113.67 CPUサイクル)ずつ進める。
        // 114 CPUサイクル = 342 PPUサイクル > 341 なのでスキャンラインが1つ進む。
        for _ in 0..241 {
            bus.tick(114);
        }

        // 1回目: trueを返してクリア
        assert!(bus.poll_nmi_status());
        // 2回目: クリア済みなのでfalse
        assert!(!bus.poll_nmi_status());
    }
}
