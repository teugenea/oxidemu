use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum DeviceEvent {
    RAM_READ{ addr: u16 },
    RAM_WRITE{ addr: u16, data: u8 },
}

pub enum DeviceEventResult {
    RAM_READ{ data: u8 }
}

pub trait Device {
    fn handle_event(&mut self, event: DeviceEvent) -> Option<DeviceEventResult>;
}

pub struct Bus {
    devices: HashMap<String, Box<dyn Device>>
}

impl Default for Bus {
    fn default() -> Self {
        Self { devices: Default::default() }
    }
}

impl Bus {

    pub fn put_device(&mut self, name: String, device: Box<dyn Device>) {
        self.devices.insert(name, device);
    }

    pub fn send_event(&mut self, event: DeviceEvent) -> Option<DeviceEventResult> {
        for (_, device) in &mut self.devices {
            match device.handle_event(event) {
                Some(result) => return Option::Some(result),
                _ => {}
            }
        }
        return Option::None;
    }

}