use tge::prelude::*;

const TITLE: &str = "Responsive";

struct App {
    view_size: Size,
    canvas: Canvas,
    sky: Texture,
    target_x: f32,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let view_size = Size::<f32>::new(320.0, 256.0);
        let canvas = Canvas::new(engine.graphics(), Size::new(view_size.width.round() as u32, view_size.height.round() as u32))?;
        let sky = Texture::load(engine, "assets/sky.png")?;
        Ok(Self {
            view_size,
            canvas,
            sky,
            target_x: 0.0,
        })
    }

    fn draw_scene(&mut self, engine: &mut Engine) {
        let sky_size = self.sky.size();
        engine.graphics().draw_sprite(
            &self.sky,
            SpriteDrawParams::default()
                .region((self.target_x, 0.0, sky_size.width as f32, sky_size.height as f32)),
            None,
        );
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.target_x += 1.0;
        if self.target_x >= self.sky.size().width as f32 {
            self.target_x = 0.0;
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().set_canvas(Some(&self.canvas));
        engine.graphics().clear(Color::BLACK);

        self.draw_scene(engine);

        engine.graphics().set_canvas(None);
        engine.graphics().clear(Color::BLACK);

        let graphics_size = engine.graphics().size();
        let position;
        let scale;
        if graphics_size.width / graphics_size.height <= self.view_size.width / self.view_size.height {
            scale = graphics_size.width / self.view_size.width;
            position = Position::new(0.0, (graphics_size.height - self.view_size.height * scale) / 2.0);
        } else {
            scale = graphics_size.height / self.view_size.height;
            position = Position::new((graphics_size.width - self.view_size.width * scale) / 2.0, 0.0);
        }

        engine.graphics().draw_sprite(
            &self.canvas,
            None,
            Transform::default()
                .scale((scale, scale))
                .translate(position),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((320.0 * 2.0, 256.0 * 2.0)))
        .build()?
        .run_with(App::new)
}
