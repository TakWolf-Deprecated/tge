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

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("FPS: {}", engine.timer().real_time_fps().round());
        engine.window().set_title(title);
        Ok(())
    }

    fn render(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title("My Game")
            .inner_size((800, 600)))
        .timer_config(TimerConfig::new()
            .fps(80.0))
        .build()?
        .run_with(App::new)
}
