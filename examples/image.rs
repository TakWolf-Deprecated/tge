use tge::prelude::*;

const TITLE: &str = "Image";

struct App {
    texture_ferris: Texture,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let texture_ferris = Texture::load(engine, "assets/ferris.png")?;
        Ok(Self {
            texture_ferris,
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
            &self.texture_ferris,
            None,
            Transform::default()
                .scale((0.5, 0.5)),
        );
        engine.graphics().draw_sprite(
            &self.texture_ferris,
            SpriteDrawParams::default()
                .colors([
                    Color::TRANSPARENT_WHITE,
                    Color::TRANSPARENT_BLACK,
                    Color::WHITE,
                    Color::WHITE,
                ]),
            Transform::default()
                .scale((0.5, -0.5))
                .translate((0.0, 700.0)),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((600.0, 700.0)))
        .build()?
        .run_with(App::new)
}
