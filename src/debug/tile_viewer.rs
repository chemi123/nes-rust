use crate::cartridge::Rom;
use crate::ppu::{frame::Frame, palette::SYSTEM_PALETTE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

fn show_tile(frame: &mut Frame, chr_rom: &[u8], bank: usize, tile_n: usize, offset_x: usize, offset_y: usize) {
    let bank = bank * 0x1000;
    let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

    for y in 0..=7 {
        let mut upper = tile[y];
        let mut lower = tile[y + 8];

        for x in (0..=7).rev() {
            let value = (1 & upper) << 1 | (1 & lower);
            upper >>= 1;
            lower >>= 1;
            let rgb = match value {
                0 => SYSTEM_PALETTE[0x01],
                1 => SYSTEM_PALETTE[0x23],
                2 => SYSTEM_PALETTE[0x27],
                3 => SYSTEM_PALETTE[0x30],
                _ => unreachable!(),
            };
            frame.set_pixel(offset_x + x, offset_y + y, rgb);
        }
    }
}

fn show_tile_bank(chr_rom: &[u8], bank: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    for tile_n in 0..256 {
        let col = tile_n % 16;
        let row = tile_n / 16;
        show_tile(&mut frame, chr_rom, bank, tile_n, col * 8, row * 8);
    }
    frame
}

pub fn run(rom_path: &str) {
    let bytes = std::fs::read(rom_path).expect("Failed to read ROM file");
    let rom = Rom::new(&bytes).expect("Failed to parse ROM");

    let frame = show_tile_bank(&rom.chr_rom, 0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile Viewer", 256 * 3, 240 * 3)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    texture.update(None, &frame.pixel, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                _ => {}
            }
        }
    }
}
