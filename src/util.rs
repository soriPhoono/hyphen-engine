use std::{
    thread,
    time::{Duration, Instant},
};

pub struct FPSTimer {
    start: Instant,
    target_fps: u32,
}

impl FPSTimer {
    pub fn new(target_fps: u32) -> Self {
        Self {
            start: Instant::now(),

            target_fps,
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn set_target_fps(&mut self, target_fps: u32) {
        self.target_fps = target_fps;
    }

    pub fn get_remaining(&self) -> Duration {
        let target_duration = Duration::from_secs(1) / self.target_fps;
        let elapsed = self.start.elapsed();

        if elapsed > target_duration {
            Duration::from_secs(0)
        } else {
            target_duration - elapsed
        }
    }

    pub fn wait(&self) {
        thread::sleep(self.get_remaining());
    }
}
