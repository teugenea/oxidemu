
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

pub trait Cpu {
    fn cycle(&mut self) -> CycleResult;
}