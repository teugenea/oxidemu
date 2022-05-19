use common::emulator::EmulMgr;
use chip8::chip8::Chip8;

fn main() {
    let mut chip = Chip8::new();
    chip.load_rom(&String::from("D:/Projects/rusty-emul/chip8-roms/games/Airplane.ch8"));
    
    let mut emul = EmulMgr::default();
    emul.set_emulator(Box::new(chip));

    ui::show(emul);
}