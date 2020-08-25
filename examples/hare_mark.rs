use tge::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

const TITLE: &str = "Hare Mark";
const STEP_COUNT: usize = 100;

struct Hare {
    position: Position,
    speed: Vector,
    angle: Angle,
    angle_speed: Angle,
    scale: Vector,
    color: Color,
}

impl Hare {
    fn new(rand: &mut ThreadRng, graphics_size: &Size) -> Self {
        let x = rand.gen_range(0.0, graphics_size.width);
        let y = rand.gen_range(0.0, graphics_size.height);
        let speed_x = rand.gen_range(-100.0, 100.0);
        let speed_y = rand.gen_range(-100.0, 100.0);
        let angle = rand.gen_range(0.0, std::f32::consts::PI * 2.0);
        let angle_speed = rand.gen_range(-10.0, 10.0);
        let scale = rand.gen_range(0.5, 1.0);
        let red = rand.gen_range(0.5, 1.0);
        let green = rand.gen_range(0.5, 1.0);
        let blue = rand.gen_range(0.5, 1.0);
        let alpha = rand.gen_range(0.5, 1.0);
        Self {
            position: Position::new(x, y),
            speed: Vector::new(speed_x, speed_y),
            angle: Angle::radians(angle),
            angle_speed: Angle::radians(angle_speed),
            scale: Vector::new(scale, scale),
            color: Color::new(red, green, blue, alpha),
        }
    }
}

struct App {
    zazaka: Texture,
    rand: ThreadRng,
    hares: Vec<Hare>,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let zazaka = Texture::load(engine, "assets/zazaka.png")?;
        let mut rand = rand::thread_rng();
        let mut hares = Vec::with_capacity(STEP_COUNT);
        let graphics_size = engine.graphics().size();
        for _ in 0..STEP_COUNT {
            hares.push(Hare::new(&mut rand, &graphics_size));
        }
        Ok(Self {
            zazaka,
            rand,
            hares,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{}: {} - FPS: {}", TITLE, self.hares.len(), engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        let delta_time_f32 = engine.timer().delta_time().as_secs_f32();
        let graphics_size = engine.graphics().size();

        for hare in &mut self.hares {
            hare.position.x += hare.speed.x * delta_time_f32;
            hare.position.y += hare.speed.y * delta_time_f32;
            if hare.position.x < 0.0 {
                hare.position.x = 0.0;
                hare.speed.x *= -1.0;
            }
            if hare.position.x > graphics_size.width {
                hare.position.x = graphics_size.width;
                hare.speed.x *= -1.0;
            }
            if hare.position.y < 0.0 {
                hare.position.y = 0.0;
                hare.speed.y *= -1.0;
            }
            if hare.position.y > graphics_size.height {
                hare.position.y = graphics_size.height;
                hare.speed.y *= -1.0;
            }
            hare.angle += hare.angle_speed * delta_time_f32;
        }

        if engine.mouse().is_button_down(MouseButton::Left) {
            for _ in 0..STEP_COUNT {
                self.hares.push(Hare::new(&mut self.rand, &graphics_size));
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        let origin = {
            let size = self.zazaka.size();
            Position::new(size.width as f32 / 2.0, size.height as f32 / 2.0)
        };

        for hare in &self.hares {
            engine.graphics().draw_sprite(
                &self.zazaka,
                SpriteDrawParams::default()
                    .origin(origin)
                    .color(hare.color),
                Transform::default()
                    .scale(hare.scale)
                    .rotate(hare.angle)
                    .translate(hare.position),
            );
        }

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((1280.0, 720.0)))
        .build()?
        .run_with(App::new)
}
