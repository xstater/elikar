use std::time::{Duration, Instant};

pub struct States {
    pub(in super) quit : bool,
    pub(in super) frames_in_sec : u32,
    pub(in super) frame_counter : u32,
    pub(in super) sec_timer : Instant,
    pub(in super) last_frame_time : Duration
}

impl States {
    pub(in super) fn new() -> Self {
        States {
            quit : false,
            frames_in_sec : 1,
            frame_counter : 0,
            sec_timer : Instant::now(),
            last_frame_time : Duration::from_secs(1)
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn actual_fps(&self) -> u32 {
        self.frames_in_sec
    }

    pub fn fps(&self) -> f64 {
        1.0 / self.last_frame_time.as_secs_f64()
    }
}
