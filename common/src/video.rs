use crate::bus::{ Readable, Writable, Rw };

pub struct VideoMemory {
    memory: Vec<u32>,
    pub wight: usize,
    pub height: usize,
}

pub trait VideoOut {
    fn set(&mut self, addr: usize, value: u32);
    fn get(&self) -> &Vec<u32>;
}

impl VideoMemory {
    pub fn new(width: usize, height: usize) -> Self {
        VideoMemory {
            memory: vec![0u32; width * height],
            wight: width,
            height: height,
        }
    }

    pub fn clear(&mut self) {
        self.memory.fill(0)
    }
}

impl VideoOut for VideoMemory {
    fn get(&self) -> &Vec<u32> { &self.memory }
    fn set(&mut self, addr: usize, value: u32) { self.memory[addr] = value; }
}
