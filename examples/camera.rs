use tge::prelude::*;

const TITLE: &str = "Camera";

struct Car {
    position: Position,
    speed: f32,
    max_speed: f32,
    speed_acceleration: f32,
    friction_acceleration: f32,
    angle: Angle,
}

impl Car {
    fn new(position: Position) -> Self {
        Self {
            position,
            speed: 0.0,
            max_speed: 6.0,
            speed_acceleration: 0.3,
            friction_acceleration: 0.08,
            angle: Angle::zero(),
        }
    }

    fn update(&mut self, engine: &mut Engine) {
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
        if engine.keyboard().is_key_hold(KeyCode::Up) || engine.keyboard().is_key_hold(KeyCode::W) {
            if self.speed < self.max_speed {
                self.speed += self.speed_acceleration
            }
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        }
        if engine.keyboard().is_key_hold(KeyCode::Down) || engine.keyboard().is_key_hold(KeyCode::S) {
            if self.speed > -self.max_speed {
                self.speed -= self.speed_acceleration
            }
            if self.speed < -self.max_speed {
                self.speed = -self.max_speed;
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
    }

    fn draw(&self, engine: &mut Engine, texture: &Texture) {
        let size = texture.size();
        engine.graphics().draw_sprite(
            texture,
            SpriteDrawParams::default()
                .origin((size.width as f32 / 2.0, size.height as f32 / 2.0)),
            Transform::default()
                .scale((0.16, 0.16))
                .rotate(self.angle)
                .translate(self.position),
        );
    }

    fn draw_info(&self, engine: &mut Engine, font: &Font) {
        let text = format!(
            "position: ({:.1}, {:.1})\nspeed: {:.1}\nangle: degrees({:+.1})",
            self.position.x,
            self.position.y,
            self.speed,
            self.angle.degrees_value(),
        );
        engine.graphics().draw_sprite(
            TextureHolder::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 200.0, 62.0))
                .color(Color::new(0.0, 0.0, 0.0, 0.8)),
            None,
        );
        engine.graphics().draw_text(
            font,
            &text,
            TextDrawParams::default()
                .wrap_width(180.0)
                .wrap_height(42.0)
                .vertical_gravity(TextLayoutGravity::Center),
            Transform::default()
                .translate((10.0, 10.0)),
        );
    }
}

struct App {
    font: Font,
    texture_car: Texture,
    car: Car,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let font = Font::load(engine, "assets/Roboto/Roboto-Regular.ttf")?;
        let texture_car = Texture::load(engine, "assets/car.png")?;
        let car = Car::new(Position::new(5050.0, 5050.0));
        Ok(Self {
            font,
            texture_car,
            car,
        })
    }

    fn set_camera_look_at_car(&mut self, engine: &mut Engine, rotate_camera: bool) {
        let viewport_size = engine.graphics().viewport().size();
        let camera_transform = if rotate_camera {
            Transform::default()
                .translate((-self.car.position.x, -self.car.position.y))
                .rotate(Angle::radians(-self.car.angle.radians_value()))
                .translate((self.car.position.x, self.car.position.y))
        } else {
            Transform::default()
        }.translate((-self.car.position.x + viewport_size.width / 2.0, -self.car.position.y + viewport_size.height / 2.0));
        engine.graphics().set_transform(camera_transform);
    }

    fn draw_coordinate(&mut self, engine: &mut Engine) {
        let viewport_size = engine.graphics().viewport().size();
        let max_distance = viewport_size.width.max(viewport_size.height);
        for x in 0..100 as i32 {
            for y in 0..100 as i32 {
                let position = Position::new(x as f32 * 100.0, y as f32 * 100.0);
                if (position.x + 50.0 - self.car.position.x).abs() <= max_distance && (position.y + 50.0 - self.car.position.y).abs() <= max_distance {
                    engine.graphics().draw_sprite(
                        TextureHolder::None,
                        SpriteDrawParams::default()
                            .region((0.0, 0.0, 100.0, 100.0))
                            .color(if (x + y) % 2 == 0 {
                                Color::new(0.0, 0.0, 0.0, 0.5)
                            } else {
                                Color::new(1.0, 1.0, 1.0, 0.5)
                            }),
                        Transform::default()
                            .translate(position),
                    );
                }
            }
        }
        for x in 0..100 as i32 {
            for y in 0..100 as i32 {
                let position = Position::new(x as f32 * 100.0, y as f32 * 100.0);
                if (position.x + 50.0 - self.car.position.x).abs() <= max_distance && (position.y + 50.0 - self.car.position.y).abs() <= max_distance {
                    engine.graphics().draw_text(
                        &self.font,
                        &format!("{}, {}", x, y),
                        TextDrawParams::default()
                            .wrap_width(100.0)
                            .wrap_height(100.0)
                            .horizontal_gravity(TextLayoutGravity::Center)
                            .vertical_gravity(TextLayoutGravity::Center)
                            .color(Color::YELLOW),
                        Transform::default()
                            .translate(position),
                    );
                }
            }
        }
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.car.update(engine);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear((0.6, 0.6, 0.6, 1.0));

        let graphics_size = engine.graphics().size();

        engine.graphics().push_transform();

        engine.graphics().set_viewport(Some((0.0, 0.0, graphics_size.width / 2.0, graphics_size.height)));
        self.set_camera_look_at_car(engine, false);
        self.draw_coordinate(engine);
        self.car.draw(engine, &self.texture_car);

        engine.graphics().set_viewport(Some((graphics_size.width / 2.0, 0.0, graphics_size.width / 2.0, graphics_size.height)));
        self.set_camera_look_at_car(engine, true);
        self.draw_coordinate(engine);
        self.car.draw(engine, &self.texture_car);

        engine.graphics().pop_transform();

        engine.graphics().set_viewport(Viewport::none());
        engine.graphics().draw_sprite(
            TextureHolder::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 2.0, graphics_size.height))
                .color(Color::BLUE),
            Transform::default()
                .translate((graphics_size.width / 2.0 - 0.1, 0.0)),
        );
        self.car.draw_info(engine, &self.font);

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((1280.0, 640.0)))
        .build()?
        .run_with(App::new)
}
