use std::time::{Duration, Instant};

pub struct Time {
    // Count how many frames until now
    frame_counter : usize,
    // the instant when leave a frame
    frame_timer : Instant,
    // Time of last frame
    last_frame_time : Duration,
    // One second timer
    one_sec_timer : Instant,
    // Record the frame count in one second start
    one_sec_frame : usize,
    // Last one second frames 
    last_one_sec_frames : usize,
}

impl Time {
    pub(in crate) fn new() -> Time {
        Time {
            frame_counter : 0,
            frame_timer : Instant::now(),
            last_frame_time : Duration::from_secs(1),
            one_sec_timer : Instant::now(),
            one_sec_frame : 0,
            last_one_sec_frames : 0
        }
    }

    pub(in crate) fn tick(&mut self) {
        self.frame_counter += 1;
        self.last_frame_time = self.frame_timer.elapsed();
        self.frame_timer = Instant::now();
        if self.one_sec_timer.elapsed() > Duration::from_secs(1) {
            self.one_sec_timer = Instant::now();
            self.last_one_sec_frames = self.frame_counter - self.one_sec_frame;
            self.one_sec_frame = self.frame_counter;
        }
    }

    // Get the time from frame start to now
    pub fn delta(&self) -> Duration {
        self.frame_timer.elapsed()
    }

    pub fn actual_fps(&self) -> usize {
        self.last_one_sec_frames
    }

    pub fn fps(&self) -> f64 {
        1.0 / self.last_frame_time.as_secs_f64()
    }

    pub fn frame_counter(&self) -> usize {
        self.frame_counter
    }
}
