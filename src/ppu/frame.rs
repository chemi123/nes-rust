pub(crate) struct Frame {
    pub(crate) pixel: Vec<u8>,
}

impl Frame {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    pub(crate) fn new() -> Self {
        Frame {
            pixel: vec![0; Self::WIDTH * Self::HEIGHT * 3],
        }
    }

    pub(crate) fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let (r, g, b) = rgb;
        let base = (y * Self::WIDTH + x) * 3;
        if base + 2 < self.pixel.len() {
            self.pixel[base..base + 3].copy_from_slice(&[r, g, b]);
        }
    }
}
