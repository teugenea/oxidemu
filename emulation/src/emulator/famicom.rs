use std::{rc::Rc, cell::RefCell};

use crate::{cpu::cpu6502::Cpu6502, common::{bus::Bus, ram::Ram}};

pub struct Famicom {
    cpu: Cpu6502,
    bus: Rc<RefCell<Bus>>
}

impl Default for Famicom {
    fn default() -> Self {
        let memory = Box::new(Ram::new(1024 * 64));
        let bus = Rc::new(RefCell::new(Bus::default()));
        bus.borrow_mut().add_component("RAM".to_string(), memory);
        Self {
            cpu: Cpu6502::new(bus.clone()),
            bus
        }
    }
}