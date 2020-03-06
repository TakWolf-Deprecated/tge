use tge::error::GameResult;
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::Color;
use tge::game::Game;

struct App {}

impl App {

    fn new(_: &mut Engine) -> GameResult<Self> {
        // load assets
        Ok(Self {})
    }

}

impl Game for App {

    fn update(&mut self, _: &mut Engine) -> GameResult {
        // handle logic
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLUE);
        // draw sprites
        Ok(())
    }

}

fn main() -> GameResult {
    let mut engine = EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title("My Game")
            .inner_size((800, 600)))
        .build()?;
    let mut app = App::new(&mut engine)?;
    engine.run(&mut app)
}
