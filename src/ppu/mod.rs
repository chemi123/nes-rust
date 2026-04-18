pub(crate) mod frame;
pub(crate) mod palette;
pub(crate) mod register;
pub(crate) mod render;
#[cfg(test)]
mod tests;

use crate::{
    Mirroring,
    ppu::register::{
        address::AddressRegister, controller::ControllerRegister, status::StatusRegister,
    },
};

// PPU レジスタ (CPU側アドレス 0x2000-0x2007 の下位3bit)
const REG_CONTROLLER: u16 = 0x00; // 0x2000: コントローラ (W)
const REG_MASK: u16 = 0x01; // 0x2001: マスク (W)
const REG_STATUS: u16 = 0x02; // 0x2002: ステータス (R)
const REG_OAM_ADDR: u16 = 0x03; // 0x2003: OAMアドレス (W)
const REG_OAM_DATA: u16 = 0x04; // 0x2004: OAMデータ (RW)
const REG_SCROLL: u16 = 0x05; // 0x2005: スクロール (W)
const REG_ADDRESS: u16 = 0x06; // 0x2006: PPUアドレス (W)
const REG_DATA: u16 = 0x07; // 0x2007: PPUデータ (RW)

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

pub(crate) const CYCLES_PER_SCANLINE: usize = 341;
pub(crate) const VBLANK_SCANLINE: u16 = 241;
pub(crate) const SCANLINES_PER_FRAME: u16 = 262;

pub struct Ppu {
    pub(super) chr_rom: Vec<u8>,
    pub(super) palette_table: [u8; 32],
    pub(super) vram: [u8; 2048],
    pub(super) oam_data: [u8; 256],
    oam_address: u8,
    mirroring: Mirroring,
    pub(super) controller_register: ControllerRegister,
    status_register: StatusRegister,
    pub(crate) nmi_interrupt: bool,
    address_register: AddressRegister,
    internal_data_buf: u8,
    cycles: usize,
    scanline: u16,
}

