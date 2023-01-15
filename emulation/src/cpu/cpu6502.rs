use std::{rc::Rc, cell::RefCell};

use super::cpu::Cpu;
use crate::common::{bus::Bus, component::{ComponentEvent, ComponentEventResult}, message::Msg};

const BP: u16 = 0x0100;

enum Flag6502 {
    C = 1,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    V = 1 << 6,
    N = 1 << 7
}

struct Instruction6502 {
    name: String,
    addr_mode: fn(&mut Cpu6502) -> u8,
    operator: fn(&mut Cpu6502) -> u8,
    cycles_takes: u8
}

pub struct Cpu6502 {
    bus: Rc<RefCell<Bus>>,

    pc: u16,
    sp: u16,
    a: u8,
    x: u8,
    y: u8,
    status: u8,

    opcode: u8,
    cycles_left: u8,
    fetched: u8,

    instruction_set: Vec<Instruction6502>,
}

impl Cpu for Cpu6502 {
    fn cycle(&mut self) {
        todo!()
    }
}

impl Cpu6502 {
    
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            bus: bus,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            opcode: 0,
            cycles_left: 0,
            fetched: 0,
            instruction_set: Cpu6502::init_instruction_set(),
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.pc = 0;
        self.sp = 0xFD;
        self.status = 0;

        self.cycles_left = 0;
        self.opcode = 0;

        self.cycles_left = 8;
    }

    fn get_flag(&self, flag: Flag6502) -> bool {
        return if (self.status & flag as u8) > 0 { true } else { false };
    }

    fn set_flag(&mut self, flag: Flag6502, v: bool) {
        if v {
            self.status = self.status | flag as u8;
        } else {
            self.status = self.status & !(flag as u8);
        }
    }

    fn write(&self, addr: u16, data: u8) -> Result<(), Box<dyn Msg>> {
        let mut bus = self.bus.borrow_mut();
        let res = bus.send_event(ComponentEvent::RamWrite { addr, data });
        match res {
            Ok(event_result) => {
                match event_result {
                    ComponentEventResult::Empty => Ok(()),
                    _ => Ok(())
                }
            },
            Err(msg) => Err(msg)
        }
    }

    fn read(&self, addr: u16) -> Result<u8, Box<dyn Msg>> {
        let mut bus = self.bus.borrow_mut();
        let res = bus.send_event(ComponentEvent::RamRead { addr });
        match res {
            Ok(event_result) => {
                match event_result {
                    ComponentEventResult::RamRead { data } => Ok(data),
                    _ => Ok(0)
                }
            }
            Err(msg) => Err(msg)
        }
    }

    fn irq(&mut self) -> Result<(), Box<dyn Msg>> {
        if self.get_flag(Flag6502::I) {
            return Ok(());
        }
        self.write(BP + self.sp, (self.pc >> 8) as u8 & 0x00FF)?;
        self.sp -= 1;
        self.write(BP + self.sp, self.pc as u8 & 0x00FF)?;
        self.sp -= 1;
        self.set_flag(Flag6502::B, false);
        self.set_flag(Flag6502::U, true);
        self.set_flag(Flag6502::I, true);
        self.write(BP + self.sp, self.status)?;

        Ok(())
    }

    fn nmi(&mut self) {
        
    }

    fn imp_mode(&mut self) -> u8 {
        self.fetched = self.a;
        return 0;
    }

    fn imm_mode(&mut self) -> u8 {
        return 0;
    }

    fn init_instruction_set() -> Vec<Instruction6502> {
        vec![
        ]
    }
    
}