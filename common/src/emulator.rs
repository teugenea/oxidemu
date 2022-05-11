use crate::input::InputKey;

pub trait Emulator {
    fn video_buffer(&self) -> Vec<u8>;
    fn cycle(&mut self);
    fn process_input(&mut self, key: InputKey);
    fn load_rom(&mut self, file_name: &String);
}