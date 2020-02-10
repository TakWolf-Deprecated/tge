use crate::error::GameResult;
use crate::math::Delta;

pub struct Touchpad {
    scroll_delta: Delta,
    pressure_level: f32,
    click_level: i64,
}

impl Touchpad {

    pub(crate) fn new(touchpad_config: TouchpadConfig) -> GameResult<Self> {
        Ok(Self {
            scroll_delta: Delta::zero(),
            pressure_level: 0.0,
            click_level: 0,
        })
    }

    pub(crate) fn handle_scroll_event(&mut self, delta: Delta) {
        self.scroll_delta += delta;
    }

    pub(crate) fn handle_press_event(&mut self, pressure_level: f32, click_level: i64) {
        self.pressure_level = pressure_level;
        self.click_level = click_level;
    }

    pub(crate) fn reset_states(&mut self) {
        self.scroll_delta.set(0.0, 0.0);
    }

    pub fn scroll_delta(&self) -> Delta {
        self.scroll_delta
    }

    pub fn pressure_level(&self) -> f32 {
        self.pressure_level
    }

    pub fn click_level(&self) -> i64 {
        self.click_level
    }

}

#[derive(Debug, Clone)]
pub struct TouchpadConfig {}

impl TouchpadConfig {

    pub fn new() -> Self {
        Self {}
    }

}
