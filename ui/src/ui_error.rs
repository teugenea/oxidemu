use std::fmt::{ Formatter, Display };
use std::error::Error;
use common::errors::*;

#[derive(Debug)]
pub struct UiError {
    pub emul_error: Option<EmulError>,
    pub source: Option<Box<dyn Error>>,
}

impl Display for UiError {
    fn fmt(&self, err: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(err, "")
    }
}

impl Error for UiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            _ => None
        }
    }
}