use crate::cpu::Cpu;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::bus_access::Bus;

impl<B: Bus> Cpu<B> {
    pub(in crate::cpu) fn inc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.peek_byte(addr).wrapping_add(1);
        self.write_byte(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    pub(in crate::cpu) fn dec(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.peek_byte(addr).wrapping_sub(1);
        self.write_byte(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    pub(in crate::cpu) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(in crate::cpu) fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub(in crate::cpu) fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(in crate::cpu) fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
}
