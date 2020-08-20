use tge::prelude::*;
use std::time::Instant;

const TITLE: &str = "Cannon";
const BULLET_SPEED: f32 = 4.0;
const BULLET_CD_SECS: f32 = 0.06;

struct Cannon {
    position: Position,
    angle: Angle,
}

impl Cannon {
    fn look_at(&mut self, target: Position) {
        let x = target.x - self.position.x;
        let y = target.y - self.position.y;
        self.angle = Angle::radians(x.atan2(-y));
    }

    fn gun_position_1(&self) -> Position {
        let angle = self.angle - Angle::n_pi(0.70);
        let x = self.position.x + 20.0 * angle.radians_value().cos();
        let y = self.position.y + 20.0 * angle.radians_value().sin();
        Position::new(x, y)
    }

    fn gun_position_2(&self) -> Position {
        let angle = self.angle - Angle::n_pi(0.30);
        let x = self.position.x + 20.0 * angle.radians_value().cos();
        let y = self.position.y + 20.0 * angle.radians_value().sin();
        Position::new(x, y)
    }

    fn draw(&self, engine: &mut Engine, texture: &Texture) {
        engine.graphics().draw_sprite(
            texture,
            SpriteDrawParams::default()
                .origin((16.0, 16.0)),
            Transform::default()
                .rotate(self.angle)
                .translate(self.position),
        );
    }
}

struct Bullet {
    position: Position,
    angle: Angle,
}

impl Bullet {
    fn update(&mut self, engine: &mut Engine) {
        let factor = engine.timer().delta_time().as_secs_f32() * 60.0;
        self.position.x += BULLET_SPEED * factor * self.angle.radians_value().sin();
        self.position.y -= BULLET_SPEED * factor * self.angle.radians_value().cos();
    }

    fn is_alive(&self, engine: &mut Engine) -> bool {
        let graphics_size = engine.graphics().size();
        self.position.x >= 0.0 && self.position.x <= graphics_size.width && self.position.y >= 0.0 && self.position.y <= graphics_size.height
    }

    fn draw(&self, engine: &mut Engine, texture: &Texture) {
        engine.graphics().draw_sprite(
            texture,
            SpriteDrawParams::default()
                .origin((23.5, 23.5)),
            Transform::default()
                .scale((0.2, 0.2))
                .rotate(self.angle)
                .translate(self.position),
        );
    }
}

struct App {
    font: Font,
    texture_cannon: Texture,
    texture_bullet: Texture,
    cannons: Vec<Cannon>,
    bullets: Vec<Bullet>,
    last_shoot_time: Instant,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let font = Font::load(engine, "assets/Roboto/Roboto-Regular.ttf")?;
        let texture_cannon = Texture::load(engine, "assets/battery.png")?;
        let texture_bullet = Texture::load(engine, "assets/bullet.png")?;
        let cannons = vec![
            Cannon {
                position: Position::new(32.0 * 2.0, 32.0 * 2.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 5.0, 32.0 * 10.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 8.0, 32.0 * 6.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 6.0, 32.0 * 12.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 17.0, 32.0 * 14.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 4.0, 32.0 * 18.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 18.0, 32.0 * 3.0),
                angle: Angle::zero(),
            },
            Cannon {
                position: Position::new(32.0 * 10.0, 32.0 * 14.0),
                angle: Angle::zero(),
            },
        ];
        Ok(Self {
            font,
            texture_cannon,
            texture_bullet,
            cannons,
            bullets: Vec::new(),
            last_shoot_time: Instant::now(),
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if let Some(mouse_position) = engine.mouse().position() {
            for cannon in &mut self.cannons {
                cannon.look_at(mouse_position);
            }
        }
        for bullet in &mut self.bullets {
            bullet.update(engine);
        }
        self.bullets.retain(|bullet| bullet.is_alive(engine));
        if engine.mouse().is_button_hold(MouseButton::Left) {
            let now = Instant::now();
            if now.duration_since(self.last_shoot_time).as_secs_f32() > BULLET_CD_SECS {
                self.last_shoot_time = now;
                for cannon in &self.cannons {
                    self.bullets.push(Bullet {
                        position: cannon.gun_position_1(),
                        angle: cannon.angle,
                    });
                    self.bullets.push(Bullet {
                        position: cannon.gun_position_2(),
                        angle: cannon.angle,
                    });
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear((0.8, 0.8, 0.8, 1.0));

        for cannon in &self.cannons {
            cannon.draw(engine, &self.texture_cannon);
        }
        for bullet in &self.bullets {
            bullet.draw(engine, &self.texture_bullet);
        }
        engine.graphics().draw_text(
            &self.font,
            &format!("bullets: {}", self.bullets.len()),
            TextDrawParams::default()
                .color(Color::BLUE),
            Transform::default()
                .translate((10.0, 10.0)),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((32.0 * 20.0, 32.0 * 20.0))
            .resizable(false))
        .build()?
        .run_with(App::new)
}
