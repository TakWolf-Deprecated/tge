use tge::prelude::*;
use std::time::Duration;

const TITLE: &str = "Frame Animation";

struct FrameAnimation {
    fps: f32,
    texture_region: Region,
    split_size: Size<usize>,
    current: usize,
    since_last_frame: Duration,
}

impl FrameAnimation {
    fn new(fps: f32, texture_region: impl Into<Region>, split_size: impl Into<Size<usize>>) -> Self {
        Self {
            fps,
            texture_region: texture_region.into(),
            split_size: split_size.into(),
            current: 0,
            since_last_frame: Duration::new(0, 0),
        }
    }

    fn update(&mut self, engine: &mut Engine) {
        self.since_last_frame += engine.timer().delta_time();
        if self.since_last_frame.as_secs_f32() >= 1.0 / self.fps {
            self.since_last_frame = Duration::new(0, 0);
            self.current += 1;
            if self.current >= self.split_size.width * self.split_size.height {
                self.current = 0;
            }
        }
    }

    fn draw(&self, engine: &mut Engine, texture: &Texture, transform: impl Into<Option<Transform>>) {
        let size = Size::new(
            self.texture_region.width / self.split_size.width as f32,
            self.texture_region.height / self.split_size.height as f32,
        );
        let position = Position::new(
            (self.current % self.split_size.width) as f32 * size.width + self.texture_region.x,
            (self.current / self.split_size.width) as f32 * size.height + self.texture_region.y,
        );
        engine.graphics().draw_sprite(
            texture,
            SpriteDrawParams::default()
                .region(Region::position_size(position, size)),
            transform,
        );
    }
}

struct App {
    texture_coin: Texture,
    texture_characters: Texture,
    animation_coin: FrameAnimation,
    animation_role_1: FrameAnimation,
    animation_role_2: FrameAnimation,
    animation_role_3: FrameAnimation,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let texture_coin = Texture::load(engine, "assets/coin.png")?;
        let texture_characters = Texture::load(engine, "assets/characters.png")?;
        let animation_coin = FrameAnimation::new(10.0, (0.0, 0.0, 128.0, 16.0), (8, 1));
        let animation_role_1 = FrameAnimation::new(6.0, (0.0, 0.0, 128.0, 32.0), (4, 1));
        let animation_role_2 = FrameAnimation::new( 8.0, (0.0, 32.0, 128.0, 32.0), (4, 1));
        let animation_role_3 = FrameAnimation::new(12.0, (0.0, 64.0, 128.0, 32.0), (4, 1));
        Ok(Self {
            texture_coin,
            texture_characters,
            animation_coin,
            animation_role_1,
            animation_role_2,
            animation_role_3,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.animation_coin.update(engine);
        self.animation_role_1.update(engine);
        self.animation_role_2.update(engine);
        self.animation_role_3.update(engine);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        self.animation_coin.draw(
            engine,
            &self.texture_coin,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 32.0)),
        );
        self.animation_role_1.draw(
            engine,
            &self.texture_characters,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 96.0)),
        );
        self.animation_role_2.draw(
            engine,
            &self.texture_characters,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 224.0)),
        );
        self.animation_role_3.draw(
            engine,
            &self.texture_characters,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 352.0)),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800.0, 600.0)))
        .build()?
        .run_with(App::new)
}
