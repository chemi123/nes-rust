use crate::joypad::{Joypad, JoypadButton};
use crate::ppu::Ppu;
use crate::ppu::frame::Frame;
use crate::ppu::render::render;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

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
    key_map: HashMap<Keycode, JoypadButton>,
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

        let mut key_map = HashMap::new();
        key_map.insert(Keycode::W, JoypadButton::UP);
        key_map.insert(Keycode::S, JoypadButton::DOWN);
        key_map.insert(Keycode::A, JoypadButton::LEFT);
        key_map.insert(Keycode::D, JoypadButton::RIGHT);
        key_map.insert(Keycode::J, JoypadButton::BUTTON_A);
        key_map.insert(Keycode::K, JoypadButton::BUTTON_B);
        key_map.insert(Keycode::Space, JoypadButton::SELECT);
        key_map.insert(Keycode::Return, JoypadButton::START);

        Screen {
            canvas,
            texture,
            _texture_creator: texture_creator,
            event_pump,
            frame: Frame::new(),
            key_map,
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

    pub fn poll_events(&mut self, joypad: &mut Joypad) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(button) = self.key_map.get(&key) {
                        joypad.set_button_pressed_status(*button, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(button) = self.key_map.get(&key) {
                        joypad.set_button_pressed_status(*button, false);
                    }
                }
                _ => {}
            }
        }
    }
}
