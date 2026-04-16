use crate::ppu::Ppu;
use crate::ppu::frame::Frame;
use crate::ppu::render::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

const NES_WIDTH: u32 = 256;
const NES_HEIGHT: u32 = 240;
const SCALE: u32 = 3;

pub struct Screen {
    canvas: Canvas<Window>,
    texture: Texture<'static>,
    // TextureCreatorを保持してtextureのライフタイムを保証する
    _texture_creator: &'static TextureCreator<WindowContext>,
    event_pump: EventPump,
    frame: Frame,
}

impl Screen {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("NES Emulator", NES_WIDTH * SCALE, NES_HEIGHT * SCALE)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = Box::leak(Box::new(canvas.texture_creator()));
        let texture = texture_creator
            .create_texture_target(PixelFormatEnum::RGB24, NES_WIDTH, NES_HEIGHT)
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Screen {
            canvas,
            texture,
            _texture_creator: texture_creator,
            event_pump,
            frame: Frame::new(),
        }
    }

    pub fn update(&mut self, ppu: &Ppu) {
        render(ppu, &mut self.frame);
        self.texture
            .update(None, &self.frame.pixel, NES_WIDTH as usize * 3)
            .unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn poll_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }
    }
}
