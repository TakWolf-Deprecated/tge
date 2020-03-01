use tge::error::GameResult;
use tge::math::{Vector, Position};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::game::Game;

const TITLE: &str = "Vertices";

struct App {}

impl App {

    fn new(_: &mut Engine) -> GameResult<Self> {
        Ok(Self {})
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::WHITE);

        let vertices = vec![
            Vertex {
                position: Position::new(100.0, 100.0),
                uv: Vector::new(0.0, 0.0),
                color: Color::RED,
            },
            Vertex {
                position: Position::new(200.0, 500.0),
                uv: Vector::new(0.0, 0.0),
                color: Color::GREEN,
            },
            Vertex {
                position: Position::new(400.0, 200.0),
                uv: Vector::new(0.0, 0.0),
                color: Color::BLUE,
            },
        ];
        engine.graphics().draw_vertices(None, PrimitiveType::Triangles, vertices, None);

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800, 600)))
        .build()?
        .run_with(App::new)
}
