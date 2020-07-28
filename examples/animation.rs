use tge::error::GameResult;
use tge::math::{Size, Region};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::game::Game;
use std::time::Duration;

const TITLE: &str = "Animation";

struct App {
    texture: Texture,
    animation_fps: f32,
    total_frame: usize,
    frame_size: Size,
    current_frame: usize,
    current_frame_time: Duration,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let texture = Texture::load(engine, "assets/coin.png")?;
        let texture_size = texture.size();
        let total_frame = 8;
        let frame_size = Size::new(texture_size.width as f32 / total_frame as f32, texture_size.height as f32);
        Ok(Self {
            texture,
            animation_fps: 10.0,
            total_frame,
            frame_size,
            current_frame: 0,
            current_frame_time: Duration::new(0, 0),
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.current_frame_time += engine.timer().delta_time();
        if self.current_frame_time.as_secs_f32() >= 1.0 / self.animation_fps {
            self.current_frame_time = Duration::new(0, 0);
            self.current_frame += 1;
            if self.current_frame >= self.total_frame {
                self.current_frame = 0;
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        let region = Region::new(
            self.frame_size.width * self.current_frame as f32,
            0.0,
            self.frame_size.width,
            self.frame_size.height,
        );
        for j in 0..8 {
            for i in 0..8 {
                engine.graphics().draw_sprite(
                    &self.texture,
                    SpriteDrawParams::default()
                        .position((i as f32 * self.frame_size.width * 2.0, j as f32 * self.frame_size.height * 2.0))
                        .region(region)
                        .scale((2.0, 2.0)),
                );
            }
        }

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((16.0 * 16.0, 16.0 * 16.0))
            .resizable(false))
        .graphics_config(GraphicsConfig::new()
            .default_filter(Filter::new(
                FilterMode::Nearest,
                FilterMode::Nearest,
                None,
            )))
        .build()?
        .run_with(App::new)
}
