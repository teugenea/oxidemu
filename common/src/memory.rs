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

    fn write_block(&mut self, start_addr: usize, data: Vec<u8>) -> Result<(), EmulError> {
        let block_end = start_addr + data.len();
        if block_end > self.size {
            let msg = format!("Cannot write BLOCK to {0} because it overflows memory. ",
                self.get_name());
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
}

impl Rw for Memory {

}