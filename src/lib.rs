pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod joypad;
mod memory;
pub mod ppu;
pub mod screen;

// =============================================================
// NES メモリマップ
// =============================================================
//
// CPU メモリマップ               PPU メモリマップ
// ---------------------------   ---------------------------
// 0x0000 - 0x1FFF  RAM          0x0000 - 0x1FFF  CHR ROM
// 0x2000 - 0x2007  PPU I/O      0x2000 - 0x2FFF  VRAM
// 0x4000 - 0x401F  APU/I/O      0x3F00 - 0x3FFF  パレット
// 0x8000 - 0xFFFF  PRG ROM
// =============================================================

#[derive(Debug, PartialEq, Clone)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(Debug)]
pub enum NesError {
    RomRead(std::io::Error),
    RomParse(String),
    Sdl(String),
    UnknownOpcode { opcode: u8, pc: u16 },
}

impl std::fmt::Display for NesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NesError::RomRead(error) => write!(f, "failed to read ROM file: {}", error),
            NesError::RomParse(message) => write!(f, "failed to parse ROM: {}", message),
            NesError::Sdl(message) => write!(f, "SDL error: {}", message),
            NesError::UnknownOpcode { opcode, pc } => {
                // 実機の JAM opcode に相当するハング状態として扱う
                write!(f, "unknown opcode {:#04X} at PC={:#06X}", opcode, pc)
            }
        }
    }
}

impl std::error::Error for NesError {}

impl From<std::io::Error> for NesError {
    fn from(error: std::io::Error) -> Self {
        NesError::RomRead(error)
    }
}
