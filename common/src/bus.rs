use std::collections::HashMap;
use std::cmp::PartialEq;

#[derive(Hash, Clone, Eq)]
pub enum DeviceType {
    Memory = 1,
    VideoMemory = 2,
}

impl PartialEq for DeviceType {
    fn eq(&self, other: &DeviceType) -> bool { self == other }
}

pub trait Readable {
    fn read(&self, address: usize) -> u8;
}

pub trait Writable {
    fn write(&mut self, address: usize, value: u8);
}

pub trait Rw : Readable + Writable {

}

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

    pub fn read(&self, d_type: &DeviceType, address: usize) -> Result<u8, &'static str> {
        let b = self.devs.get(d_type).take();
        match b {
            Some(b) => Ok(b.read(address)),
            _ => Err("No such device")
        }
    }

    pub fn write(&mut self, d_type: &DeviceType, address: usize, value: u8) {

    }

}