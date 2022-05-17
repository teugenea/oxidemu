use common::emulator::Emul;
use std::sync::{Arc, Mutex};
use std::thread;
use chip8::chip8::Chip8;

fn main() {
    let mut chip = Chip8::new();
    chip.load_rom(&String::from("D:/Projects/rusty-emul/chip8-roms/games/Airplane.ch8"));
    let emul: Emul = Arc::new(Mutex::new(Box::new(chip)));
    let r = Arc::clone(&emul);
    thread::spawn(move || {
        loop {
             let mut em = r.lock().unwrap();
             let r = em.cycle();
             match r {
                 
                 //Err(e) => println!("{:?}", e),
                 _ => {}
             }
        }
    });
    ui::show(emul);
}