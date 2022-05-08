
pub struct CycleResult {
    pub video_buff_changed: bool,
}

impl Default for CycleResult {
    fn default() -> Self {
        Self {
            video_buff_changed: false
        }
    }
}

pub trait Cpu {
    fn cycle(&mut self) -> CycleResult;
}