use js_sys::Uint8Array;
use libchip8cpu::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct WasmEmu {
    chip8: CPU,
    ctx: CanvasRenderingContext2d,
}
#[wasm_bindgen]
impl WasmEmu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEmu {
        let chip8 = CPU::new();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("display_stage").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        WasmEmu { chip8, ctx }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.chip8.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) -> u8 {
        let mut sound_timer_done = false;
        self.chip8.tick_timers(&mut sound_timer_done);
        if sound_timer_done {
            1
        } else {
            0
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, key_evt: KeyboardEvent, pressed: bool) {
        let key = key_evt.key();
        if let Some(key_index) = key_to_input(&key) {
            self.chip8.keypress(key_index, pressed)
        }
    }
    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        self.chip8.load(&data.to_vec());
    }
    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        let display = self.chip8.get_display();
        for i in 0..((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) {
            if display[i] {
                let x = i % SCREEN_WIDTH as usize;
                let y = i / SCREEN_WIDTH as usize;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }
}

fn key_to_input(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
