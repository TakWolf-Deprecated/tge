use crate::error::GameResult;
use crate::engine::Engine;
use crate::event::Event;

pub trait Game {

    fn update(&mut self, engine: &mut Engine) -> GameResult;

    fn render(&mut self, engine: &mut Engine) -> GameResult;

    fn event(&mut self, _engine: &mut Engine, _event: Event) -> GameResult<bool> {
        return Ok(false)
    }

}
