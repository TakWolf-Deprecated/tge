use tge::prelude::*;

const TITLE: &str = "Cannon";

struct Cannon {
    position: Position,
    angle: Angle,
}

struct App {
    battery: Texture,
    cannons: Vec<Cannon>,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let battery = Texture::load(engine, "assets/battery.png")?;
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
            battery,
            cannons,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if let Some(mouse_position) = engine.mouse().position() {
            for cannon in &mut self.cannons {
                let x = mouse_position.x - cannon.position.x;
                let y = mouse_position.y - cannon.position.y;
                cannon.angle = Angle::radians(x.atan2(-y));
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear((0.8, 0.8, 0.8, 1.0));

        for cannon in self.cannons.iter() {
            engine.graphics().draw_sprite(
                &self.battery,
                None,
                TransformParams::default()
                    .origin((16.0, 16.0))
                    .position(cannon.position)
                    .rotation(cannon.angle),
            );
        }

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
