use std::fmt::{ Formatter, Display, Error };

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    OutOfBounds{ addr: usize, max: usize, size: usize },
    DeviceNotFound,
    RomFileNotFound,
    UnknownInstruction,
    NotInitialized,
}

#[derive(Debug, PartialEq)]
pub enum ErrorTopic {
    RamRead,
    RamWrite,
    VramRead,
    VramWrite,
    Emulator,
}

impl Display for ErrorTopic {
    fn fmt(&self, err: &mut Formatter<'_>) -> Result<(), Error> {
        let msg = match self {
            ErrorTopic::RamWrite => "RAM write",
            ErrorTopic::RamRead => "RAM read",
            ErrorTopic::VramRead => "VRAM read",
            ErrorTopic::VramWrite => "VRAM write",
            ErrorTopic::Emulator => "Emulator",
            _ => "Unknown topic",
        };
        write!(err, "{}", msg)
    }
}

#[derive(Debug)]
pub struct EmulError {
    pub kind: ErrorKind,
    pub topic: ErrorTopic,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl EmulError {
    pub fn new(kind: ErrorKind, topic: ErrorTopic) -> Self {
        EmulError {
            kind,
            topic,
            source: None,
        }
    }

    pub fn new_source(kind: ErrorKind, topic: ErrorTopic, source: Box<dyn std::error::Error>) -> Self {
        Self {
            kind,
            topic,
            source: Some(source),
        }
    }
}

impl std::error::Error for EmulError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            _ => None
        }
    }
}

impl Display for EmulError {
    fn fmt(&self, err: &mut Formatter<'_>) -> Result<(), Error> {
        let msg = match self.kind {
            ErrorKind::OutOfBounds{ addr, max, size } => 
                format!("OutOfBounds: address: {0}, max address: {1}, data size: {2}", addr, max, size),
            _ => String::from("")
        };
        write!(err, "({}:, {})", self.topic, msg)
    }
}