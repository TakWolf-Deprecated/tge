use crate::error::GameResult;
use crate::engine::Engine;
use crate::event::Event;

pub trait Game {
    fn update(&mut self, engine: &mut Engine) -> GameResult;

    fn render(&mut self, engine: &mut Engine) -> GameResult;

    fn event(&mut self, _: &mut Engine, _: Event) -> GameResult<bool> {
        Ok(false)
    }
}
