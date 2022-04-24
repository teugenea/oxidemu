pub mod memory;
pub mod video;
pub mod bus;
pub mod cpu;
pub mod utils;
pub mod errors;

pub trait Emulator : cpu::Cpu + video::VideoOut { }

#[cfg(test)]
mod tests;