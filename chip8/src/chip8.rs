extern crate common;

use common::ram::Ram;
use common::vram::Vram;
use common::cpu::*;
use common::utils;
use common::input::*;
use common::emulator::Emulator;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use rand::thread_rng;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const START_ADDRESS: usize = 0x200;
const STACK_LEVELS: usize = 16;
const COLOR: u32 = 0x00FF00FF;
const KEY_COUNT: usize = 16;

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

pub struct Chip8 {
    memory: Ram,
    video_memory: Vram,
    registers: Vec<u8>,
    stack: Vec<u16>,
    pc: u16,
    sp: u8,
    index: u16,
    opcode: u16,
    delay_timer: u8,
    sound_timer: u8,
    keypad: Vec<u8>,
    last_cycle_time: u128,
    cycle_delay: u128,
    active: bool,
    cnt: u32,
}

impl Chip8 {
    pub fn new() -> Self {

        Chip8 {
            memory: Chip8::init_memory(),
            video_memory: Vram::new(64, 32),
            registers: vec![0u8; REGISTERS_COUNT],
            stack: vec![0u16; STACK_LEVELS],
            pc: START_ADDRESS as u16,
            sp: 0,
            index: 0,
            opcode: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: vec![0u8; KEY_COUNT],
            last_cycle_time: Chip8::get_time(),
            cycle_delay: 0,
            active: false,
            cnt: 0,
        }
    }

