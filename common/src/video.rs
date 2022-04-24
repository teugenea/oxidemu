use crate::bus::{ Readable, Writable, Rw };

pub struct VideoMemory {
    memory: Vec<u32>,
    pub wight: usize,
    pub height: usize,
}

pub trait VideoOut {
    fn get_video_buf_32(&self) -> &Vec<u32>;
    fn get_video_buf_8(&self) -> Vec<u8>;
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
    fn get_video_buf_32(&self) -> &Vec<u32> { &self.memory }

    fn get_video_buf_8(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(4 * self.memory.len());
        for value in &self.memory {
            out.extend(&value.to_be_bytes());
        }

        out
    }
}
