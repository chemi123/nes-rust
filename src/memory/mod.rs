pub const CARTRIDGE_ROM_START: u16 = 0x8000;
pub const RESET_VECTOR: u16 = 0xFFFC;
pub const STACK_BASE: u16 = 0x0100;
pub const STACK_POINTER_INIT: u8 = 0xFD;

pub(crate) struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 0x10000] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn load(&mut self, addr: u16, data: &[u8]) {
        self.data[addr as usize..(addr as usize + data.len())].copy_from_slice(data);
    }
}
