mod address_register;
mod controller_register;

use crate::{
    Mirroring,
    ppu::{address_register::AddressRegister, controller_register::ControllerRegister},
};

// PPU メモリマップ
const CHR_ROM_START: u16 = 0x0000;
const CHR_ROM_END: u16 = 0x1FFF;
const VRAM_START: u16 = 0x2000;
const VRAM_END: u16 = 0x2FFF;
const PALETTE_START: u16 = 0x3F00;
const PALETTE_END: u16 = 0x3FFF;

// VRAMミラーリング
const VRAM_MIRROR_MASK: u16 = 0b0010_1111_1111_1111; // 0x3000→0x2000 のミラー解決
const NAMETABLE_SIZE: u16 = 0x0400; // 1 Nametable = 1KiB

pub(crate) struct Ppu {
    pub(crate) chr_rom: Vec<u8>,
    pub(crate) palette_table: [u8; 32],
    pub(crate) vram: [u8; 2048],
    pub(crate) oam_data: [u8; 256],
    pub(crate) mirroring: Mirroring,
    pub(crate) controller_register: ControllerRegister,
    address_register: AddressRegister,
    internal_data_buf: u8,
}

impl Ppu {
    pub(crate) fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Ppu {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
            mirroring,
            controller_register: ControllerRegister::new(),
            address_register: AddressRegister::new(),
            internal_data_buf: 0,
        }
    }

    pub(crate) fn read_memory(&mut self) -> u8 {
        let addr = self.address_register.get();
        self.address_register
            .increment(self.controller_register.vram_address_step());

        match addr {
            CHR_ROM_START..=CHR_ROM_END => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            VRAM_START..=VRAM_END => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_address(addr)];
                result
            }
            PALETTE_START..=PALETTE_END => self.palette_table[(addr - PALETTE_START) as usize],
            _ => 0,
        }
    }

    pub(crate) fn write_to_memory(&mut self, data: u8) {
        let addr = self.address_register.get();
        self.address_register
            .increment(self.controller_register.vram_address_step());

        match addr {
            CHR_ROM_START..=CHR_ROM_END => {} // CHR ROMはreadonlyなので何もしない
            VRAM_START..=VRAM_END => {
                self.vram[self.mirror_vram_address(addr) as usize] = data;
            }
            PALETTE_START..=PALETTE_END => {
                self.palette_table[(addr - PALETTE_START) as usize] = data;
            }
            _ => {}
        }
    }

    pub(crate) fn write_to_address_register(&mut self, data: u8) {
        self.address_register.write(data);
    }

    pub(crate) fn write_to_controller_register(&mut self, data: u8) {
        self.controller_register.update(data);
    }

    // PPUアドレス (0x2000-0x2FFF) を物理VRAMインデックス (0-2047) に変換する。
    //
    // PPUは4つのNametable分のアドレス空間 (4KiB) を持つが、
    // 物理VRAMは2KiB (2つ分) しかないため、ミラーリングで対応する。
    //
    // Vertical:   [A][B][A][B]  横スクロール向け (左右が独立)
    // Horizontal: [A][A][B][B]  縦スクロール向け (上下が独立)
    //
    //   Nametable:    0      1      2      3
    //   PPUアドレス:  0x2000 0x2400 0x2800 0x2C00
    //   Vertical:     A      B      A      B
    //   Horizontal:   A      A      B      B
    fn mirror_vram_address(&self, addr: u16) -> usize {
        // 0x3000-0x3EFFは0x2000-0x2EFFのミラー
        let mirrored = (addr & VRAM_MIRROR_MASK) - VRAM_START;
        let nametable_index = mirrored / NAMETABLE_SIZE;

        match (&self.mirroring, nametable_index) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => {
                (mirrored - NAMETABLE_SIZE * 2) as usize
            }
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 2) => {
                (mirrored - NAMETABLE_SIZE) as usize
            }
            (Mirroring::Horizontal, 3) => (mirrored - NAMETABLE_SIZE * 2) as usize,
            _ => mirrored as usize,
        }
    }
}
