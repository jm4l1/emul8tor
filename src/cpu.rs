extern crate rand;

pub const SCREEN_WIDTH: u32 = 0x0040;
pub const SCREEN_HEIGHT: u32 = 0x0020;
const START_ADDR: u16 = 0x0200;
const MEMORY_SIZE: usize = 0x1000;
const NUM_DATA_REGISTERS: usize = 0x10;
const STACK_LENGTH: usize = 0x10;
const FONT_SET_SIZE: usize = 80;
const NUM_KEYS: usize = 16;
const FONT_SET: [u8; FONT_SET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
pub struct CPU {
    memory: [u8; MEMORY_SIZE],
    data_registers: [u8; NUM_DATA_REGISTERS],
    address_register: u16,
    program_counter: u16,
    stack: [u16; STACK_LENGTH],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    frame_buffer: [bool; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
    pub inputs: [bool; NUM_KEYS],
}

impl CPU {
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[..FONT_SET_SIZE].copy_from_slice(&FONT_SET);
        Self {
            memory,
            data_registers: [0; NUM_DATA_REGISTERS],
            address_register: 0x0000,
            program_counter: START_ADDR,
            stack: [0; STACK_LENGTH],
            stack_pointer: 0x00,
            delay_timer: 0x00,
            sound_timer: 0x00,
            frame_buffer: [false; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            inputs: [false; NUM_KEYS],
        }
    }
    pub fn get_display(&self) -> &[bool] {
        &self.frame_buffer
    }
    pub fn reset(&mut self) {
        self.memory[..FONT_SET_SIZE].copy_from_slice(&FONT_SET);
        self.data_registers = [0; NUM_DATA_REGISTERS];
        self.address_register = 0x0000;
        self.program_counter = START_ADDR;
        self.stack = [0; STACK_LENGTH];
        self.stack_pointer = 0x00;
        self.delay_timer = 0x00;
        self.sound_timer = 0x00;
        self.frame_buffer = [false; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
        self.inputs = [false; NUM_KEYS];
    }
    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.inputs[idx] = pressed;
    }
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = start + data.len();
        self.memory[start..end].copy_from_slice(data);
    }
    fn push(&mut self, address: u16) {
        self.stack[self.stack_pointer as usize] = address;
        self.stack_pointer += 1;
    }
    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }
    fn fetch(&mut self) -> u16 {
        let high_byte = self.read_memory(self.program_counter.into()) as u16;
        self.program_counter += 1;
        let low_byte = self.read_memory(self.program_counter.into()) as u16;
        self.program_counter += 1;
        (high_byte << 8) | low_byte
    }
    fn read_memory(&self, address: usize) -> u8 {
        self.memory[address]
    }
    fn execute(&mut self, opcode: u16) {
        let d1 = ((opcode & 0xF000) >> 12) as u8;
        let d2 = ((opcode & 0x0F00) >> 8) as u8;
        let d3 = ((opcode & 0x00F0) >> 4) as u8;
        let d4 = (opcode & 0x000F) as u8;
        match (d1, d2, d3, d4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.frame_buffer = [false; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            (0, 0, 0xE, 0xE) => {
                self.program_counter = self.pop();
            }
            (1, _, _, _) => {
                self.program_counter = opcode & 0xFFF;
            }
            (2, _, _, _) => {
                let nnn = opcode & 0xFFF;
                self.push(self.program_counter);
                self.program_counter = nnn;
            }
            (3, x, k1, k2) => {
                let kk = (k1 << 4) | k2;
                if self.data_registers[x as usize] == kk {
                    self.program_counter += 2;
                }
            }
            (4, x, k1, k2) => {
                let kk = (k1 << 4) | k2;
                if self.data_registers[x as usize] != kk {
                    self.program_counter += 2;
                }
            }
            (5, x, y, 0) => {
                if self.data_registers[x as usize] == self.data_registers[y as usize] {
                    self.program_counter += 2;
                }
            }
            (6, x, k1, k2) => {
                let kk = (k1 << 4) | k2;
                self.data_registers[x as usize] = kk;
            }
            (7, x, k1, k2) => {
                let kk = (k1 << 4) | k2;
                self.data_registers[x as usize] = self.data_registers[x as usize].wrapping_add(kk);
            }
            (8, x, y, 0) => self.data_registers[x as usize] = self.data_registers[y as usize],
            (8, x, y, 1) => self.data_registers[x as usize] |= self.data_registers[y as usize],
            (8, x, y, 2) => self.data_registers[x as usize] &= self.data_registers[y as usize],
            (8, x, y, 3) => self.data_registers[x as usize] ^= self.data_registers[y as usize],
            (8, x, y, 4) => {
                let (sum, carry) = self.data_registers[x as usize]
                    .overflowing_add(self.data_registers[y as usize]);
                self.data_registers[x as usize] = sum;
                self.data_registers[0xf] = if carry { 1 } else { 0 };
            }
            (8, x, y, 5) => {
                let (diff, borrow) = self.data_registers[x as usize]
                    .overflowing_sub(self.data_registers[y as usize]);
                self.data_registers[x as usize] = diff;
                self.data_registers[0xf] = if !borrow { 1 } else { 0 };
            }
            (8, x, _, 6) => {
                let x = x as usize;
                let lsb = self.data_registers[x] & 1;
                self.data_registers[x] >>= 1;
                self.data_registers[0xf] = lsb;
            }
            (8, x, y, 7) => {
                let x = x as usize;
                let y = y as usize;
                let (diff, borrow) = self.data_registers[y].overflowing_sub(self.data_registers[x]);
                self.data_registers[x as usize] = diff;
                self.data_registers[0xf] = if !borrow { 1 } else { 0 };
            }
            (8, x, _, 0xE) => {
                let x = x as usize;
                let msb = (self.data_registers[x] >> 7) & 1;
                self.data_registers[x] <<= 1;
                self.data_registers[0xf] = msb;
            }
            (9, x, y, 0) => {
                let x = x as usize;
                let y = y as usize;
                if self.data_registers[x] != self.data_registers[y] {
                    self.program_counter += 2;
                }
            }
            (0xA, _, _, _) => {
                self.address_register = opcode & 0xFFF;
            }
            (0xB, _, _, _) => {
                self.program_counter = self.data_registers[0] as u16 + (opcode & 0xFFF);
            }
            (0xC, x, k1, k2) => {
                let x = x as usize;
                let kk = (k1 << 4) | k2;
                let byte: u8 = rand::random();
                self.data_registers[x] = kk & byte;
            }
            (0xD, x, y, n) => {
                let x_cord = self.data_registers[x as usize] as u16;
                let y_cord = self.data_registers[y as usize] as u16;
                let mut flipped = false;
                for y_line in 0..n {
                    let addr = self.address_register + y_line as u16;
                    let pixels = self.memory[addr as usize];
                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_cord + x_line) as usize % SCREEN_WIDTH as usize;
                            let y = (y_cord + y_line as u16) as usize % (SCREEN_HEIGHT as usize);

                            let idx = x + (SCREEN_WIDTH as usize) * y;
                            flipped |= self.frame_buffer[idx];
                            self.frame_buffer[idx] ^= true;
                        }
                    }
                }
                self.data_registers[0xf] = if flipped { 1 } else { 0 };
            }
            (0xE, x, 9, 0xE) => {
                let x = x as usize;
                let vx = self.data_registers[x];
                if self.inputs[vx as usize] {
                    self.program_counter += 2;
                }
            }
            (0xE, x, 0xA, 1) => {
                let x = x as usize;
                let vx = self.data_registers[x];
                if !self.inputs[vx as usize] {
                    self.program_counter += 2;
                }
            }
            (0xF, x, 0, 7) => {
                let x = x as usize;
                self.data_registers[x] = self.delay_timer;
            }
            (0xF, x, 0, 0xA) => {
                let x = x as usize;
                let mut pressed = false;
                for i in 0..NUM_KEYS {
                    if self.inputs[i] {
                        self.data_registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.program_counter -= 2;
                }
            }
            (0xF, x, 1, 5) => {
                let x = x as usize;
                self.delay_timer = self.data_registers[x];
            }
            (0xF, x, 1, 8) => {
                let x = x as usize;
                self.sound_timer = self.data_registers[x];
            }
            (0xF, x, 1, 0xE) => {
                let x = x as usize;
                self.address_register = self
                    .address_register
                    .wrapping_add(self.data_registers[x].into());
            }
            (0xF, x, 2, 9) => {
                let x = x as usize;
                let c = self.data_registers[x] as u16;
                self.address_register = c * 5;
            }
            (0xF, x, 3, 3) => {
                let x = x as usize;
                let vx = self.data_registers[x] as f32;

                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx - (hundreds as f32 * 100.0)) / 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                let i = self.address_register as usize;
                self.memory[i] = hundreds;
                self.memory[i + 1] = tens;
                self.memory[i + 2] = ones;
            }
            (0xF, x, 5, 5) => {
                let x = x as usize;
                let i = self.address_register as usize;
                for idx in 0..=x {
                    self.memory[i + idx] = self.data_registers[idx];
                }
            }
            (0xF, x, 6, 5) => {
                let x = x as usize;
                let i = self.address_register as usize;
                for idx in 0..=x {
                    self.data_registers[idx] = self.memory[i + idx];
                }
            }
            (_, _, _, _) => unimplemented!("opcode {:04x}", opcode),
        }
    }
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {}
            self.sound_timer -= 1;
        }
    }
}
