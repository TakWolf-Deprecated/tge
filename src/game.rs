use crate::error::GameResult;
use crate::engine::Engine;

pub trait Game {

    fn update(&mut self, engine: &mut Engine) -> GameResult;

    fn render(&mut self, engine: &mut Engine) -> GameResult;

}