impl Ppu {
    pub(crate) fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Ppu {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
            oam_address: 0,
            mirroring,
            controller_register: ControllerRegister::new(),
            status_register: StatusRegister::new(),
            address_register: AddressRegister::new(),
            internal_data_buf: 0,
            cycles: 0,
            scanline: 0,
            nmi_interrupt: false,
        }
    }

    // NMI割り込みが発生しているか確認し、確認後にフラグをクリアする
    pub(crate) fn poll_nmi_interrupt(&mut self) -> bool {
        let nmi = self.nmi_interrupt;
        self.nmi_interrupt = false;
        nmi
    }

    // CPU からの PPU レジスタ読み出し (0x2000-0x2007)
    // register: アドレスの下位3bit (0-7)
    pub(crate) fn read_register(&mut self, register: u16) -> u8 {
        match register {
            REG_STATUS => self.read_status(),
            REG_OAM_DATA => self.oam_data[self.oam_address as usize],
            REG_DATA => self.read_memory(),
            _ => todo!("PPU register read: {:#06X}", register),
        }
    }

    // CPU からの PPU レジスタ書き込み (0x2000-0x2007)
    // register: アドレスの下位3bit (0-7)
    pub(crate) fn write_to_register(&mut self, register: u16, value: u8) {
        match register {
            REG_CONTROLLER => self.write_to_controller_register(value),
            REG_MASK => {} // TODO: マスクレジスタの実装
            REG_OAM_ADDR => self.oam_address = value,
            REG_OAM_DATA => {
                self.oam_data[self.oam_address as usize] = value;
                self.oam_address = self.oam_address.wrapping_add(1);
            }
            REG_SCROLL => {} // TODO: スクロールレジスタの実装
            REG_ADDRESS => self.write_to_address_register(value),
            REG_DATA => self.write_to_memory(value),
            _ => todo!("PPU register write: {:#06X}", register),
        }
    }

    // OAM DMA ($4014 経由): 256 byte を現在の oam_address を起点に書き込む。
    // 実機ではリングバッファ的に oam_address が一周するため wrapping_add を使う。
    pub(crate) fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for byte in data {
            self.oam_data[self.oam_address as usize] = *byte;
            self.oam_address = self.oam_address.wrapping_add(1);
        }
    }

    // PPU Status (0x2002) の読み出し
    // 読み出し時にVBlankフラグをクリアし、アドレスラッチをリセットする
    fn read_status(&mut self) -> u8 {
        let data = self.status_register.bits();
        self.status_register.remove(StatusRegister::VBLANK_STARTED);
        self.address_register.reset_latch();
        data
    }

    fn read_memory(&mut self) -> u8 {
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
            PALETTE_START..=PALETTE_END => self.palette_table[Self::mirror_palette_address(addr)],
            _ => 0,
        }
    }

    fn write_to_memory(&mut self, data: u8) {
        let addr = self.address_register.get();
        self.address_register
            .increment(self.controller_register.vram_address_step());

        match addr {
            CHR_ROM_START..=CHR_ROM_END => {} // CHR ROMはreadonlyなので何もしない
            VRAM_START..=VRAM_END => {
                self.vram[self.mirror_vram_address(addr) as usize] = data;
            }
            PALETTE_START..=PALETTE_END => {
                self.palette_table[Self::mirror_palette_address(addr)] = data;
            }
            _ => {}
        }
    }

    fn write_to_address_register(&mut self, data: u8) {
        self.address_register.write(data);
    }

    fn write_to_controller_register(&mut self, data: u8) {
        let is_nmi_off = !self
            .controller_register
            .contains(ControllerRegister::GENERATE_NMI);
        self.controller_register.update(data);
        let is_nmi_enabled_by_update = is_nmi_off
            && self
                .controller_register
                .contains(ControllerRegister::GENERATE_NMI);

        if is_nmi_enabled_by_update && self.is_in_vblank() {
            self.nmi_interrupt = true;
        }
    }

    pub(crate) fn tick(&mut self, cycles: u16) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= CYCLES_PER_SCANLINE {
            self.cycles -= CYCLES_PER_SCANLINE;
            self.scanline += 1;

            if self.scanline == VBLANK_SCANLINE {
                self.status_register.insert(StatusRegister::VBLANK_STARTED);
                if self
                    .controller_register
                    .contains(ControllerRegister::GENERATE_NMI)
                {
                    self.nmi_interrupt = true;
                }
            }

            if self.scanline >= SCANLINES_PER_FRAME {
                self.scanline = 0;
                self.nmi_interrupt = false;
                self.status_register.remove(StatusRegister::VBLANK_STARTED);
                return true;
            }
        }
        false
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

    // パレットアドレス (0x3F00-0x3FFF) を palette_table のインデックス (0-31) に変換する。
    //
    // パレットは32バイトだが256バイト分のアドレス空間にマッピングされるため % 32 が必要。
    // また、スプライトパレットの透明色 (0x10, 0x14, 0x18, 0x1C) は
    // 背景パレットの透明色 (0x00, 0x04, 0x08, 0x0C) のミラーとなる。
    fn mirror_palette_address(addr: u16) -> usize {
        let index = (addr - PALETTE_START) as usize % 32;
        // スプライトパレットの透明色は背景パレットの透明色にミラー
        match index {
            0x10 | 0x14 | 0x18 | 0x1C => index - 0x10,
            _ => index,
        }
    }

    fn is_in_vblank(&self) -> bool {
        VBLANK_SCANLINE <= self.scanline && self.scanline < SCANLINES_PER_FRAME
    }
}

#[cfg(test)]
impl Ppu {
    pub(crate) fn scanline(&self) -> u16 {
        self.scanline
    }

    pub(crate) fn cycles(&self) -> usize {
        self.cycles
    }

    pub(crate) fn nmi_interrupt(&self) -> bool {
        self.nmi_interrupt
    }

    pub(crate) fn set_nmi_interrupt(&mut self, value: bool) {
        self.nmi_interrupt = value;
    }
}
