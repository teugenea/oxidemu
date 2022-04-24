use crate::errors::{EmulError, EmulErrorKind};

use std::collections::HashMap;
use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Hash, Clone, Copy, Eq, Debug)]
#[repr(u8)]
pub enum DeviceType {
    Memory,
}

impl DeviceType {
    fn to_string(&self) -> &'static str {
        match self {
            DeviceType::Memory => "Memory"
        }
    }
}

impl PartialEq for DeviceType {
    fn eq(&self, other: &DeviceType) -> bool {
        *self as u8 == *other as u8
    }
}

impl Display for DeviceType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.to_string())
    }
}

pub trait Debuggable {
    fn get_name(&self) -> &'static str;
}

pub trait Readable : Debuggable {
    fn read_byte(&self, addr: usize) -> Result<u8, EmulError>;
    fn read_word(&self, addr: usize) -> Result<u16, EmulError>;
}

pub trait Writable : Debuggable {
    fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), EmulError>;
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

    pub fn read(&self, d_type: &DeviceType, addr: usize) -> Result<u8, EmulError> {
        let b = self.devs.get(d_type).take();
        match b {
            Some(b) => {
                match b.read_byte(addr) {
                    Ok(byte) => Ok(byte),
                    Err(err) => Err(err)
                }
            },
            _ => {
                let msg = format!("Cannot read ");
                Err(EmulError::new(EmulErrorKind::DeviceNotFound, msg))
            }
        }
    }

    pub fn write(&mut self, d_type: DeviceType, addr: usize, value: u8) -> Result<(), EmulError> {
        let device = self.devs.get_mut(&d_type);
        match device {
            Some(b) => b.write_byte(addr, value),
            _ => {
                let msg = format!("");
                Err(EmulError::new(EmulErrorKind::DeviceNotFound, msg))
            }
        }
    }

}