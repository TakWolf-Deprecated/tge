use tge::error::GameResult;
use tge::engine::{EngineBuilder, Engine};
use tge::window::WindowConfig;
use tge::graphics::GraphicsConfig;
use tge::timer::TimerConfig;
use tge::game::Game;

struct App {}

impl App {

    fn new(_engine: &mut Engine) -> GameResult<Self> {
        Ok(Self {})
    }

}

impl Game for App {

    fn update(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }

    fn render(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new())
        .graphics_config(GraphicsConfig::new())
        .timer_config(TimerConfig::new())
        .build()?
        .run_with(App::new)
}
