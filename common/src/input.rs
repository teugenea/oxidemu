
pub enum InputDevice {
    Keyboard(u32),
    Gamepad(u32),
}

pub struct InputKey {
    pub device: InputDevice,
    pub key_code: u32,
    pub pressed: bool,
}

impl InputKey {
    pub fn new(device: InputDevice, key_code: u32, pressed: bool) -> Self {
        Self {
            device: device,
            key_code: key_code,
            pressed: pressed,
        }
    }
}

pub trait InputProcessor {
    fn process_input(&mut self, key: InputKey);
}