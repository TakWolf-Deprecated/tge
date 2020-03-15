# TakWolf's Game Engine (tge)

[![Crates.io](https://img.shields.io/crates/v/tge)](https://crates.io/crates/tge)
[![Docs.rs](https://docs.rs/tge/badge.svg)](https://docs.rs/tge)
[![License](https://img.shields.io/crates/l/tge)](#License)

A lightweight cross-platform 2D game framework written in pure Rust and based on OpenGL 3.3+.

__Tge is currently in a very early stage of development. The API may be changed. Until the version to `0.1.0`.__

## Features

* Modular API like [LÖVE](https://love2d.org).
* Vertex and sprite rendering use hardware-accelerated.
* Font support and text rendering. (TODO)
* Interface for handling keyboard, mouse, touch, touchpad and gamepad.
* Audio play. (TODO)

## Non goals

Tge is a lightweight 2D game framework.
So it doesn't support:

* 3D. (But can be expanded in theory.)
* Visual editor.

Some features depends on the specific formats.
They can be easily implemented.
So tge doesn't include:

* Animation engine.
* Particle system.
* Tile map.

Some features depends on the specific programming paradigms.
You can design them by yourself.
So tge doesn't include:

* Scene manager.

Tge can easily work with other crates.
So tge doesn't include:

* [Entity Component System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system).
* Physics engines and collision detection.
* Network.

## Usage

Just add the dependency line to your `Cargo.toml` file:

```toml
[dependencies]
tge = "0.0.1"
```

## Examples

Here is the minimal example that will create a window:

```rust
use tge::error::GameResult;
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::Color;
use tge::game::Game;

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
            .inner_size((800, 600)))
        .build()?;
    let mut app = App::new(&mut engine)?;
    engine.run(&mut app)
}
```

Execute `cargo run --example hello_world` to run this example.
You can also browse the [`examples/`](examples/) directory to learn more.

More complete demos can be found in [tge-demos](https://github.com/TakWolf/tge-demos).

## Performance

Cargo builds projects in debug mode by default.
This may cause the program running slowly.

Add following to your `Cargo.toml` file to release performance:

```toml
[profile.dev]
opt-level = 3
```

You can also use the `--release` flag when building your project to enable release mode.
Please note that release mode will increase build times quite significantly and remove debug info from the binary.

Run the example [`bunny_mark`](examples/bunny_mark.rs) and [`sprites`](examples/sprites.rs) 
both with and without the `--release` flag to observe the impact of compiler optimizations.

## TODO

Working in progress:
* blend
* program uniform
* font and text
* texture to image and screenshot
* graphics transform matrix
* virtual assets path
* assets load async
* audio
* document

## Others

* Issue and PR are welcome.
* Third-party extend crates are welcome, but please do not name with prefix `tge-`.

## License

[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)
