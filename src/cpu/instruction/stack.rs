use crate::cpu::Cpu;
use crate::cpu::bus_access::Bus;
use crate::cpu::flags::Flag;

impl<B: Bus> Cpu<B> {
    pub(in crate::cpu) fn pha(&mut self) {
        self.push_byte(self.register_a);
    }

    pub(in crate::cpu) fn php(&mut self) {
        self.push_byte(self.processor_status | Flag::Break as u8 | Flag::AlwaysSet as u8);
    }

    pub(in crate::cpu) fn pla(&mut self) {
        self.register_a = self.pop_byte();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub(in crate::cpu) fn plp(&mut self) {
        self.processor_status = self.pop_byte();
        self.set_flag(Flag::Break, false);
        self.set_flag(Flag::AlwaysSet, true);
    }

    pub(in crate::cpu) fn rti(&mut self) {
        self.processor_status = self.pop_byte();
        self.set_flag(Flag::Break, false);
        self.set_flag(Flag::AlwaysSet, true);
        self.program_counter = self.pop_word();
    }
}
