const PPU_ADDR_MASK: u16 = 0b0011_1111_1111_1111; // 14bit

pub(crate) struct AddressRegister {
    hi: u8,
    lo: u8,
    next_is_hi: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        AddressRegister {
            hi: 0,
            lo: 0,
            next_is_hi: true,
        }
    }

    pub fn get(&self) -> u16 {
        (self.hi as u16) << 8 | self.lo as u16
    }

    pub fn write(&mut self, data: u8) {
        if self.next_is_hi {
            self.hi = data;
        } else {
            self.lo = data;
        }
        self.clamp_to_valid_range();
        self.next_is_hi = !self.next_is_hi;
    }

    pub fn increment(&mut self, step: u8) {
        let prev_lo = self.lo;
        self.lo = self.lo.wrapping_add(step);
        if prev_lo > self.lo {
            self.hi = self.hi.wrapping_add(1);
        }
        self.clamp_to_valid_range();
    }

    pub fn reset_latch(&mut self) {
        self.next_is_hi = true;
    }

    fn clamp_to_valid_range(&mut self) {
        let addr = self.get() & PPU_ADDR_MASK;
        self.hi = (addr >> 8) as u8;
        self.lo = addr as u8;
    }
}
