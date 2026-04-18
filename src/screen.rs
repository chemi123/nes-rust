use crate::NesError;
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

fn sdl_err<E: std::fmt::Display>(error: E) -> NesError {
    NesError::Sdl(error.to_string())
}

impl Screen {
    pub fn new() -> Result<Self, NesError> {
        let sdl_context = sdl2::init().map_err(sdl_err)?;
        let video_subsystem = sdl_context.video().map_err(sdl_err)?;
        let window = video_subsystem
            .window("NES Emulator", NES_WIDTH * SCALE, NES_HEIGHT * SCALE)
            .position_centered()
            .build()
            .map_err(sdl_err)?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(sdl_err)?;
        let texture_creator = Box::leak(Box::new(canvas.texture_creator()));
        let texture = texture_creator
            .create_texture_target(PixelFormatEnum::RGB24, NES_WIDTH, NES_HEIGHT)
            .map_err(sdl_err)?;

        let event_pump = sdl_context.event_pump().map_err(sdl_err)?;

        let mut key_map = HashMap::new();
        key_map.insert(Keycode::W, JoypadButton::UP);
        key_map.insert(Keycode::S, JoypadButton::DOWN);
        key_map.insert(Keycode::A, JoypadButton::LEFT);
        key_map.insert(Keycode::D, JoypadButton::RIGHT);
        key_map.insert(Keycode::J, JoypadButton::BUTTON_A);
        key_map.insert(Keycode::K, JoypadButton::BUTTON_B);
        key_map.insert(Keycode::Space, JoypadButton::SELECT);
        key_map.insert(Keycode::Return, JoypadButton::START);

        Ok(Screen {
            canvas,
            texture,
            _texture_creator: texture_creator,
            event_pump,
            frame: Frame::new(),
            key_map,
        })
    }

    pub fn update(&mut self, ppu: &Ppu) -> Result<(), NesError> {
        render(ppu, &mut self.frame);
        self.texture
            .update(None, &self.frame.pixel, NES_WIDTH as usize * 3)
            .map_err(sdl_err)?;
        self.canvas.copy(&self.texture, None, None).map_err(sdl_err)?;
        self.canvas.present();
        Ok(())
    }

    // 戻り値: ユーザが終了要求 (Quit / Escape) を出した場合 false
    pub fn poll_events(&mut self, joypad: &mut Joypad) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
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
        true
    }
}
