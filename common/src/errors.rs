

pub enum EmulErrorKind {
    OutOfBounds,
    DeviceNotFound,
}

pub struct EmulError {
    kind: EmulErrorKind,
    message: String,
}

impl EmulError {
    pub fn new(kind: EmulErrorKind, message: String) -> Self {
        EmulError {
            kind: kind,
            message: message
        }
    }
}
