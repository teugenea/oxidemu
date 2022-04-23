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
    fn readByte(&self, addr: usize) -> u8 {
        self.memory[addr]
    }
}

impl Writable for Memory {
    fn write(&mut self, addr: usize, value: u8) -> Result<(), EmulError> {
        if addr > self.memory.len() {
            let msg = format!("Cannot write to {0} because address is out of bounds. Targer address is {1} but available size is {2}",
                self.get_name(), addr, self.size);
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        Ok(self.memory[addr] = value)
    }

    fn writeBlock(&mut self, start_addr: usize, data: Vec<u8>) -> Result<(), EmulError> {
        let block_end = start_addr + data.len();
        if block_end > self.size {
            let msg = format!("Cannot write BLOCK to {0} because is overflows memory. ",
                self.get_name());
            return Err(EmulError::new(EmulErrorKind::OutOfBounds, msg));
        }
        let mut i = start_addr;
        for b in data {
            self.memory[i] = b;
            i += 1;
        }
        Ok(())
    }
}

impl Rw for Memory {

}

/*
impl Loadable for Memory {
    fn load_rom(&mut self, file_name: &str, start_address: usize) {
        let mut input = BufReader::new(File::open(file_name).expect("Cannot open file file_name"));
        let mut bytes = Vec::new();
        loop {
            use std::io::ErrorKind;
            let mut buffer = [0u8; std::mem::size_of::<u8>()];
            let res = input.read_exact(&mut buffer);
            match res {
                Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
                _ => {}
            }
            res.expect("error during read");
            bytes.push(u8::from_le_bytes(buffer));
        }
        let mut i = start_address;
        for byte in bytes {
            self.memory[i] = byte;
            i += 1;
        }
    }
}
*/