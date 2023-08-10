extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

const SCALE: u32 = 10;
pub struct Peripheral {
    pump: EventPump,
    canvas: Canvas<Window>,
    screen_width: u32,
}

fn key_to_input(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

impl Peripheral {
    pub fn new(title: &'static str, screen_width: u32, screen_height: u32) -> Self {
        let context = match sdl2::init() {
            Ok(context) => context,
            Err(_) => panic!("Unable to create screen"),
        };
        let window = match context.video() {
            Ok(video_subsystem) => match video_subsystem
                .window(title, screen_width * SCALE, screen_height * SCALE)
                .position_centered()
                .opengl()
                .build()
            {
                Ok(window) => window,
                Err(_) => panic!("Unable to create screen"),
            },
            Err(_) => panic!("Unable to create screen"),
        };
        let canvas = match window.into_canvas().present_vsync().build() {
            Ok(canvas) => canvas,
            Err(_) => panic!("Unable to create screen"),
        };
        let pump = match context.event_pump() {
            Ok(pump) => pump,
            Err(_) => panic!("Unable to create screen"),
        };
        Self {
            canvas,
            pump,
            screen_width,
        }
    }
    pub fn handle_event(
        &mut self,
        should_break: &mut bool,
        key_pressed_down: &mut usize,
        key_pressed_up: &mut usize,
    ) {
        for evt in self.pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => *should_break = true,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key_to_input(key) {
                    Some(key) => *key_pressed_down = key,
                    None => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key_to_input(key) {
                    Some(key) => *key_pressed_up = key,
                    None => {}
                },
                _ => (),
            }
        }
    }
    pub fn draw_screen(&mut self, screen_buffer: &[bool]) {
        self.set_draw_color(Color::RGB(0, 0, 0));
        self.clear();
        self.set_draw_color(Color::RGB(255, 255, 255));
        for (i, pixel) in screen_buffer.iter().enumerate() {
            if *pixel {
                let x: u32 = i as u32 % self.screen_width;
                let y = i as u32 / self.screen_width;
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.present();
    }
    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    pub fn draw_rect(&mut self, rect: Rect) {
        self.canvas.draw_rect(rect).unwrap();
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
}
