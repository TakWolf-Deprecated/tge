/// Based on https://github.com/openfl/openfl-samples/tree/master/demos/BunnyMark
/// and https://github.com/17cupsofcoffee/tetra/blob/master/examples/bunnymark.rs
use tge::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

const TITLE: &str = "Bunny Mark";
const STEP_COUNT: usize = 100;
const GRAVITY: f32 = 0.5;

struct Bunny {
    position: Position,
    speed: Vector,
}

impl Bunny {

    fn new(rand: &mut ThreadRng) -> Self {
        let speed_x = rand.gen::<f32>() * 5.0;
        let speed_y = rand.gen::<f32>() * 5.0 - 2.5;
        Self {
            position: Position::zero(),
            speed: Vector::new(speed_x, speed_y),
        }
    }

}

struct App {
    wabbit: Texture,
    rand: ThreadRng,
    bunnies: Vec<Bunny>,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let wabbit = Texture::load(engine, "assets/wabbit_alpha.png")?;
        let mut rand = rand::thread_rng();
        let mut bunnies = Vec::with_capacity(STEP_COUNT);
        for _ in 0..STEP_COUNT {
            bunnies.push(Bunny::new(&mut rand));
        }
        Ok(Self {
            wabbit,
            rand,
            bunnies,
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{}: {} - FPS: {}", TITLE, self.bunnies.len(), engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if engine.mouse().is_button_down(MouseButton::Left) {
            for _ in 0..STEP_COUNT {
                self.bunnies.push(Bunny::new(&mut self.rand));
            }
        }

        let max_position = {
            let graphics_size = engine.graphics().size();
            let texture_size = self.wabbit.size();
            Position::new(graphics_size.width - texture_size.width as f32, graphics_size.height - texture_size.height as f32)
        };

        for bunny in &mut self.bunnies {
            bunny.position += bunny.speed;
            bunny.speed.y += GRAVITY;
            if bunny.position.x < 0.0 {
                bunny.position.x = 0.0;
                bunny.speed.x *= -1.0;
            }
            if bunny.position.x > max_position.x {
                bunny.position.x = max_position.x;
                bunny.speed.x *= -1.0;
            }
            if bunny.position.y < 0.0 {
                bunny.position.y = 0.0;
                bunny.speed.y = 0.0;
            }
            if bunny.position.y > max_position.y {
                bunny.position.y = max_position.y;
                bunny.speed.y *= -0.8;
                if self.rand.gen::<bool>() {
                    bunny.speed.y -= self.rand.gen::<f32>() * 4.0 + 3.0;
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear((0.392, 0.584, 0.929, 1.0));

        for bunny in &self.bunnies {
            engine.graphics().draw_sprite(
                &self.wabbit,
                SpriteDrawParams::default(),
                TransformParams::default()
                    .position(bunny.position),
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
        .graphics_config(GraphicsConfig::new()
            .default_filter(Filter::new(
                FilterMode::Nearest,
                FilterMode::Nearest,
                None,
            )))
        .build()?
        .run_with(App::new)
}
