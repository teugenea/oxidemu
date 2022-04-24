use crate::bus::{Readable, Writable, Rw, Debuggable};
use crate::errors::{EmulError, EmulErrorKind};

pub struct Memory {
    memory: Vec<u8>,
    size: usize
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            size: size,
            memory: vec![0u8; size]
        }
    }
}

impl Debuggable for Memory {
    fn get_name(&self) -> &'static str { "RAM" }
}

impl Readable for Memory {
    fn read_byte(&self, addr: usize) -> Result<u8, EmulError> {
        if self.size < addr {
            let msg = format!("Cannot read from {0}", self.get_name());
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg))
        }
        Ok(self.memory[addr])
    }

    fn read_word(&self, addr: usize) -> Result<u16, EmulError> {
        if self.size < addr || self.size < addr + 1 {
            let msg = format!("Cannot read from {0}", self.get_name());
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg))
        }
        let first_byte = self.memory[addr] as u16;
        let second_byte = self.memory[addr + 1] as u16;
        let mut word: u16 = first_byte << 8;
        word = word | second_byte;
        Ok(word)
    }
}

impl Writable for Memory {
    fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), EmulError> {
        if addr > self.memory.len() {
            let msg = format!("Cannot write to {0} because address is out of bounds. Targer address is {1} but available size is {2}",
                self.get_name(), addr, self.size);
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        Ok(self.memory[addr] = value)
    }
}

impl Rw for Memory {

}