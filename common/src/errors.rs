

#[derive(Debug, PartialEq)]
pub enum EmulErrorKind {
    OutOfBounds,
    DeviceNotFound,
}

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