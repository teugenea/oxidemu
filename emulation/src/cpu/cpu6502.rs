
pub struct Cpu6502 {
    pc: u16,
    sp: u16,
    a: u8,
    x: u8,
    y: u8,
    flag: u8,
}