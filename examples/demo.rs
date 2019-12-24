use tge::error::GameResult;
use tge::engine::{EngineBuilder, Engine};
use tge::window::WindowConfig;
use tge::graphics::GraphicsConfig;
use tge::timer::TimerConfig;

struct App {}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        Ok(Self {})
    }

}

fn main() {
    let mut engine = EngineBuilder::new()
        .window_config(WindowConfig::new())
        .graphics_config(GraphicsConfig::new())
        .timer_config(TimerConfig::new())
        .build()
        .unwrap();
    let mut app = App::new(&mut engine).unwrap();
}
