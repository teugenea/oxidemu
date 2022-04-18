mod chip8_emulator;

use chip8_emulator::Chip8;

fn main() {
    let ch = Chip8::new(&String::from("D:/Projects/chip8/chip8-roms/games/Airplane.ch8"));
    print!("hi!");
}
