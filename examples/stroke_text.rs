use tge::prelude::*;

const TITLE: &str = "Stroke Text";

struct App {
    font: Font,
    text: String,
    text_size: f32,
    origin: Position,
    position: Position,
    rotation: Angle,
    scale: Vector,
    color: Color,
    stroke_color: Color,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let font = Font::load(engine, "assets/Roboto/Roboto-Regular.ttf")?;
        let text = "Hello, world!".to_owned();
        let text_size = 40.0;
        let origin = Position::new(80.0, 30.0);
        let position = Position::new(300.0, 300.0);
        let rotation = Angle::zero();
        let scale = Vector::new(2.0, 2.0);
        let color = Color::RED;
        let stroke_color = Color::WHITE;
        Ok(Self {
            font,
            text,
            text_size,
            origin,
            position,
            rotation,
            scale,
            color,
            stroke_color,
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.rotation += Angle::radians(engine.timer().delta_time().as_secs_f32() / 2.0);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x - 1.0, self.position.y))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x - 1.0, self.position.y - 1.0))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x, self.position.y - 1.0))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x + 1.0, self.position.y - 1.0))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x + 1.0, self.position.y))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x + 1.0, self.position.y + 1.0))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x, self.position.y + 1.0))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.stroke_color),
            TransformParams::default()
                .origin(self.origin)
                .position((self.position.x - 1.0, self.position.y + 1.0))
                .rotation(self.rotation)
                .scale(self.scale),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .color(self.color),
            TransformParams::default()
                .origin(self.origin)
                .position(self.position)
                .rotation(self.rotation)
                .scale(self.scale),
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
