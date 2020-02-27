use tge::error::GameResult;
use tge::math::Angle;
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::{Color, Texture, SpriteDrawParams};
use tge::game::Game;
use chrono::{Local, Timelike};

struct App {
    clock_disk: Texture,
    hour_angle: Angle,
    minute_angle: Angle,
    second_angle: Angle,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let clock_disk = Texture::load(engine, "assets/clock-disk.png")?;
        Ok(Self {
            clock_disk,
            hour_angle: Angle::zero(),
            minute_angle: Angle::zero(),
            second_angle: Angle::zero(),
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let now_time = Local::now().time();

        let title = format!("{} - FPS: {}", now_time.format("%H:%M:%S"), engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.second_angle = Angle::Degrees(now_time.second() as f32 * 6.0);
        self.minute_angle = Angle::Degrees(now_time.minute() as f32 * 6.0 + now_time.second() as f32 / 10.0);
        self.hour_angle = Angle::Degrees(now_time.hour() as f32 * 30.0 + now_time.minute() as f32 / 2.0 + now_time.second() as f32 / 120.0);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::new(0.9, 0.9, 0.9, 1.0));

        engine.graphics().draw_sprite(
            Some(&self.clock_disk),
            SpriteDrawParams::default(),
        );
        engine.graphics().draw_sprite(
            None,
            SpriteDrawParams::default()
                .source((0.0, 0.0, 30.0, 200.0))
                .origin((15.0, 190.0))
                .position((300.0, 300.0))
                .rotation(self.hour_angle)
                .color(Color::new(0.2, 0.2, 1.0, 1.0)),
        );
        engine.graphics().draw_sprite(
            None,
            SpriteDrawParams::default()
                .source((0.0, 0.0, 20.0, 240.0))
                .origin((10.0, 220.0))
                .position((300.0, 300.0))
                .rotation(self.minute_angle)
                .color(Color::new(0.2, 1.0, 0.2, 1.0)),
        );
        engine.graphics().draw_sprite(
            None,
            SpriteDrawParams::default()
                .source((0.0, 0.0, 10.0, 270.0))
                .origin((5.0, 240.0))
                .position((300.0, 300.0))
                .rotation(self.second_angle)
                .color(Color::new(1.0, 0.2, 0.2, 1.0)),
        );
        engine.graphics().draw_sprite(
            None,
            SpriteDrawParams::default()
                .source((0.0, 0.0, 6.0, 6.0))
                .origin((3.0, 3.0))
                .position((300.0, 300.0))
                .color(Color::BLACK),
        );

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .inner_size((600, 600))
            .resizable(false))
        .build()?
        .run_with(App::new)
}
