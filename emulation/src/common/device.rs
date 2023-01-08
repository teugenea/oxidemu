use super::message::ErrorMsg;

pub trait Device {
    fn handle_event(&mut self, event: DeviceEvent) -> Result<DeviceEventResult, ErrorMsg>;
}

#[derive(Clone, Copy)]
pub enum DeviceEvent {
    RamRead{ addr: u16 },
    RamWrite{ addr: u16, data: u8 },
}

pub enum DeviceEventResult {
    NotProcessed,
    Empty,
    RamRead{ data: u8 }
}