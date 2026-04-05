use crate::cpu::Cpu;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::bus_access::Bus;

impl<B: Bus> Cpu<B> {
    pub(in crate::cpu) fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a = self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_x = self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(in crate::cpu) fn ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_y = self.peek_byte(addr);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub(in crate::cpu) fn sta(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write_byte(addr, self.register_a);
    }

    pub(in crate::cpu) fn stx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write_byte(addr, self.register_x);
    }

    pub(in crate::cpu) fn sty(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.write_byte(addr, self.register_y);
    }
}
