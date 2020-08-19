use tge::prelude::*;

const TITLE: &str = "Custom Mouse Cursor";

struct App {
    cursor: Texture,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let cursor = Texture::load(engine, "assets/cursor.png")?;
        Ok(Self {
            cursor,
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

        if let Some(position) = engine.mouse().position() {
            engine.graphics().draw_sprite(
                &self.cursor,
                None,
                Transform::default()
                    .translate(position),
            );
        }

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800.0, 600.0)))
        .mouse_config(MouseConfig::new()
            .cursor_visible(false))
        .build()?
        .run_with(App::new)
}
