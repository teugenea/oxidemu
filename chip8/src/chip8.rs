extern crate common;

use common::bus::{Writable, Readable};
use common::bus::DeviceType;
use common::memory::Memory;
use common::cpu::Cpu;
use common::video::VideoMemory;
use rand::Rng;
use rand::thread_rng;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const START_ADDRESS: usize = 0x200;
const STACK_LEVELS: usize = 16;

const FONTSET_START_ADDRESS: usize = 0x50;
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


struct Chip8 {
    memory: Memory,
    video_memory: VideoMemory,
    registers: Vec<u8>,
    stack: Vec<u16>,
    pc: u16,
    sp: u8,
    index: u16,
    opcode: u16,
    delay_timer: u8,
    sound_timer: u8
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Chip8::init_memory(),
            video_memory: VideoMemory::new(64, 32),
            registers: vec![0u8; REGISTERS_COUNT],
            stack: vec![0u16; STACK_LEVELS],
            pc: START_ADDRESS as u16,
            sp: 0,
            index: 0,
            opcode: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }

    fn init_memory() -> Memory {
        let mut memory = Memory::new(MEMORY_SIZE);
        let mut i = FONTSET_START_ADDRESS;
        for byte in FONT_SET {
            let res = memory.write_byte(i, byte);
            i += 1;
            match res {
                Ok(_) => {}
                Err(err) => { panic!("Cannot load fontset: {}", err) }
            }
        }
        memory
    }

    fn decode(opcode: &u16) -> u16 {
        let code = opcode & 0xF000;
        match code >> 12 {
            0x0|0x8|0xE => opcode & 0xF00F,
            0xF => opcode & 0xF0FF,
            _ => opcode & 0xF000
        }
    }

    fn get_rand() -> u8 { thread_rng().gen_range(0..256) as u8}

    fn exec_intruction(&mut self) {
        match Chip8::decode(&self.opcode) {
            0x0000 => self.op_00e0(),
            0x000E => self.op_00ee(),
            0x1000 => self.op_1nnn(),
            0x2000 => self.op_2nnn(),
            0x3000 => self.op_3xkk(),
            0x4000 => self.op_4xkk(),
            0x5000 => self.op_5xy0(),
            0x6000 => self.op_6xkk(),
            0x7000 => self.op_7xkk(),
            y => {
                panic!("Cannot decode instruction {}", y);
            }
        }
    }

    //CLS
    fn op_00e0(&mut self) {
        self.video_memory.clear();
    }

    //RET
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    //JP
    fn op_1nnn(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

    //CALL addr
    fn op_2nnn(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
    }

    //SE Vx, byte - skip if equals
    fn op_3xkk(&mut self)  {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        if self.registers[vx as usize] == byte as u8 {
            self.pc += 2;
        }
    }

    //SNE Vx, byte - skip if not equals
    fn op_4xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        if self.registers[vx as usize] != byte as u8 {
            self.pc += 2;
        }
    }

    //SE vx, vy - skip if registers equals
    fn op_5xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        if self.registers[vx as usize] ==  self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    //LD Vx, byte - set register
    fn op_6xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        self.registers[vx as usize] = byte as u8;
    }

    //ADD vx, byte
    fn op_7xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        self.registers[vx as usize] = byte as u8;
    }
}

impl Cpu for Chip8 {
    
    fn cycle(&mut self) {
        let opcode = self.memory.read_word(self.pc as usize).expect("Cannot read from memory");
        self.pc += 2;
        self.opcode = opcode;
        self.exec_intruction();
        if self.delay_timer > 0 { self.delay_timer -= 1 }
        if self.sound_timer > 0 { self.sound_timer -= 1 }
    }

}

#[cfg(test)]
mod Chip8Tests {
    
    use super::*;

    #[test]
    fn test_op_5xy0() {
        let mut c8 = Chip8::new();
        let start_addr = START_ADDRESS as u16;
        c8.registers[1] = 2;
        c8.registers[2] = 5;
        c8.opcode = 0x5120;
        c8.exec_intruction();
        assert_eq!(c8.pc, start_addr);

        c8.registers[1] = 5;
        c8.exec_intruction();
        assert_eq!(c8.pc, start_addr + 2);
    }
}