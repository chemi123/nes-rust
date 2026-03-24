use crate::cpu::Cpu;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flags::{Flag, LOW_BIT, SIGN_BIT};

impl Cpu {
    pub(in crate::cpu) fn asl(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.set_flag(Flag::Carry, self.register_a & SIGN_BIT != 0);
            self.register_a <<= 1;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.peek_byte(addr);
            self.set_flag(Flag::Carry, value & SIGN_BIT != 0);
            let result = value << 1;
            self.write_byte(addr, result);
            self.update_zero_and_negative_flags(result);
        }
    }

    pub(in crate::cpu) fn lsr(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Accumulator {
            self.set_flag(Flag::Carry, self.register_a & LOW_BIT != 0);
            self.register_a >>= 1;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.peek_byte(addr);
            self.set_flag(Flag::Carry, value & LOW_BIT != 0);
            let result = value >> 1;
            self.write_byte(addr, result);
            self.update_zero_and_negative_flags(result);
        }
    }

    pub(in crate::cpu) fn rol(&mut self, mode: AddressingMode) {
        let old_carry = self.processor_status & (Flag::Carry as u8);
        if mode == AddressingMode::Accumulator {
            self.set_flag(Flag::Carry, self.register_a & SIGN_BIT != 0);
            self.register_a = (self.register_a << 1) | old_carry;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.peek_byte(addr);
            self.set_flag(Flag::Carry, value & SIGN_BIT != 0);
            let result = (value << 1) | old_carry;
            self.write_byte(addr, result);
            self.update_zero_and_negative_flags(result);
        }
    }

    pub(in crate::cpu) fn ror(&mut self, mode: AddressingMode) {
        let old_carry = self.processor_status & (Flag::Carry as u8);
        if mode == AddressingMode::Accumulator {
            self.set_flag(Flag::Carry, self.register_a & LOW_BIT != 0);
            self.register_a = (self.register_a >> 1) | (old_carry << 7);
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.peek_byte(addr);
            self.set_flag(Flag::Carry, value & LOW_BIT != 0);
            let result = (value >> 1) | (old_carry << 7);
            self.write_byte(addr, result);
            self.update_zero_and_negative_flags(result);
        }
    }
}
