extern crate common;

use common::bus::Bus;
use common::bus::DeviceType;

struct Chip8 {
    bus: Bus
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            bus: Bus::new()
        }
    }

    pub fn cycle(&mut self) {
        
    }
}