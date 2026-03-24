use crate::cpu::flags::{Flag, LOW_BIT, SIGN_BIT, U8_OVERFLOW};

use super::Cpu;
use super::addressing_mode::AddressingMode;

impl Cpu {
    pub(super) fn adc(&mut self, mode: AddressingMode) {
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

    pub(super) fn sbc(&mut self, mode: AddressingMode) {
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

    pub(super) fn asl(&mut self, mode: AddressingMode) {
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

    pub(super) fn rol(&mut self, mode: AddressingMode) {
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

    pub(super) fn ror(&mut self, mode: AddressingMode) {
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

    pub(super) fn lsr(&mut self, mode: AddressingMode) {
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

    pub(super) fn cmp(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        let result = self.register_a.wrapping_sub(value);
        self.set_flag(Flag::Carry, self.register_a >= value);
        self.update_zero_and_negative_flags(result);
    }

    pub(super) fn cpx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        let result = self.register_x.wrapping_sub(value);
        self.set_flag(Flag::Carry, self.register_x >= value);
        self.update_zero_and_negative_flags(result);
    }

    pub(super) fn cpy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        let result = self.register_y.wrapping_sub(value);
        self.set_flag(Flag::Carry, self.register_y >= value);
        self.update_zero_and_negative_flags(result);
    }

    pub(super) fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a &= self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(super) fn ora(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a |= self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(super) fn eor(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a ^= self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    // Load instructions
    pub(super) fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a = self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(super) fn ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_x = self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(super) fn ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_y = self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_y);
    }

    // Store instructions
    pub(super) fn sta(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write_byte(addr, self.register_a);
    }

    pub(super) fn stx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write_byte(addr, self.register_x);
    }

    pub(super) fn sty(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write_byte(addr, self.register_y);
    }

    // Transfer instructions
    pub(super) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(super) fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub(super) fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(super) fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(super) fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(super) fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    // Increment/Decrement instructions
    pub(super) fn inc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.peek_byte(addr).wrapping_add(1);
        self.write_byte(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    pub(super) fn dec(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.peek_byte(addr).wrapping_sub(1);
        self.write_byte(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    pub(super) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(super) fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub(super) fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(super) fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    // Flag
    pub(super) fn clc(&mut self) {
        self.set_flag(Flag::Carry, false);
    }

    pub(super) fn sec(&mut self) {
        self.set_flag(Flag::Carry, true);
    }

    pub(super) fn cld(&mut self) {
        self.set_flag(Flag::DecimalMode, false);
    }

    pub(super) fn sed(&mut self) {
        self.set_flag(Flag::DecimalMode, true);
    }

    pub(super) fn cli(&mut self) {
        self.set_flag(Flag::InterruptDisable, false);
    }

    pub(super) fn sei(&mut self) {
        self.set_flag(Flag::InterruptDisable, true);
    }

    pub(super) fn clv(&mut self) {
        self.set_flag(Flag::OverFlow, false);
    }

    pub(super) fn pha(&mut self) {
        self.push_byte(self.register_a);
    }

    pub(super) fn php(&mut self) {
        self.push_byte(self.processor_status | Flag::Break as u8 | Flag::Unused as u8);
    }

    pub(super) fn pla(&mut self) {
        self.register_a = self.pop_byte();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(super) fn plp(&mut self) {
        self.processor_status = self.pop_byte();
        self.set_flag(Flag::Break, false);
        self.set_flag(Flag::Unused, true);
    }

    // Branch
    pub(super) fn bcc(&mut self) {
        self.branch(self.processor_status & Flag::Carry as u8 == 0);
    }

    pub(super) fn bcs(&mut self) {
        self.branch(self.processor_status & Flag::Carry as u8 != 0);
    }

    pub(super) fn beq(&mut self) {
        self.branch(self.processor_status & Flag::Zero as u8 != 0);
    }

    pub(super) fn bne(&mut self) {
        self.branch(self.processor_status & Flag::Zero as u8 == 0);
    }

    pub(super) fn bmi(&mut self) {
        self.branch(self.processor_status & Flag::Negative as u8 != 0);
    }

    pub(super) fn bpl(&mut self) {
        self.branch(self.processor_status & Flag::Negative as u8 == 0);
    }

    pub(super) fn bvc(&mut self) {
        self.branch(self.processor_status & Flag::OverFlow as u8 == 0);
    }

    pub(super) fn bvs(&mut self) {
        self.branch(self.processor_status & Flag::OverFlow as u8 != 0);
    }

    pub(super) fn jmp_absolute(&mut self) {
        self.program_counter = self.fetch_word();
    }

    pub(super) fn jmp_indirect(&mut self) {
        let addr = self.fetch_word();
        // 6502 page boundary bug: if addr is $xxFF,
        // high byte is read from $xx00 instead of $(xx+1)00
        let low = self.peek_byte(addr) as u16;
        let high = if addr & 0x00FF == 0x00FF {
            self.peek_byte(addr & 0xFF00) as u16
        } else {
            self.peek_byte(addr + 1) as u16
        };
        self.program_counter = (high << 8) | low;
    }

    pub(super) fn jsr(&mut self) {
        let target = self.fetch_word();
        self.push_word(self.program_counter - 1);
        self.program_counter = target;
    }

    pub(super) fn rts(&mut self) {
        self.program_counter = self.pop_word() + 1;
    }

    pub(super) fn bit(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.peek_byte(addr);
        self.set_flag(Flag::Zero, self.register_a & value == 0);
        self.set_flag(Flag::Negative, value & SIGN_BIT != 0);
        self.set_flag(Flag::OverFlow, value & (Flag::OverFlow as u8) != 0);
    }

    pub(super) fn rti(&mut self) {
        self.processor_status = self.pop_byte();
        self.set_flag(Flag::Break, false);
        self.set_flag(Flag::Unused, true);
        self.program_counter = self.pop_word();
    }

    pub(super) fn nop(&mut self) {}

    fn branch(&mut self, condition: bool) {
        let offset = self.fetch_byte() as i8;
        if condition {
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }
}
