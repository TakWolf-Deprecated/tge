use tge::error::GameResult;
use tge::math::{Position, Angle};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::keyboard::KeyCode;
use tge::game::Game;

const TITLE: &str = "Inertia";

struct App {
    car: Texture,
    position: Position,
    speed: f32,
    max_speed: f32,
    speed_acceleration: f32,
    friction_acceleration: f32,
    angle: Angle,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let car = Texture::load(engine, "assets/car.png")?;
        let graphics_size = engine.graphics().size();
        Ok(Self {
            car,
            position: Position::new(graphics_size.width / 2.0, graphics_size.height / 2.0),
            speed: 0.0,
            max_speed: 6.0,
            speed_acceleration: 0.3,
            friction_acceleration: 0.08,
            angle: Angle::zero(),
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if engine.keyboard().is_key_hold(KeyCode::Up) || engine.keyboard().is_key_hold(KeyCode::W) {
            if self.speed < self.max_speed {
                self.speed += self.speed_acceleration
            } else if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        }
        if engine.keyboard().is_key_hold(KeyCode::Down) || engine.keyboard().is_key_hold(KeyCode::S) {
            if self.speed > -self.max_speed {
                self.speed -= self.speed_acceleration
            } else if self.speed < -self.max_speed {
                self.speed = -self.max_speed;
            }
        }

        if self.speed > 0.0 {
            self.speed -= self.friction_acceleration;
            if self.speed < 0.0 {
                self.speed = 0.0;
            }
        } else if self.speed < 0.0 {
            self.speed += self.friction_acceleration;
            if self.speed > 0.0 {
                self.speed = 0.0;
            }
        }

        if self.speed != 0.0 {
            let angle_speed = Angle::degrees(self.speed / 2.0);
            if engine.keyboard().is_key_hold(KeyCode::Left) || engine.keyboard().is_key_hold(KeyCode::A) {
                self.angle -= angle_speed;
            }
            if engine.keyboard().is_key_hold(KeyCode::Right) || engine.keyboard().is_key_hold(KeyCode::D) {
                self.angle += angle_speed;
            }
        }

        self.position.x += self.speed * self.angle.radians_value().sin();
        self.position.y -= self.speed * self.angle.radians_value().cos();

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::new(0.6, 0.6, 0.6, 1.0));

        engine.graphics().draw_sprite(
            Some(&self.car),
            SpriteDrawParams::default()
                .origin((50.0, 105.0))
                .position(self.position)
                .rotation(self.angle)
                .scale((0.3, 0.3)),
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
