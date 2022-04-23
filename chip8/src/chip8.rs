extern crate common;

use common::bus::Bus;
use common::bus::DeviceType;
use common::memory::Memory;
use common::cpu::Cpu;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const START_ADDRESS: usize = 0x200;


struct Chip8 {
    bus: Bus,
    registers: Vec<u8>,
    pc: u16,
    sp: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            bus: Chip8::init_bus(),
            registers: vec![0u8; REGISTERS_COUNT],
            pc: START_ADDRESS as u16,
            sp: 0,
        }
    }

    pub fn load_rom(&mut self, file_name: &'static str) {
        
    }

    fn init_bus() -> Bus {
        let mut bus = Bus::new();
        let mem = Memory::new(MEMORY_SIZE);
        bus.add_dev(DeviceType::Memory, Box::new(mem));
        bus
    }

    fn decode(opcode: &u16) -> u16 {
        let code = opcode & 0xF000;
        match code >> 12 {
            0x0|0x8|0xE => opcode & 0xF00F,
            0xF => opcode & 0xF0FF,
            y => opcode & 0xF000
        }
    }

    fn execute_instruction(&mut self, opcode: &u16) {
        match Chip8::decode(opcode) {
            0x0000 => self.op_00e0(),
            0x000E => self.op_00ee(),
            0x1000 => self.op_1nnn(),
            y => {}
        }
    }

    //CLS
    fn op_00e0(&mut self) {
        
    }

    //RET
    fn op_00ee(&mut self) {

    }

    //JP
    fn op_1nnn(&mut self) {

    }

}

impl Cpu for Chip8 {
    
    fn cycle(&mut self) {
        todo!() 
    }

    fn exec_intruction(&mut self, opcode: &u16) {
        self.execute_instruction(opcode);
    }
}