use crate::error::GameResult;
use std::time::{Instant, Duration};

pub struct Timer {
    fps: f32,
    last_instant: Instant,
    delta_time: Duration,
}

impl Timer {

    pub(crate) fn new(timer_config: TimerConfig) -> GameResult<Self> {
        Ok(Self {
            fps: timer_config.fps,
            last_instant: Instant::now(),
            delta_time: Duration::new(0, 0),
        })
    }

    pub(crate) fn reset_tick(&mut self) {
        self.last_instant = Instant::now();
        self.delta_time = Duration::new(0, 0);
    }

    pub(crate) fn tick_and_check(&mut self) -> bool {
        let now_instant = Instant::now();
        let delta_time = now_instant.duration_since(self.last_instant);
        if delta_time.as_secs_f32() >= 1.0 / self.fps {
            self.last_instant = now_instant;
            self.delta_time = delta_time;
            true
        } else {
            false
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn set_fps(&mut self, fps: f32) {
        check_fps(fps);
        self.fps = fps;
    }

    pub fn real_time_fps(&self) -> f32 {
        let delta_time_secs_f32 = self.delta_time.as_secs_f32();
        if delta_time_secs_f32 > 0.0 {
            1.0 / delta_time_secs_f32
        } else {
            0.0
        }
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

}

#[derive(Debug, Clone)]
pub struct TimerConfig {
    fps: f32,
}

impl TimerConfig {

    pub fn new() -> Self {
        Self { fps: 60.0 }
    }

    pub fn fps(mut self, fps: f32) -> Self {
        check_fps(fps);
        self.fps = fps;
        self
    }

}

fn check_fps(fps: f32) {
    assert!(fps > 0.0, "fps must > 0.0");
}
