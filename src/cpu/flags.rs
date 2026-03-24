use super::Cpu;

pub(super) const U8_OVERFLOW: u16 = 0x0100;
pub(super) const SIGN_BIT: u8 = 0b1000_0000;
pub(super) const LOW_BIT: u8 = 0b0000_0001;

pub(super) enum Flag {
    Carry = 0b0000_0001,
    Zero = 0b0000_0010,
    InterruptDisable = 0b0000_0100,
    DecimalMode = 0b0000_1000,
    Break = 0b0001_0000,
    Unused = 0b0010_0000,
    OverFlow = 0b0100_0000,
    Negative = 0b1000_0000,
}

impl Cpu {
    pub(super) fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.processor_status |= flag as u8;
        } else {
            self.processor_status &= !(flag as u8);
        }
    }
}
