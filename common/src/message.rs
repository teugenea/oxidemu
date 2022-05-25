use std::fmt::{ Formatter, Display, Debug };
use std::error::Error;
use strum_macros::IntoStaticStr;

#[derive(Debug, PartialEq, IntoStaticStr)]
pub enum ErrorMsgId {
    OutOfBounds,
    DeviceNotFound,
    RomFileNotFound,
    UnknownInstruction,
    NotInitialized,
}

#[derive(Debug, PartialEq, IntoStaticStr)]
pub enum ErrorTopicId {
    RamRead,
    RamWrite,
    VramRead,
    VramWrite,
    Emulator,
}

pub trait MsgInfo {
    fn kind(&self) -> MsgKind;
    fn topic_id(&self) -> &'static str;
    fn msg_id(&self) -> &'static str;
    fn params(&self) -> &Option<Vec<String>>;
    fn source(&self) -> &Option<Box<dyn Error>>;
}

pub trait Msg: MsgInfo + Display + Debug {

}

#[derive(Debug, PartialEq)]
pub enum MsgKind {
    Error,
    Warning,
    Info
}

#[derive(Debug)]
pub struct ErrorMsg {
    pub msg_id: &'static str,
    pub topic_id: &'static str,
    pub params: Option<Vec<String>>,
    pub source: Option<Box<dyn Error>>,
}

impl ErrorMsg {
    pub fn new(topic_id: &'static str, msg_id: &'static str) -> Self {
        Self {
            msg_id,
            topic_id,
            params: None,
            source: None,
        }
    }

    pub fn add_param(mut self, param: String) -> Self {
        if self.params.is_none() {
            self.params = Some(Vec::new());
        }
        self.params.as_mut().unwrap().push(param);
        self
    }

    pub fn set_source(mut self, source: Box<dyn std::error::Error>) -> Self{
        self.source = Some(source);
        self
    }
}

impl MsgInfo for ErrorMsg {
    fn kind(&self) -> MsgKind {
        MsgKind::Error
    }

    fn topic_id(&self) -> &'static str {
        self.topic_id
    }

    fn msg_id(&self) -> &'static str {
        self.msg_id
    }

    fn params(&self) -> &Option<Vec<String>> {
        &self.params
    }

    fn source(&self) -> &Option<Box<dyn Error>> {
        &self.source
    }
}

impl Display for ErrorMsg {
    fn fmt(&self, err: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(err, "{}: {}", self.topic_id, self.msg_id)
    }
}

impl Error for ErrorMsg {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            _ => None
        }
    }
}

impl Msg for ErrorMsg {}