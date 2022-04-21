use std::io::{BufReader, Read};
use std::fs::File;
use crate::bus::{Readable, Writable};

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

    fn get(&self, address: usize) -> u8 { self.memory[address] }

    fn set(&mut self, address: usize, value: u8) { self.memory[address] = value }

    pub fn load_rom(&mut self, file_name: &str, start_address: usize) {
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

impl Readable for Memory {
    fn read(&self, address: usize) -> u8 {
        self.get(address)
    }
}

impl Writable for Memory {
    fn write(&mut self, address: usize, value: u8) {
        self.set(address, value)
    }
}