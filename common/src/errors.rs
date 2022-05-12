use std::fmt::{ Formatter, Display, Error };

#[derive(Debug, PartialEq)]
pub enum EmulErrorKind {
    OutOfBounds{ addr: usize, max: usize, size: usize },
    DeviceNotFound,
    RomFileNotFound,
    UnknownInstruction,
}

#[derive(Debug)]
pub struct EmulError {
    pub kind: EmulErrorKind,
    pub topic: String,
}

impl EmulError {
    pub fn new(kind: EmulErrorKind, topic: String) -> Self {
        EmulError {
            kind,
            topic
        }
    }
}

impl Display for EmulError {
    fn fmt(&self, err: &mut Formatter<'_>) -> Result<(), Error> {
        let msg = match self.kind {
            EmulErrorKind::OutOfBounds{ addr, max, size } => 
                format!("OutOfBounds: address: {0}, max address: {1}, data size: {2}", addr, max, size),
            _ => String::from("")
        };
        write!(err, "({}, {})", self.topic, msg)
    }
}