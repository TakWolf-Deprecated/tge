use tge::error::GameResult;
use tge::math::{Vector, Position};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::game::Game;
use rand::Rng;
use rand::rngs::ThreadRng;

const TITLE: &str = "Shape";

struct App {
    rand: ThreadRng,
}

impl App {

    fn new(_: &mut Engine) -> GameResult<Self> {
        let rand = rand::thread_rng();
        Ok(Self { rand })
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

        // point
        for j in 0..50 {
            for i in 0..50 {
                let i = i as f32;
                let j = j as f32;
                let red = self.rand.gen_range(0.5, 1.0);
                let green = self.rand.gen_range(0.5, 1.0);
                let blue = self.rand.gen_range(0.5, 1.0);
                engine.graphics().draw_sprite(
                    None,
                    SpriteDrawParams::default()
                        .position((10.0 * i, 10.0 * j))
                        .region((0.0, 0.0, 1.0, 1.0))
                        .color(Color::new(red, green, blue, 1.0)),
                );
            }
        }

        // line
        engine.graphics().draw_sprite(
            None,
            SpriteDrawParams::default()
                .position((100.0, 100.0))
                .region((0.0, 0.0, 1.0, 300.0))
                .colors([
                    Color::RED,
                    Color::RED,
                    Color::GREEN,
                    Color::GREEN,
                ]),
        );

        // triangle
        let vertices = vec![
            Vertex {
                position: Position::new(600.0, 100.0),
                uv: Vector::new(0.0, 0.0),
                color: Color::RED,
            },
            Vertex {
                position: Position::new(700.0, 500.0),
                uv: Vector::new(0.0, 0.0),
                color: Color::GREEN,
            },
            Vertex {
                position: Position::new(400.0, 200.0),
                uv: Vector::new(0.0, 0.0),
                color: Color::BLUE,
            },
        ];
        engine.graphics().draw_mesh(None, PrimitiveType::Triangles, vertices, None);

        // rectangle
        engine.graphics().draw_sprite(
            None,
            SpriteDrawParams::default()
                .position((150.0, 350.0))
                .region((0.0, 0.0, 300.0, 200.0))
                .colors([
                    Color::YELLOW,
                    Color::YELLOW,
                    Color::GREEN,
                    Color::GREEN,
                ]),
        );

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
