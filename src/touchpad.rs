use crate::error::GameResult;
use crate::math::Delta;

pub struct Touchpad {
    scroll_delta: Delta,
    pressure: f32,
    click_stage: i64,
}

impl Touchpad {

    pub(crate) fn new(_: TouchpadConfig) -> GameResult<Self> {
        Ok(Self {
            scroll_delta: Delta::zero(),
            pressure: 0.0,
            click_stage: 0,
        })
    }

    pub(crate) fn handle_scroll_event(&mut self, delta: Delta) {
        self.scroll_delta += delta;
    }

    pub(crate) fn handle_press_event(&mut self, pressure: f32, click_stage: i64) {
        self.pressure = pressure;
        self.click_stage = click_stage;
    }

    pub(crate) fn clear_states(&mut self) {
        self.scroll_delta.set(0.0, 0.0);
    }

    pub fn scroll_delta(&self) -> Delta {
        self.scroll_delta
    }

    pub fn pressure(&self) -> f32 {
        self.pressure
    }

    pub fn click_stage(&self) -> i64 {
        self.click_stage
    }

}

#[derive(Debug, Clone)]
pub struct TouchpadConfig {}

impl TouchpadConfig {

    pub fn new() -> Self {
        Self {}
    }

}
