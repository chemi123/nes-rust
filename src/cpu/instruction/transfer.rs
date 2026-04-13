use crate::cpu::Cpu;
use crate::cpu::bus_access::Bus;

impl<B: Bus> Cpu<B> {
    pub(in crate::cpu) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(in crate::cpu) fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub(in crate::cpu) fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub(in crate::cpu) fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }
}
