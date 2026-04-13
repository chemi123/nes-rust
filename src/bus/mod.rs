use crate::cartridge::{PRG_ROM_PAGE_SIZE, Rom};
use crate::cpu::bus_access::Bus;
use crate::memory::Memory;
use crate::ppu::Ppu;

#[cfg(test)]
mod tests;

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

pub(crate) struct NESBus {
    memory: Memory,
    rom: Rom,
    pub(crate) ppu: Ppu,
}

impl NESBus {
    pub(crate) fn new(mut rom: Rom) -> Self {
        let chr_rom = std::mem::take(&mut rom.chr_rom);
        let ppu = Ppu::new(chr_rom, rom.screen_mirroring.clone());
        NESBus {
            memory: Memory::new(),
            rom,
            ppu,
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

impl Bus for NESBus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => self.memory.read(addr & RAM_MIRROR_MASK),
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                self.ppu.read_register(addr & PPU_MIRROR_MASK)
            }
            APU_IO_START..=APU_IO_END => todo!("APU"),
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
            APU_IO_START..=APU_IO_END => todo!("APU"),
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
        self.ppu.tick(cycles as u16 * 3)
    }

    fn poll_nmi_status(&mut self) -> bool {
        self.ppu.poll_nmi_interrupt()
    }
}
