
pub trait Cpu {
    fn cycle(&mut self);
    fn exec_intruction(&mut self, opcode: &u16);
}