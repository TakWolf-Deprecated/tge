use tge::error::GameResult;
use tge::math::Region;
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::game::Game;

const TITLE: &str = "Animation";

struct App {
    character: Texture,
    frames: Vec<Region>,
    current_frame: usize,
    current_frame_time: f32,
    animation_fps: f32,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let character = Texture::load(engine, "assets/pixel-character/pixel-character.png")?;
        let mut sheet = Vec::new();
        let frame_size = 384.0 / 8.0;
        for j in 0..8 {
            for i in 0..8 {
                sheet.push(Region::new(
                    i as f32 * frame_size,
                    j as f32 * frame_size,
                    frame_size,
                    frame_size,
                ));
            }
        }
        let frames = vec![
            *&sheet[24],
            *&sheet[25],
            *&sheet[26],
            *&sheet[27],
            *&sheet[28],
            *&sheet[29],
            *&sheet[30],
            *&sheet[31],
        ];
        Ok(Self {
            character,
            frames,
            current_frame: 0,
            current_frame_time: 0.0,
            animation_fps: 16.0,
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        let delta_time = engine.timer().delta_time().as_secs_f32();
        self.current_frame_time += delta_time;
        if self.current_frame_time >= 1.0 / self.animation_fps {
            self.current_frame_time = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.frames.len() {
                self.current_frame = 0;
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        engine.graphics().draw_sprite(
            Some(&self.character),
            SpriteDrawParams::default()
                .region(*&self.frames[self.current_frame])
                .scale((8.0, 8.0)),
        );

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((384, 384))
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
