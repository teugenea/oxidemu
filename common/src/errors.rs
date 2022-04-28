use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum EmulErrorKind {
    OutOfBounds,
    DeviceNotFound,
    RomFileNotFound,
}

#[derive(Debug)]
pub struct EmulError {
    pub kind: EmulErrorKind,
    pub message: String,
}

impl EmulError {
    pub fn new(kind: EmulErrorKind, message: String) -> Self {
        EmulError {
            kind: kind,
            message: message
        }
    }
}

impl EmulErrorKind {
    fn to_string(&self) -> &'static str {
        match self {
            EmulErrorKind::DeviceNotFound => "DeviceNotFound",
            EmulErrorKind::OutOfBounds => "OutOfBounds",
            EmulErrorKind::RomFileNotFound => "RomFileNotFound",
        }
    }
}

impl Display for EmulErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.to_string())
    }
}

impl Display for EmulError {
    
    fn fmt(&self, err: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(err, "({}, {})", self.kind, self.message)
    }
}