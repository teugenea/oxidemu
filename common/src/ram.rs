use crate::errors::{EmulError, EmulErrorKind};

pub struct Ram {
    memory: Vec<u8>,
    size: usize,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Ram {
            size: size,
            memory: vec![0u8; size],
        }
    }

    pub fn read_byte(&self, addr: usize) -> Result<u8, EmulError> {
        if self.size < addr {
            let msg = format!("Cannot read from RAM");
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        Ok(self.memory[addr])
    }

    pub fn read_word(&self, addr: usize) -> Result<u16, EmulError> {
        if self.size < addr || self.size < addr + 1 {
            let msg = format!("Cannot read from RAM");
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        let first_byte = self.memory[addr] as u16;
        let second_byte = self.memory[addr + 1] as u16;
        let mut word: u16 = first_byte << 8;
        word = word | second_byte;
        Ok(word)
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), EmulError> {
        if addr > self.memory.len() {
            let msg = format!("Cannot write to RAM because address is out of bounds. Targer address is {0} but available size is {1}",
                addr, self.size);
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        Ok(self.memory[addr] = value)
    }

    pub fn write_block(&mut self, start_addr: usize, data: Vec<u8>) -> Result<(), EmulError> {
        let block_end = start_addr + data.len();
        if block_end > self.size {
            let msg = format!("Cannot write BLOCK to RAM because it overflows memory");
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        let mut i = start_addr;
        for byte in data {
            let res = self.write_byte(i, byte);
            i += 1;
            match res {
                Err(err) => return Err(err),
                _ => {}
            }
        }
        Ok(())
    }
    
    pub fn write_word(&mut self, _: usize, _: u16) -> std::result::Result<(), EmulError> {
        
        Ok(())
    }
}
