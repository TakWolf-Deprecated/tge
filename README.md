# TakWolf's Game Engine (tge)

[![Crates.io](https://img.shields.io/crates/v/tge)](https://crates.io/crates/tge)
[![Docs.rs](https://docs.rs/tge/badge.svg)](https://docs.rs/tge)
[![License](https://img.shields.io/crates/l/tge)](#License)

A lightweight cross-platform 2D game framework written in pure Rust and based on OpenGL 3.3+.

Inspired by [LÖVE](https://love2d.org).

__Tge is currently in a very early stage of development. The API may be changed. Until the version to `0.1.0`.__

## Features

* 2D only and use pixel unit.
* Hardware-accelerated rendering base on OpenGL.
* Automatically process rendering batch.
* Dynamic font rendering with text layout.
* Support high-DPI.
* Keyboard, mouse, touch, touchpad and gamepad input handling.
* Audio play. (TODO)

## Non goals

* 3D.
* Visual editor.

The following does not contain, but can easily work with other crates:

* Entity Component System (ECS).
* Physics engines and collision detection.
* Network.

## Usage

Add the dependency line to your `Cargo.toml` file:

```toml
[dependencies]
tge = "0.0.4"
```

To release performance, also add the following configs:

```toml
[profile.dev]
opt-level = 3
```

Then create a basic template. Here is the minimal example that will create a window:

```rust
use tge::prelude::*;

struct App {}

impl App {
    fn new(_: &mut Engine) -> GameResult<Self> {
        // load assets
        Ok(Self {})
    }
}

impl Game for App {
    fn update(&mut self, _: &mut Engine) -> GameResult {
        // handle logic
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLUE);
        // draw sprites
        Ok(())
    }
}

fn main() -> GameResult {
    let mut engine = EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title("My Game")
            .inner_size((1024.0, 600.0)))
        .build()?;
    let mut app = App::new(&mut engine)?;
    engine.run(&mut app)
}
```

That is!

You can also see the [`examples/`](examples/) directory to learn other examples.

## Performance

See the example [`bunny_mark`](examples/bunny_mark.rs) and [`hare_mark`](examples/hare_mark.rs).

## TODO

The following is working in progress:

* blend
* program uniform
* screenshot
* virtual assets path
* assets load async
* audio
* document

## License

[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)
