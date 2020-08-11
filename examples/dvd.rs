use tge::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

const TITLE: &str = "DVD";

struct App {
    dvd_logo: Texture,
    size: Size,
    position: Position,
    speed: Vector,
    color: Color,
    rand: ThreadRng,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let dvd_logo = Texture::load(engine, "assets/dvd-logo.png")?;
        let size = {
            let size = dvd_logo.size();
            Size::new(size.width as f32, size.height as f32)
        };
        let graphics_size = engine.graphics().size();
        let position = Position::new(graphics_size.width / 2.0, graphics_size.height / 2.0);
        let speed = Vector::new(1.0, 1.0);
        let color = Color::new(0.5, 0.5, 1.0, 1.0);
        let rand = rand::thread_rng();
        Ok(Self {
            dvd_logo,
            size,
            position,
            speed,
            color,
            rand,
        })
    }

    fn set_logo_random_color(&mut self) {
        let red = self.rand.gen::<f32>() * 0.6 + 0.4;
        let green = self.rand.gen::<f32>() * 0.6 + 0.4;
        let blue = self.rand.gen::<f32>() * 0.6 + 0.4;
        self.color = Color::new(red, green, blue, 1.0);
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        let graphics_size = engine.graphics().size();
        self.position += self.speed;
        if self.position.x < self.size.width / 2.0 {
            self.position.x = self.size.width / 2.0;
            self.speed.x *= -1.0;
            self.set_logo_random_color();
        } else if self.position.x > graphics_size.width - self.size.width / 2.0 {
            self.position.x = graphics_size.width - self.size.width / 2.0;
            self.speed.x *= -1.0;
            self.set_logo_random_color();
        }
        if self.position.y < self.size.height / 2.0 {
            self.position.y = self.size.height / 2.0;
            self.speed.y *= -1.0;
            self.set_logo_random_color();
        } else if self.position.y > graphics_size.height - self.size.height / 2.0 {
            self.position.y = graphics_size.height - self.size.height / 2.0;
            self.speed.y *= -1.0;
            self.set_logo_random_color();
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        engine.graphics().draw_sprite(
            &self.dvd_logo,
            SpriteDrawParams::default()
                .color(self.color),
            TransformParams::default()
                .position(self.position)
                .origin((self.size.width / 2.0, self.size.height / 2.0)),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800.0, 600.0))
            .resizable(false))
        .build()?
        .run_with(App::new)
}
