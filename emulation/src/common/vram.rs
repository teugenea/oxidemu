
pub struct Vram {
    memory: Vec<u32>,
    width: usize,
    height: usize,
    size: usize,
}

impl Vram {
    pub fn new(width: usize, height: usize) -> Self {
        Vram {
            memory: vec![0u32; width * height],
            width: width,
            height: height,
            size: width * height,
        }
    }

    pub fn width(&self) -> usize { self.width }

    pub fn height(&self) -> usize { self.height }

    pub fn size(&self) -> usize { self.size }

    pub fn clear(&mut self) {
        self.memory.fill(0)
    }

    pub fn read_pixel(&self, addr: usize) -> u32 {
        self.memory[addr]
    }

    pub fn write_pixel(&mut self, addr: usize, pixel: u32) {
        self.memory[addr] = pixel;
    }

    pub fn video_32(&self) -> &Vec<u32> { &self.memory }

    pub fn video_8(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(4 * self.memory.len());
        for value in &self.memory {
            out.extend(value.to_be_bytes());
        }
        out
    }
}
