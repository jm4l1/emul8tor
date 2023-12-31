extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

const SCALE: u32 = 10;
pub struct Peripheral {
    pump: EventPump,
    canvas: Canvas<Window>,
    screen_width: u32,
    speaker: Option<AudioDevice<SquareWave>>,
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

        let speaker = match context.audio() {
            Ok(audio_subsystem) => {
                let desired_spec = AudioSpecDesired {
                    freq: Some(44100),
                    channels: Some(1), // mono
                    samples: None,     // default sample size
                };
                match audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    // initialize the audio callback
                    SquareWave {
                        phase_inc: 440.0 / spec.freq as f32,
                        phase: 0.0,
                        volume: 0.25,
                    }
                }) {
                    Ok(speaker) => Some(speaker),
                    Err(err) => {
                        eprintln!("unable to initialize sound {}", err);
                        None
                    }
                }
            }
            Err(err) => {
                eprintln!("unable to initialize sound {}", err);
                None
            }
        };

        Self {
            canvas,
            pump,
            screen_width,
            speaker,
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
    pub fn beep(&self) {
        if self.speaker.is_some() {
            match &self.speaker {
                Some(speaker) => {
                    speaker.resume();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    speaker.pause();
                }
                None => {}
            };
        }
    }
}
