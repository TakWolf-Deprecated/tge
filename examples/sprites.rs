use tge::error::GameResult;
use tge::math::{Vector, Position, Point, Scale, Angle};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::mouse::MouseButton;
use tge::game::Game;
use rand::Rng;
use rand::rngs::ThreadRng;

const TITLE: &str = "Sprites";
const STEP_COUNT: usize = 100;

struct Sprite {
    position: Position,
    speed: Vector,
    angle: Angle,
    angle_speed: Angle,
    scale: Scale,
    color: Color,
}

fn create_sprites(engine: &mut Engine, rand: &mut ThreadRng, count: usize) -> Vec<Sprite> {
    let mut sprites = Vec::with_capacity(count);
    let graphics_size = engine.graphics().size();
    for _ in 0..count {
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
        sprites.push(Sprite {
            position: Position::new(x, y),
            speed: Vector::new(speed_x, speed_y),
            angle: Angle::radians(angle),
            angle_speed: Angle::radians(angle_speed),
            scale: Scale::new(scale, scale),
            color: Color::new(red, green, blue, alpha),
        });
    }
    sprites
}

struct App {
    zazaka: Texture,
    rand: ThreadRng,
    sprites: Vec<Sprite>,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let zazaka = Texture::load(engine, "assets/zazaka.png")?;
        let mut rand = rand::thread_rng();
        let mut sprites = Vec::new();
        sprites.extend(create_sprites(engine, &mut rand, STEP_COUNT));
        Ok(Self {
            zazaka,
            rand,
            sprites,
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{}: {} - FPS: {}", TITLE, self.sprites.len(), engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if engine.mouse().is_button_down(MouseButton::Left) {
            self.sprites.extend(create_sprites(engine, &mut self.rand, STEP_COUNT));
        }

        let graphics_size = engine.graphics().size();
        let delta_time_f32 = engine.timer().delta_time().as_secs_f32();
        for sprite in &mut self.sprites {
            sprite.position.x += sprite.speed.x * delta_time_f32;
            sprite.position.y += sprite.speed.y * delta_time_f32;
            if sprite.position.x < 0.0 {
                sprite.position.x = 0.0;
                sprite.speed.x *= -1.0;
            }
            if sprite.position.x > graphics_size.width {
                sprite.position.x = graphics_size.width;
                sprite.speed.x *= -1.0;
            }
            if sprite.position.y < 0.0 {
                sprite.position.y = 0.0;
                sprite.speed.y *= -1.0;
            }
            if sprite.position.y > graphics_size.height {
                sprite.position.y = graphics_size.height;
                sprite.speed.y *= -1.0;
            }
            sprite.angle += sprite.angle_speed * delta_time_f32;
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        let origin = {
            let size = self.zazaka.size();
            Point::new(size.width as f32 / 2.0, size.height as f32 / 2.0)
        };

        for sprite in self.sprites.iter() {
            engine.graphics().draw_sprite(
                Some(&self.zazaka),
                SpriteDrawParams::default()
                    .origin(origin)
                    .position(sprite.position)
                    .rotation(sprite.angle)
                    .scale(sprite.scale)
                    .color(sprite.color),
            );
        }

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((1280, 720)))
        .graphics_config(GraphicsConfig::new()
            .default_filter(Filter::new(
                FilterMode::Nearest,
                FilterMode::Nearest,
                None,
            )))
        .build()?
        .run_with(App::new)
}
