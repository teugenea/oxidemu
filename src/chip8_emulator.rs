use std::fs::File;
use std::io::{BufReader, Read};

const START_ADDRESS: usize = 0x200;
const REGISTERS_COUNT: usize = 16;
const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const KEYS_COUNT: usize = 16;
const VIDEO_MEMORY_SIZE: usize = 64 * 32;

const FONTSER_START_ADDRESS: usize = 0x50;
const FONT_SET: Vec<u8> = vec![
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

pub struct Chip8
{
    registers: Vec<u8>,
    memory: Vec<u8>,
    stack: Vec<u16>,
    keypad: Vec<u8>,
    video: Vec<u32>,
    index: u16,
    pc: u16,
    sp: u8,
    opcode: u16,
    delay_timer: u8,
    sound_timer: u8
}

impl Chip8 {

    pub fn new(file_name: &String) -> Chip8 {
        let mut result = Chip8 {
            registers: vec![0u8; REGISTERS_COUNT],
            memory: vec![0u8; MEMORY_SIZE],
            stack: vec![0u16; STACK_SIZE],
            keypad: vec![0u8; KEYS_COUNT],
            video: vec![0u32; VIDEO_MEMORY_SIZE],
            index: 0,
            pc: START_ADDRESS as u16,
            sp: 0,
            opcode: 0,
            delay_timer: 60,
            sound_timer: 60
        };
        Chip8::load_fontset(&mut result);
        Chip8::load_rom(&mut result, file_name);
        return result;
    }

    fn load_fontset(&mut self) {
        let mut i = FONTSER_START_ADDRESS;
        for byte in FONT_SET {
            self.memory[i] = byte;
            i += 1;
        }
    }
    
    fn load_rom(&mut self, file_name: &String) { 
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
        let mut i = START_ADDRESS;
        for byte in bytes {
            self.memory[i] = byte;
            i += 1;
        }
    }
}