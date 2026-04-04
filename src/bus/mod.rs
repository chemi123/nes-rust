pub(crate) struct Bus {
    cpu_vram: [u8; 2048],
}

impl Bus {
    pub(crate) fn new() -> Self {
        Bus {
            cpu_vram: [0; 2048],
        }
    }
}
