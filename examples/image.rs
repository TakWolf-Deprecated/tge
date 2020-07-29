use tge::error::GameResult;
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::game::Game;

const TITLE: &str = "Image";

struct App {
    ferris: Texture,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let ferris = Texture::load(engine, "assets/ferris.png")?;
        Ok(Self {
            ferris,
        })
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

        engine.graphics().draw_sprite(
            &self.ferris,
            SpriteDrawParams::default(),
            TransformParams::default()
                .scale((0.5, 0.5)),
        );
        engine.graphics().draw_sprite(
            &self.ferris,
            SpriteDrawParams::default()
                .colors([
                    Color::TRANSPARENT,
                    Color::TRANSPARENT,
                    Color::WHITE,
                    Color::WHITE,
                ]),
            TransformParams::default()
                .position((0.0, 700.0))
                .scale((0.5, -0.5)),
        );

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800.0, 600.0)))
        .build()?
        .run_with(App::new)
}
