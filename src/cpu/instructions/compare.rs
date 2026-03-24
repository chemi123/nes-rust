use crate::cpu::Cpu;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flags::Flag;

impl Cpu {
    pub(in crate::cpu) fn cmp(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        let result = self.register_a.wrapping_sub(value);
        self.set_flag(Flag::Carry, self.register_a >= value);
        self.update_zero_and_negative_flags(result);
    }

    pub(in crate::cpu) fn cpx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        let result = self.register_x.wrapping_sub(value);
        self.set_flag(Flag::Carry, self.register_x >= value);
        self.update_zero_and_negative_flags(result);
    }

    pub(in crate::cpu) fn cpy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        let result = self.register_y.wrapping_sub(value);
        self.set_flag(Flag::Carry, self.register_y >= value);
        self.update_zero_and_negative_flags(result);
    }
}
