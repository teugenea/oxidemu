use std::collections::HashMap;
use super::component::*;

use super::message::{Msg};

pub struct Bus {
    devices: HashMap<String, Box<dyn Component>>
}

impl Default for Bus {
    fn default() -> Self {
        Self { devices: Default::default() }
    }
}

impl Bus {

    pub fn add_component(&mut self, name: String, device: Box<dyn Component>) {
        self.devices.insert(name, device);
    }

    pub fn send_event(&mut self, event: ComponentEvent) -> Result<ComponentEventResult, Box<dyn Msg>> {
        for (_, device) in &mut self.devices {
            let result = device.handle_event(event);
            match result {
                Ok(ComponentEventResult::NotProcessed) => {},
                _ => return result
            }
        }
        return Result::Ok(ComponentEventResult::NotProcessed);
    }

}