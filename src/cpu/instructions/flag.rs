use crate::cpu::Cpu;
use crate::cpu::flags::Flag;

impl Cpu {
    pub(in crate::cpu) fn clc(&mut self) {
        self.set_flag(Flag::Carry, false);
    }

    pub(in crate::cpu) fn sec(&mut self) {
        self.set_flag(Flag::Carry, true);
    }

    pub(in crate::cpu) fn cld(&mut self) {
        self.set_flag(Flag::DecimalMode, false);
    }

    pub(in crate::cpu) fn sed(&mut self) {
        self.set_flag(Flag::DecimalMode, true);
    }

    pub(in crate::cpu) fn cli(&mut self) {
        self.set_flag(Flag::InterruptDisable, false);
    }

    pub(in crate::cpu) fn sei(&mut self) {
        self.set_flag(Flag::InterruptDisable, true);
    }

    pub(in crate::cpu) fn clv(&mut self) {
        self.set_flag(Flag::OverFlow, false);
    }
}
