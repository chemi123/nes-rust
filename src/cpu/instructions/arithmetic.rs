use crate::cpu::Cpu;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flags::{Flag, SIGN_BIT, U8_OVERFLOW};

impl Cpu {
    pub(in crate::cpu) fn adc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let accumulator = self.register_a;
        let value = self.peek_byte(addr);
        let sum = (accumulator as u16)
            + (value as u16)
            + ((self.processor_status & (Flag::Carry as u8)) as u16);
        let result = sum as u8;
        self.register_a = result;
        self.set_flag(Flag::Carry, sum >= U8_OVERFLOW);
        self.set_flag(
            Flag::OverFlow,
            (accumulator ^ result) & (value ^ result) & SIGN_BIT != 0,
        );
        self.update_zero_and_negative_flags(result);
    }

    pub(in crate::cpu) fn sbc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let accumulator = self.register_a;
        let value = !self.peek_byte(addr);
        let sum = (accumulator as u16)
            + (value as u16)
            + ((self.processor_status & (Flag::Carry as u8)) as u16);
        let result = sum as u8;
        self.register_a = result;
        self.set_flag(Flag::Carry, sum >= U8_OVERFLOW);
        self.set_flag(
            Flag::OverFlow,
            (accumulator ^ result) & (value ^ result) & SIGN_BIT != 0,
        );
        self.update_zero_and_negative_flags(result);
    }
}
