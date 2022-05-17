use std::sync::Mutex;
use std::sync::Arc;
use crate::input::InputKey;
use crate::errors::EmulError;

pub struct CycleResult {
    pub video_buff_changed: bool,
    pub cycle_count: u32,
}

impl Default for CycleResult {
    fn default() -> Self {
        Self {
            video_buff_changed: false,
            cycle_count: 0,
        }
    }
}

pub trait Emulator {
    fn video_buffer(&self) -> Vec<u8>;
    fn cycle(&mut self) -> Result<CycleResult, EmulError>;
    fn process_input(&mut self, key: InputKey);
    fn load_rom(&mut self, file_name: &String);
    fn resolution(&self) -> [usize; 2];
}

pub type Emul = Arc<Mutex<Box<dyn Emulator + Send>>>;