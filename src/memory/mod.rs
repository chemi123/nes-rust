const RAM_SIZE: usize = 0x0800; // 2KiB

pub(crate) struct Memory {
    data: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; RAM_SIZE] }
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
