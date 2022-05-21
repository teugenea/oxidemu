use crate::errors::{ EmulError, ErrorTopic, ErrorKind };
use crate::input::InputKey;
use std::ops::Deref;
use std::sync::{ Arc, Condvar, Mutex };
use std::thread;
use std::thread::JoinHandle;

pub struct CycleResult {
    pub video_buff_changed: bool,
    pub cycle_count: u32,
}

impl Default for CycleResult {
    fn default() -> Self {
        Self {
            video_buff_changed: false,
            cycle_count: 0,
        }
    }
}

pub trait Emulator {
    fn video_buffer(&self) -> Vec<u8>;
    fn cycle(&mut self) -> Result<CycleResult, EmulError>;
    fn process_input(&mut self, key: InputKey);
    fn load_rom(&mut self, file_name: &String);
    fn resolution(&self) -> [u32; 2];
}

struct EmulSync {
    emulator: Box<dyn Emulator + Send>,
    stop: bool,
    pause: bool,
}

pub struct EmulMgr {
    emulator: Option<Arc<(Mutex<EmulSync>, Condvar)>>,
    thread_handle: Option<JoinHandle<()>>,
    version: u32,
    resolution: Option<[u32; 2]>,
}

impl Default for EmulMgr {
    fn default() -> Self {
        Self {
            emulator: None,
            thread_handle: None,
            version: 0,
            resolution: None,
        }
    }
}

impl EmulMgr {
    pub fn set_emulator(&mut self, emulator: Box<dyn Emulator + Send>) {
        if self.emulator.is_some() {
            let emul_opt = self.emulator.take();
            if let Some(emul_sync) = emul_opt {
                let (e, _) = emul_sync.deref();
                e.lock().unwrap().stop = true;
                let jh = self.thread_handle.take();
                let _result = jh.ok_or("").unwrap().join();
            }
        }

        let emul_sync = EmulSync {
            emulator,
            stop: false,
            pause: false,
        };
        self.resolution = Some(emul_sync.emulator.resolution());
        let emul_rc = Arc::new((Mutex::new(emul_sync), Condvar::new()));
        self.thread_handle = Some(self.spawn_thread(&emul_rc));
        self.emulator = Some(emul_rc);
        self.version += 1;
    }

    fn spawn_thread(&mut self, emulator: &Arc<(Mutex<EmulSync>, Condvar)>) -> JoinHandle<()> {
        let emul_rc = Arc::clone(emulator);
        thread::spawn(move || {
            loop {
                let (e, c) = emul_rc.deref();
                let mut emul_sync = e.lock().unwrap();
                if emul_sync.stop {
                    break;
                }
                emul_sync = c.wait_while(emul_sync, |es| es.pause).unwrap();
                let _res = emul_sync.emulator.cycle();
            }
        })
    }

    pub fn set_pause(&self, pause: bool) {
        if let Some(emul_sync) = &self.emulator.as_deref() {
            let (e, _) = emul_sync;
            e.lock().unwrap().pause = pause;
        }
    }

    pub fn is_paused(&self) -> bool {
        if let Some(emul_sync) = &self.emulator.as_deref() {
            let (e, _) = emul_sync;
            return e.lock().unwrap().pause;
        }
        false
    }

    pub fn video_buffer(&self) -> Result<Vec<u8>, EmulError> {
        if let Some(emul_rc) = &self.emulator {
            let (e, _) = emul_rc.deref();
            return Ok(e.lock().unwrap().emulator.video_buffer());
        }
        Err(EmulError::new(ErrorKind::NotInitialized, ErrorTopic::Emulator))
    }

    pub fn version(&self) -> u32 { self.version }

    pub fn resolution(&self) -> Result<[u32; 2], EmulError> {
        if let Some(resolution) = self.resolution {
            return Ok(resolution);
        }
        Err(EmulError::new(ErrorKind::NotInitialized, ErrorTopic::Emulator))
    }

}
