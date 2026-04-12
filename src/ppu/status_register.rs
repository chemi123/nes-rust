use bitflags::bitflags;

bitflags! {
    // PPU Status Register (0x2002) - Read only
    //
    // 7  bit  0
    // ---- ----
    // VSO. ....
    // |||| ||||
    // |||+-++++- PPU open bus (未使用)
    // ||+------- Sprite overflow
    // |+-------- Sprite 0 Hit
    // +--------- VBlank has started (0: not in vblank; 1: in vblank)
    pub(crate) struct StatusRegister: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_ZERO_HIT = 0b0100_0000;
        const VBLANK_STARTED  = 0b1000_0000;
    }
}

impl StatusRegister {
    pub(crate) fn new() -> Self {
        StatusRegister::from_bits_truncate(0)
    }
}
