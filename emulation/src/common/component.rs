use super::message::Msg;

pub trait Component {
    fn handle_event(&mut self, event: ComponentEvent) -> Result<ComponentEventResult, Box<dyn Msg>>;
}

#[derive(Clone, Copy)]
pub enum ComponentEvent {
    RamRead{ addr: u16 },
    RamWrite{ addr: u16, data: u8 },
    VramWrite{ addr: u16, data: u8 }
}

pub enum ComponentEventResult<'a> {
    NotProcessed,
    Empty,
    RamRead{ data: u8 },
    VRamRead{ data: &'a Vec<u8>}
}