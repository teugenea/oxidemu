use crate::message::*;

pub struct CycleResult {
    pub video_buff_changed: bool,
    pub total_cycle_count: u128,
    pub last_cycle_count: u128,
}

impl Default for CycleResult {
    fn default() -> Self {
        Self {
            video_buff_changed: false,
            total_cycle_count: 0,
            last_cycle_count: 0,
        }
    }
}

pub trait Emulator {
    fn video_buffer(&self) -> Vec<u8>;
    fn cycle(&mut self) -> Result<CycleResult, Box<dyn Msg>>;
    fn process_input(&mut self, emul_key: u32, pressed: bool);
    fn load_rom(&mut self, file_name: &String);
    fn resolution(&self) -> [u32; 2];
    fn cycles_in_sec(&self) -> u64;
}

pub struct EmulMgr {
    emulator: Option<Box<dyn Emulator>>,
    version: u32,
    pause: bool,
}

impl Default for EmulMgr {
    fn default() -> Self {
        Self {
            emulator: None,
            version: 0,
            pause: false,
        }
    }
}

impl EmulMgr {
    pub fn set_emulator(&mut self, emulator: Box<dyn Emulator>) {
        self.emulator.replace(emulator);
        self.version += 1;
    }

    pub fn cycle(&mut self) -> Result<CycleResult, Box<dyn Msg>> {
        if let Some(emul) = self.emulator.as_mut() {
            if !self.pause {
                return emul.cycle();
            }
        }
        let err = ErrorMsg::new(
            ErrorTopicId::Emulator.into(),
            ErrorMsgId::NotInitialized.into(),
        );
        Err(Box::new(err))
    }

    pub fn set_pause(&mut self, pause: bool) {
        if self.emulator.is_some() {
            self.pause = pause;
        }
    }

    pub fn is_paused(&self) -> bool {
        self.pause
    }

    pub fn video_buffer(&self) -> Result<Vec<u8>, Box<dyn Msg>> {
        if let Some(emul) = &self.emulator {
            return Ok(emul.video_buffer());
        }
        Err(self.not_init_error())
    }

    pub fn process_input(&mut self, emul_key: u32, pressed: bool) {
        if let Some(emul) = self.emulator.as_mut() {
            emul.process_input(emul_key, pressed);
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn resolution(&self) -> Result<[u32; 2], Box<dyn Msg>> {
        if let Some(emul) = &self.emulator {
            return Ok(emul.resolution());
        }
        Err(self.not_init_error())
    }

    pub fn cycles_in_sec(&self) -> Result<u64, Box<dyn Msg>> {
        if let Some(emul) = &self.emulator {
            return Ok(emul.cycles_in_sec());
        }
        Err(self.not_init_error())
    }

    fn not_init_error(&self) -> Box<dyn Msg> {
        let err = ErrorMsg::new(
            ErrorTopicId::Emulator.into(),
            ErrorMsgId::NotInitialized.into(),
        );
        Box::new(err)
    }
}
