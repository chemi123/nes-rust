mod bus;
mod cartridge;
mod cpu;
// mod debug;
mod memory;
mod ppu;

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
pub(crate) enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}
