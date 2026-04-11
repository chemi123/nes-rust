use bitflags::bitflags;

bitflags! {
    pub(crate) struct ControllerRegister: u8 {
        const NEMETABLE_1             = 0b0000_0001;
        const NEMETABLE_2             = 0b0000_0010;
        const VRAM_ADD_INCREMENT      = 0b0000_0100;
        const SPRITE_PATTERN_ADDR     = 0b0000_1000;
        const BACKGROUND_PATTERN_ADDR = 0b0001_0000;
        const SPRITE_SIZE             = 0b0010_0000;
        const MASTER_SLAVE_SELECT     = 0b0100_0000;
        const GENERATE_NMI            = 0b1000_0000;
    }
}

impl ControllerRegister {
    pub(crate) fn new() -> Self {
        ControllerRegister::from_bits_truncate(0)
    }

    pub(crate) fn vram_address_step(&self) -> u8 {
        if self.contains(ControllerRegister::VRAM_ADD_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub(crate) fn update(&mut self, data: u8) {
        *self = ControllerRegister::from_bits_truncate(data);
    }
}
