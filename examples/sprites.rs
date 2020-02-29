use tge::error::GameResult;
use tge::math::{Vector, Position, Point, Scale, Angle};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::timer::TimerConfig;
use tge::game::Game;
use rand::Rng;

const TITLE: &str = "Sprites";
const SPRITE_COUNT: usize = 1000;

struct Sprite {
    position: Position,
    speed: Vector,
    angle: Angle,
    angle_speed: Angle,
    scale: Scale,
    color: Color,
}

struct App {
    zazaka: Texture,
    sprites: Vec<Sprite>,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let zazaka = Texture::load(engine, "assets/zazaka.png")?;

        let mut rand = rand::thread_rng();
        let mut sprites = Vec::with_capacity(SPRITE_COUNT);
        let graphics_size = engine.graphics().size();
        for _ in 0..SPRITE_COUNT {
            let x = rand.gen_range(0.0, graphics_size.width);
            let y = rand.gen_range(0.0, graphics_size.height);
            let speed_x = rand.gen_range(-100.0, 100.0);
            let speed_y = rand.gen_range(-100.0, 100.0);
            let angle = rand.gen_range(0.0, std::f32::consts::PI * 2.0);
            let angle_speed = rand.gen_range(-10.0, 10.0);
            let scale = rand.gen_range(1.0, 2.0);
            let red = rand.gen_range(0.5, 1.0);
            let green = rand.gen_range(0.5, 1.0);
            let blue = rand.gen_range(0.5, 1.0);
            let alpha = rand.gen_range(0.5, 1.0);
            sprites.push(Sprite {
                position: Position::new(x, y),
                speed: Vector::new(speed_x, speed_y),
                angle: Angle::Radians(angle),
                angle_speed: Angle::Radians(angle_speed),
                scale: Scale::new(scale, scale),
                color: Color::new(red, green, blue, alpha),
            });
        }

        Ok(Self {
            zazaka,
            sprites,
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        let graphics_size = engine.graphics().size();
        let delta_time_f32 = engine.timer().delta_time().as_secs_f32();
        for sprite in self.sprites.iter_mut() {
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
            .inner_size((800, 600)))
        .timer_config(TimerConfig::new()
            .fps(1000.0))
        .build()?
        .run_with(App::new)
}
