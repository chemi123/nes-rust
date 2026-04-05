use crate::cpu::Cpu;
use crate::cpu::bus_access::Bus;

impl<B: Bus> Cpu<B> {
    pub(in crate::cpu) fn nop(&mut self) {}
}
