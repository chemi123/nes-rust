use crate::cartridge::{PRG_ROM_PAGE_SIZE, Rom};
use crate::cpu::bus_access::Bus;
use crate::memory::Memory;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const RAM_MIRROR_MASK: u16 = 0x07FF;

const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;
const PPU_MIRROR_MASK: u16 = 0x2007;

const APU_IO_START: u16 = 0x4000;
const APU_IO_END: u16 = 0x401F;

pub(crate) const CARTRIDGE_START: u16 = 0x8000;
const CARTRIDGE_END: u16 = 0xFFFF;

pub(crate) struct NESBus {
    memory: Memory,
    rom: Rom,
}

impl NESBus {
    pub(crate) fn new(rom: Rom) -> Self {
        NESBus {
            memory: Memory::new(),
            rom,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_program(program: &[u8]) -> Self {
        NESBus {
            memory: Memory::new(),
            rom: Rom::with_program(program),
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
    fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => self.memory.read(addr & RAM_MIRROR_MASK),
            PPU_REGISTERS_START..=PPU_REGISTERS_END => todo!("PPU"),
            APU_IO_START..=APU_IO_END => todo!("APU"),
            CARTRIDGE_START..=CARTRIDGE_END => self.read_cartridge(addr),
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            RAM_START..=RAM_END => self.memory.write(addr & RAM_MIRROR_MASK, value),
            PPU_REGISTERS_START..=PPU_REGISTERS_END => todo!("PPU"),
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
}
