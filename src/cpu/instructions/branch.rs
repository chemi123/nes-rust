use crate::cpu::Cpu;
use crate::cpu::flags::Flag;

impl Cpu {
    pub(in crate::cpu) fn bcc(&mut self) {
        self.branch(!self.has_flag(Flag::Carry));
    }

    pub(in crate::cpu) fn bcs(&mut self) {
        self.branch(self.has_flag(Flag::Carry));
    }

    pub(in crate::cpu) fn beq(&mut self) {
        self.branch(self.has_flag(Flag::Zero));
    }

    pub(in crate::cpu) fn bne(&mut self) {
        self.branch(!self.has_flag(Flag::Zero));
    }

    pub(in crate::cpu) fn bmi(&mut self) {
        self.branch(self.has_flag(Flag::Negative));
    }

    pub(in crate::cpu) fn bpl(&mut self) {
        self.branch(!self.has_flag(Flag::Negative));
    }

    pub(in crate::cpu) fn bvc(&mut self) {
        self.branch(!self.has_flag(Flag::OverFlow));
    }

    pub(in crate::cpu) fn bvs(&mut self) {
        self.branch(self.has_flag(Flag::OverFlow));
    }

    fn branch(&mut self, condition: bool) {
        let offset = self.fetch_byte() as i8;
        if condition {
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }
}
