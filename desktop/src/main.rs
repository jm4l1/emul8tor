use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_path = args.get(1).expect("Unable to find file");
    let mut emulator: desktop_emul8tor::emulator::Emulator =
        desktop_emul8tor::emulator::Emulator::new();
    let rom = OpenOptions::new()
        .read(true)
        .open(Path::new(rom_path))
        .expect("Unable to open file");
    let mut f = BufReader::new(rom);
    let mut rom_buffer = Vec::<u8>::new();
    match f.read_to_end(&mut rom_buffer) {
        Ok(_) => {}
        Err(err) => eprintln!("{err}"),
    }
    emulator.load_rom(&rom_buffer);
    emulator.start();
}
