use crate::message::*;

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

    pub fn read_byte(&self, addr: usize) -> Result<u8, Box<dyn Msg>> {
        if self.size < addr {
            //let err = ErrorKind::OutOfBounds{addr, max: self.memory.len(), size: 1};
            let err = ErrorMsg::new(ErrorTopicId::RamRead, ErrorMsgId::OutOfBounds)
                .add_param(addr.to_string())
                .add_param(self.memory.len().to_string());
            return Err(Box::new(err));
        }
        Ok(self.memory[addr])
    }

    pub fn read_word(&self, addr: usize) -> Result<u16, Box<dyn Msg>> {
        if self.size < addr || self.size < addr + 1 {
            //let err = ErrorKind::OutOfBounds{addr, max: self.memory.len(), size: 2};
            let err = ErrorMsg::new(ErrorTopicId::RamRead, ErrorMsgId::OutOfBounds)
                .add_param(addr.to_string())
                .add_param(self.memory.len().to_string());
            return Err(Box::new(err));
        }
        let first_byte = self.memory[addr] as u16;
        let second_byte = self.memory[addr + 1] as u16;
        let mut word: u16 = first_byte << 8;
        word = word | second_byte;
        Ok(word)
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), Box<dyn Msg>> {
        if addr > self.memory.len() {
            // let err = ErrorKind::OutOfBounds{addr, max: self.memory.len(), size: 2};
            let err = ErrorMsg::new(ErrorTopicId::RamWrite, ErrorMsgId::OutOfBounds)
                .add_param(addr.to_string())
                .add_param(self.memory.len().to_string());
            return Err(Box::new(err));
        }
        Ok(self.memory[addr] = value)
    }

    pub fn write_block(&mut self, start_addr: usize, data: Vec<u8>) -> Result<(), Box<dyn Msg>> {
        let block_end = start_addr + data.len();
        if block_end > self.size {
            let err = ErrorMsg::new(ErrorTopicId::RamRead, ErrorMsgId::OutOfBounds)
                .add_param(start_addr.to_string())
                .add_param(self.memory.len().to_string())
                .add_param(data.len().to_string());
            return Err(Box::new(err));
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
    
    pub fn write_word(&mut self, _: usize, _: u16) -> std::result::Result<(), Box<dyn Msg>> {
        
        Ok(())
    }
}
