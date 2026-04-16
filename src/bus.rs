use crate::cartridge::{PRG_ROM_PAGE_SIZE, Rom};
use crate::cpu::bus_access::Bus;
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

pub(crate) const CARTRIDGE_START: u16 = 0x8000;
const CARTRIDGE_END: u16 = 0xFFFF;

pub struct NESBus<'a> {
    memory: Memory,
    rom: Rom,
    pub(crate) ppu: Ppu,
    gameloop_callback: Box<dyn FnMut(&Ppu) + 'a>,
}

impl<'a> NESBus<'a> {
    pub fn new<F>(mut rom: Rom, gameloop_callback: F) -> Self
    where
        F: FnMut(&Ppu) + 'a,
    {
        let chr_rom = std::mem::take(&mut rom.chr_rom);
        let ppu = Ppu::new(chr_rom, rom.screen_mirroring.clone());
        NESBus {
            memory: Memory::new(),
            rom,
            ppu,
            gameloop_callback: Box::new(gameloop_callback),
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
            gameloop_callback: Box::new(|_| {}),
        }
    }

    fn read_cartridge(&self, mut addr: u16) -> u8 {
        addr -= CARTRIDGE_START;
        if self.rom.prg_rom.len() == PRG_ROM_PAGE_SIZE && addr >= PRG_ROM_PAGE_SIZE as u16 {
            addr %= PRG_ROM_PAGE_SIZE as u16;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl<'a> Bus for NESBus<'a> {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => self.memory.read(addr & RAM_MIRROR_MASK),
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                self.ppu.read_register(addr & PPU_MIRROR_MASK)
            }
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
            APU_IO_START..=APU_IO_END => {} // TODO: APU
            CARTRIDGE_START..=CARTRIDGE_END => {
                panic!(
                    "Attempt to write to Cartridge ROM space: addr={:#06X}",
                    addr
                )
            }
            _ => {}
        }
    }

    fn tick(&mut self, cycles: u8) -> bool {
        let nmi_before = self.ppu.nmi_interrupt;
        let frame_complete = self.ppu.tick(cycles as u16 * 3);
        let nmi_after = self.ppu.nmi_interrupt;

        if !nmi_before && nmi_after {
            (self.gameloop_callback)(&self.ppu);
        }

        frame_complete
    }

    fn poll_nmi_status(&mut self) -> bool {
        self.ppu.poll_nmi_interrupt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
