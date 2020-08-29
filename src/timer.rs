use crate::error::GameResult;
use std::time::{Instant, Duration};

pub struct Timer {
    frame_duration: Duration,
    last_frame_instant: Instant,
    delta_time: Duration,
}

impl Timer {
    pub(crate) fn new(timer_config: TimerConfig) -> GameResult<Self> {
        Ok(Self {
            frame_duration: timer_config.frame_duration,
            last_frame_instant: Instant::now(),
            delta_time: Duration::new(0, 0),
        })
    }

    pub(crate) fn reset_tick(&mut self) {
        self.last_frame_instant = Instant::now();
        self.delta_time = Duration::new(0, 0);
    }

    pub(crate) fn tick_and_check(&mut self) -> bool {
        let now_instant = Instant::now();
        let delta_time = now_instant.duration_since(self.last_frame_instant);
        if delta_time >= self.frame_duration {
            self.last_frame_instant = now_instant;
            self.delta_time = delta_time;
            true
        } else {
            false
        }
    }

    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    pub fn set_frame_duration(&mut self, frame_duration: Duration) {
        self.frame_duration = frame_duration;
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.frame_duration.as_secs_f32()
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.frame_duration = Duration::from_secs_f32(1.0 / fps);
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
    frame_duration: Duration,
}

impl TimerConfig {
    pub fn new() -> Self {
        Self {
            frame_duration: Duration::from_secs_f32(1.0 / 60.0),
        }
    }

    pub fn frame_duration(mut self, frame_duration: Duration) -> Self {
        self.frame_duration = frame_duration;
        self
    }

    pub fn fps(mut self, fps: f32) -> Self {
        self.frame_duration = Duration::from_secs_f32(1.0 / fps);
        self
    }
}
