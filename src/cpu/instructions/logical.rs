use crate::cpu::Cpu;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flags::{Flag, SIGN_BIT};

impl Cpu {
    pub(in crate::cpu) fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a &= self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn ora(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a |= self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn eor(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a ^= self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn bit(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        self.set_flag(Flag::Zero, self.register_a & value == 0);
        self.set_flag(Flag::Negative, value & SIGN_BIT != 0);
        self.set_flag(Flag::OverFlow, value & (Flag::OverFlow as u8) != 0);
    }
}
