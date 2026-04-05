use crate::cpu::bus_access::Bus;
use crate::memory::Memory;

pub(crate) struct NESBus {
    memory: Memory,
}

impl NESBus {
    pub(crate) fn new() -> Self {
        NESBus {
            memory: Memory::new(),
        }
    }
}

impl Bus for NESBus {
    fn read(&self, addr: u16) -> u8 {
        self.memory.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory.write(addr, value);
    }
}
