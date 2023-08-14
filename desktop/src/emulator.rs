use crate::peripherals::*;
use libchip8cpu::*;

const TICK_PER_FRAME: usize = 30;
pub struct Emulator {
    cpu: CPU,
    peripherals: Peripheral,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            peripherals: Peripheral::new("Emulator", SCREEN_WIDTH, SCREEN_HEIGHT),
        }
    }
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.cpu.load(rom);
    }
    pub fn start(&mut self) {
        let mut should_break = false;
        let mut key_pressed_down: usize = 0;
        let mut key_pressed_up: usize = 0;
        let mut sound_timer_done = false;
        loop {
            self.peripherals.handle_event(
                &mut should_break,
                &mut key_pressed_down,
                &mut key_pressed_up,
            );
            if key_pressed_down != 0 {
                self.cpu.keypress(key_pressed_down, true);
                key_pressed_down = 0;
            }
            if key_pressed_up != 0 {
                self.cpu.keypress(key_pressed_up, false);
                key_pressed_up = 0;
            }
            if should_break {
                break;
            }
            for _ in 0..TICK_PER_FRAME {
                self.cpu.tick();
            }
            self.peripherals.draw_screen(&self.cpu.get_display());

            self.cpu.tick_timers(&mut sound_timer_done);
            if sound_timer_done {
                self.peripherals.beep();
                sound_timer_done = false;
            }
        }
    }
}
