use crate::errors::EmulError;
use crate::input::InputKey;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
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
    fn resolution(&self) -> [usize; 2];
}

pub type Emul = Arc<Mutex<Box<dyn Emulator + Send>>>;

struct EmulSync {
    emulator: Box<dyn Emulator + Send>,
    stop: bool,
    pause: bool,
}

pub struct EmulMgr {
    emulator: Option<Arc<(Mutex<EmulSync>, Condvar)>>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Default for EmulMgr {
    fn default() -> Self {
        Self {
            emulator: None,
            thread_handle: None,
        }
    }
}

impl EmulMgr {
    pub fn set_emulator(&mut self, emulator: Box<dyn Emulator + Send>) {
        if let Some(rc) = &self.emulator {
            let (e, c) = rc.deref();
            e.lock().unwrap().stop = true;
        }

        let emul_sync = EmulSync {
            emulator,
            stop: false,
            pause: false,
        };
        let emul_rc = Arc::new((Mutex::new(emul_sync), Condvar::new()));
        self.thread_handle = Some(self.spawn_thread(&emul_rc));
        self.emulator = Some(emul_rc);
    }

    fn spawn_thread(&mut self, emulator: &Arc<(Mutex<EmulSync>, Condvar)>) -> JoinHandle<()> {
        let emul_rc = Arc::clone(emulator);
        thread::spawn(move || {
            let (e, c) = emul_rc.deref();
            while !e.lock().unwrap().stop {
                let res = e.lock().unwrap().emulator.cycle();
                if let Ok(r) = res {
                    if r.cycle_count % 1_000_000 == 0 {
                        println!("{:?}", r.cycle_count);
                    }
                }
            }
        })
    }

    pub fn video_buffer(&self) -> Result<Vec<u8>, ()> {
        if let Some(emul_rc) = &self.emulator {
            let (e, c) = emul_rc.deref();
            return Ok(e.lock().unwrap().emulator.video_buffer());
        }
        Err(())
    }
}
