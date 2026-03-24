use crate::cpu::Cpu;
use crate::cpu::flags::Flag;

impl Cpu {
    pub(in crate::cpu) fn bcc(&mut self) {
        self.branch(self.processor_status & Flag::Carry as u8 == 0);
    }

    pub(in crate::cpu) fn bcs(&mut self) {
        self.branch(self.processor_status & Flag::Carry as u8 != 0);
    }

    pub(in crate::cpu) fn beq(&mut self) {
        self.branch(self.processor_status & Flag::Zero as u8 != 0);
    }

    pub(in crate::cpu) fn bne(&mut self) {
        self.branch(self.processor_status & Flag::Zero as u8 == 0);
    }

    pub(in crate::cpu) fn bmi(&mut self) {
        self.branch(self.processor_status & Flag::Negative as u8 != 0);
    }

    pub(in crate::cpu) fn bpl(&mut self) {
        self.branch(self.processor_status & Flag::Negative as u8 == 0);
    }

    pub(in crate::cpu) fn bvc(&mut self) {
        self.branch(self.processor_status & Flag::OverFlow as u8 == 0);
    }

    pub(in crate::cpu) fn bvs(&mut self) {
        self.branch(self.processor_status & Flag::OverFlow as u8 != 0);
    }

    fn branch(&mut self, condition: bool) {
        let offset = self.fetch_byte() as i8;
        if condition {
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }
}
