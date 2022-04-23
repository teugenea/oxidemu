use crate::errors::{EmulError, EmulErrorKind};

use std::collections::HashMap;
use std::cmp::PartialEq;

#[derive(Hash, Clone, Eq)]
pub enum DeviceType {
    Memory,
    VideoMemory,
}

impl PartialEq for DeviceType {
    fn eq(&self, other: &DeviceType) -> bool { self == other }
}

pub trait Debuggable {
    fn get_name(&self) -> &'static str;
}

pub trait Readable : Debuggable {
    fn readByte(&self, addr: usize) -> u8;
}

pub trait Writable : Debuggable {
    fn write(&mut self, addr: usize, value: u8) -> Result<(), EmulError>;
    fn writeBlock(&mut self, start_addr: usize, data: Vec<u8>) -> Result<(), EmulError>;
}

pub trait Rw : Readable + Writable { }

pub struct Bus {
    devs: HashMap<DeviceType, Box<dyn Rw>>
}

impl Bus {

    pub fn new() -> Self {
        Bus {
            devs: HashMap::new()
        }
    }

    pub fn add_dev(&mut self, d_type: DeviceType, dev: Box<dyn Rw>) {
        self.devs.insert(d_type, dev);
    }

    pub fn read(&self, d_type: &DeviceType, addr: usize) -> Result<u8, &'static str> {
        let b = self.devs.get(d_type).take();
        match b {
            Some(b) => Ok(b.readByte(addr)),
            _ => Err("No such device")
        }
    }

    pub fn write(&mut self, d_type: &DeviceType, addr: usize, value: u8) {
        let mut b = self.devs.get_mut(d_type).take();
        match b {
            Some(b) => { b.write(addr, value); },
            _ => {}
        }
    }

}