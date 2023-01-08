use std::collections::HashMap;
use crate::common::device::*;

use super::message::ErrorMsg;

pub struct Bus {
    devices: HashMap<String, Box<dyn Device>>
}

impl Default for Bus {
    fn default() -> Self {
        Self { devices: Default::default() }
    }
}

impl Bus {

    pub fn plug_device(&mut self, name: String, device: Box<dyn Device>) {
        self.devices.insert(name, device);
    }

    pub fn unplug_device(&mut self, name: &String) {
        self.devices.remove(name);
    }

    pub fn send_event(&mut self, event: DeviceEvent) -> Result<DeviceEventResult, ErrorMsg> {
        for (_, device) in &mut self.devices {
            return device.handle_event(event);
        }
        return Result::Ok(DeviceEventResult::NotProcessed);
    }

}