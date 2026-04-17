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
        // 画面端を跨ぐスプライトの off-screen 部分はクリップする
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return;
        }
        let base = (y * Self::WIDTH + x) * 3;
        self.pixel[base..base + 3].copy_from_slice(&[rgb.0, rgb.1, rgb.2]);
    }
}