    fn init_memory() -> Ram {
        let mut memory = Ram::new(MEMORY_SIZE);
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
            _ => code
        }
    }

    fn get_rand() -> u8 { thread_rng().gen_range(0..256) as u8}

    fn do_cycle(&mut self) -> CycleResult{
        let opcode = self.memory.read_word(self.pc as usize).expect("Cannot read from memory");
        self.pc += 2;
        self.opcode = opcode;
        let res = self.exec_intruction();
        if self.delay_timer > 0 { self.delay_timer -= 1 }
        if self.sound_timer > 0 { self.sound_timer -= 1 }
        res
    }

    pub fn load_rom(&mut self, file_name: String) {
        match utils::load_rom(&file_name) {
            Err(error) => print!("Cannot load rom {}", error),
            Ok(result) => {
                let load_res = self.memory.write_block(START_ADDRESS, result);
                match load_res {
                    Err(error) => print!("Cannot write to memory. {}", error),
                    _ => self.active = true
                }
            }
        }
    }

    fn exec_intruction(&mut self) -> CycleResult {
        self.cnt += 1;
        let mut res = CycleResult::default();
        res.cycle_count = self.cnt;
        match Chip8::decode(&self.opcode) {
            0x0000 => { self.op_00e0(); res },
            0x000E => { self.op_00ee(); res },
            0x1000 => { self.op_1nnn(); res },
            0x2000 => { self.op_2nnn(); res },
            0x3000 => { self.op_3xkk(); res },
            0x4000 => { self.op_4xkk(); res },
            0x5000 => { self.op_5xy0(); res },
            0x6000 => { self.op_6xkk(); res },
            0x7000 => { self.op_7xkk(); res },
            0x8000 => { self.op_8xy0(); res },
            0x8001 => { self.op_8xy1(); res },
            0x8002 => { self.op_8xy2(); res },
            0x8003 => { self.op_8xy3(); res },
            0x8004 => { self.op_8xy4(); res },
            0x8005 => { self.op_8xy5(); res },
            0x8006 => { self.op_8xy6(); res },
            0x8007 => { self.op_8xy7(); res },
            0x800E => { self.op_8xyE(); res },
            0x9000 => { self.op_9xy0(); res },
            0xA000 => { self.op_Annn(); res },
            0xB000 => { self.op_Bnnn(); res },
            0xC000 => { self.op_Cxkk(); res },
            0xD000 => { self.op_Dxyn(); res.video_buff_changed=true; res },
            0xE00E => { self.op_Ex9E(); res },
            0xE001 => { self.op_ExA1(); res },
            0xF007 => { self.op_Fx07(); res },
            0xF015 => { self.op_Fx15(); res },
            0xF018 => { self.op_Fx18(); res },
            0xF029 => { self.op_Fx29(); res },
            0xF033 => { self.op_Fx33(); res },
            0xF055 => { self.op_Fx55(); res },
            0xF065 => { self.op_Fx65(); res },
            0xF00A => { self.op_Fx0A(); res },
            0xF01E => { self.op_Fx1E(); res },
            y => {
                panic!("Cannot decode instruction {}", y);
            }
        }
    }

    fn get_time() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Cannot get time").as_millis()
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
        let mut res: u16 = self.registers[vx as usize] as u16 + byte as u16;
        if res > 255 {
            res = res - 255;
        }
        self.registers[vx as usize] = res as u8;
    }

    //LD Vx, Vy
    fn op_8xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.registers[vx as usize] = self.registers[vy as usize];
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.registers[vx as usize] |= self.registers[vy as usize];
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.registers[vx as usize] &= self.registers[vy as usize];
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.registers[vx as usize] ^= self.registers[vy as usize];
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let sum = self.registers[vx] as u16 + self.registers[vy] as u16;
        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[vx] = (sum & 0xFF) as u8;
    }

    //SUB Vx, Vy
    fn op_8xy5(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.registers[vx] >= self.registers[vy] {
            self.registers[0xF] = 1;
            self.registers[vx] -= self.registers[vy];            
        } else {
            self.registers[0xF] = 0;
            let res = 0xFF - (self.registers[vy] - self.registers[vx] - 1);
            self.registers[vx] = res;
        }
    }

    //SHR Vx
    fn op_8xy6(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.registers[0xF] = self.registers[vx] & 0x1;
        self.registers[vx] >>= 1;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.registers[vx] < self.registers[vy] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[vx] = self.registers[vy] - self.registers[vx];
    }

    // SHL Vx {, Vy}
    fn op_8xyE(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.registers[0xF] = (self.registers[vx] & 0x80) >> 7;
        self.registers[vx] <<= 1;
    }

    //SNE Vx, Vy - skip if not eq
    fn op_9xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }

    //LD I, addr. Set I = nnn
    fn op_Annn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.index = address;
    }

    //Bnnn - JP V0, addr. Jump nnn + V0
    fn op_Bnnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.pc = self.registers[0] as u16 + address;
    }

    //RND Vx, byte. Set Vx = random byte AND kk.
    fn op_Cxkk(&mut self) {
        let vx = ((self.opcode & 0xF00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;
        self.registers[vx] = Chip8::get_rand() & byte;
    }

    //Dxyn - DRW Vx, Vy, nibble
    fn op_Dxyn(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let height = self.opcode & 0x000F;

        let x_pos = self.registers[vx] % self.video_memory.width() as u8;
        let y_pos = self.registers[vy] % self.video_memory.height() as u8;
        self.registers[0xF] = 0;
        for row in 0..height {
            let sprite_byte = self.memory.read_byte((self.index + row) as usize)
                .expect("Cannot read");
            for col in 0..8 {
                let sprite_pixel = sprite_byte & (0x80 >> col);
                let addr = (y_pos as usize + row as usize) * self.video_memory.width()
                    + (x_pos as usize + col as usize);
                let screen_pixel = self.video_memory.read_pixel(addr);
                if sprite_pixel != 0 {
                    if screen_pixel == COLOR {
                        self.registers[0xF] = 1;
                    }
                    self.video_memory.write_pixel(addr, screen_pixel ^ COLOR);
                }
            }
        }
    }

    //Ex9E - SKP Vx. Skip next instruction if key with the value of Vx is pressed
    fn op_Ex9E(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let key = self.registers[vx];
        if self.keypad[key as usize] != 0 {
            self.pc += 2;
        }
    }

    //ExA1 - SKNP Vx. Skip next instruction if key with the value of Vx is not pressed
    fn op_ExA1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let key = self.registers[vx];
        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }

    //Fx07 - LD Vx, DT. Set Vx = delay timer value
    fn op_Fx07(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.registers[vx] = self.delay_timer;
    }

    //Fx0A - LD Vx, K. Wait for a key press, store the value of the key in Vx
    fn op_Fx0A(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        for key_number in 0..KEY_COUNT {
            if self.keypad[key_number] != 0 {
                self.registers[vx] = key_number as u8;
                return;
            }
        }
        self.pc -= 2;
    }

    //Fx15 - LD DT, Vx. Set delay timer = Vx
    fn op_Fx15(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.delay_timer = self.registers[vx];
    }

    //Fx18 - LD ST, Vx. Set sound timer = Vx
    fn op_Fx18(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.sound_timer = self.registers[vx];
    }

    //Fx1E - ADD I, Vx. Set I = I + Vx
    fn op_Fx1E(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.index += self.registers[vx] as u16;
    }

    //Fx29 - LD F, Vx. Set I = location of sprite for digit Vx
    fn op_Fx29(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let digit = self.registers[vx];
        self.index = FONTSET_START_ADDRESS as u16 + 5 * digit as u16;
    }

    //Fx33 - LD B, Vx. Store BCD representation of Vx in memory locations I, I+1, and I+2
    fn op_Fx33(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let mut value = self.registers[vx];
        let addr = self.index as usize;
        self.memory.write_byte(addr + 2, value % 10).expect("!");
        value /= 10;
        self.memory.write_byte(addr + 1, value % 10).expect("!");
        value /= 10;
        self.memory.write_byte(addr, value % 10).expect("!");
    }

    //Fx55 - LD [I], Vx. Store registers V0 through Vx in memory starting at location I
    fn op_Fx55(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        for i in 0..vx+1 {
            let addr = self.index as usize + i;
            self.memory.write_byte(addr, self.registers[i]).expect("!");
        }
    }

    //Fx65 - LD Vx, [I]. Read registers V0 through Vx from memory starting at location I
    fn op_Fx65(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        for i in 0..vx+1 {
            self.registers[i] = self.memory.read_byte(self.index as usize + i).expect("!");
        }
    }
}

impl Cpu for Chip8 {
    
    fn cycle(&mut self) -> CycleResult {
        let time = Chip8::get_time();
        if self.active && self.last_cycle_time + self.cycle_delay <= time {
            self.last_cycle_time = time;
            return self.do_cycle();
        }
        CycleResult::default()
    }

}

impl Emulator for Chip8 {
    
    fn video_buffer(&self) -> Vec<u8> {
        self.video_memory.video_8()
    }

    fn cycle(&mut self) {
        self.do_cycle();
    }

    fn process_input(&mut self, key: InputKey) { todo!() }

    fn load_rom(&mut self, file_name: &String) { todo!() }

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

    #[test]
    fn test_op_8xy4() {
        let mut c8 = Chip8::new();
        
    }
}