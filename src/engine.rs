use crate::error::{GameError, GameResult};
use crate::window::{Window, WindowConfig};
use crate::graphics::{Graphics, GraphicsConfig};
use crate::timer::{Timer, TimerConfig};
use crate::keyboard::{Keyboard, KeyboardConfig};
use crate::mouse::{Mouse, MouseConfig};
use crate::gamepad::{Gamepad, GamepadConfig};
use crate::audio::{Audio, AudioConfig};
use crate::game::Game;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::platform::desktop::EventLoopExtDesktop;

#[derive(Debug)]
enum State {
    Ready,
    Running,
    Finished,
    Broken(Option<GameError>),
}

pub struct Engine {
    window: Window,
    graphics: Graphics,
    timer: Timer,
    keyboard: Keyboard,
    mouse: Mouse,
    gamepad: Gamepad,
    audio: Audio,
    event_loop: Option<EventLoop<()>>,
    state: State,
}

impl Engine {

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn graphics(&mut self) -> &mut Graphics {
        &mut self.graphics
    }

    pub fn timer(&mut self) -> &mut Timer {
        &mut self.timer
    }

    pub fn keyboard(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    pub fn mouse(&mut self) -> &mut Mouse {
        &mut self.mouse
    }

    pub fn gamepad(&mut self) -> &mut Gamepad {
        &mut self.gamepad
    }

    pub fn audio(&mut self) -> &mut Audio {
        &mut self.audio
    }

    pub fn run(&mut self, game: &mut dyn Game) -> GameResult {
        match &self.state {
            State::Ready => self.state = State::Running,
            _ => return Err(GameError::StateError(format!("engine can not be run on state {:?}", self.state))),
        }

        self.timer.reset_tick();

        let mut event_loop = self.event_loop.take().expect("no event_loop instance");
        event_loop.run_return(|event, window_target, control_flow| {
            match &self.state {
                State::Finished | State::Broken(_) => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                State::Running => (),
                _ => {
                    self.state = State::Broken(Some(GameError::StateError(format!("engine state {:?} incorrect at the beginning of the event loop one step", self.state))));
                    return;
                }
            }

            match event {
                winit::event::Event::LoopDestroyed => {
                    match &self.state {
                        State::Finished | State::Broken(_) => (),
                        State::Running => self.state = State::Finished,
                        _ => self.state = State::Broken(Some(GameError::StateError(format!("engine state {:?} incorrect on event loop event {:?}", self.state, event)))),
                    }
                    return;
                }
                winit::event::Event::WindowEvent { window_id, event } => {
                    if self.window.window().id() == window_id {
                        match event {
                            // TODO
                            _ => (),
                        }
                    }
                }
                winit::event::Event::RedrawEventsCleared => {
                    if self.timer.tick_and_check() {
                        match game.update(self) {
                            Ok(()) => (),
                            Err(error) => {
                                self.state = State::Broken(Some(error));
                                return;
                            }
                        }
                        match game.render(self) {
                            Ok(()) => (),
                            Err(error) => {
                                self.state = State::Broken(Some(error));
                                return;
                            }
                        }
                        match self.graphics.present() {
                            Ok(()) => (),
                            Err(error) => {
                                self.state = State::Broken(Some(error));
                                return;
                            }
                        }
                    }
                }
                _ => (),
            }
        });
        self.event_loop = Some(event_loop);

        match &mut self.state {
            State::Finished => Ok(()),
            State::Broken(error) => Err(error.take().expect("no engine broken error instance")),
            _ => Err(GameError::StateError(format!("engine state {:?} incorrect at the end of the event loop", self.state))),
        }
    }

    pub fn run_with<G, F>(&mut self, init: F) -> GameResult
        where
            G: Game,
            F: FnOnce(&mut Self) -> GameResult<G>,
    {
        let mut game = init(self)?;
        self.run(&mut game)
    }

    pub fn quit(&mut self) {
        self.state = State::Finished;
    }

    pub fn exit(&mut self, error: GameError) {
        self.state = State::Broken(Some(error));
    }

}

#[derive(Debug, Clone)]
pub struct EngineBuilder {
    window_config: Option<WindowConfig>,
    graphics_config: Option<GraphicsConfig>,
    timer_config: Option<TimerConfig>,
    keyboard_config: Option<KeyboardConfig>,
    mouse_config: Option<MouseConfig>,
    gamepad_config: Option<GamepadConfig>,
    audio_config: Option<AudioConfig>,
}

impl EngineBuilder {

    pub fn new() -> Self {
        Self {
            window_config: None,
            graphics_config: None,
            timer_config: None,
            keyboard_config: None,
            mouse_config: None,
            gamepad_config: None,
            audio_config: None,
        }
    }

    pub fn window_config(mut self, window_config: WindowConfig) -> Self {
        self.window_config = Some(window_config);
        self
    }

    pub fn graphics_config(mut self, graphics_config: GraphicsConfig) -> Self {
        self.graphics_config = Some(graphics_config);
        self
    }

    pub fn timer_config(mut self, timer_config: TimerConfig) -> Self {
        self.timer_config = Some(timer_config);
        self
    }

    pub fn keyboard_config(mut self, keyboard_config: KeyboardConfig) -> Self {
        self.keyboard_config = Some(keyboard_config);
        self
    }

    pub fn mouse_config(mut self, mouse_config: MouseConfig) -> Self {
        self.mouse_config = Some(mouse_config);
        self
    }

    pub fn gamepad_config(mut self, gamepad_config: GamepadConfig) -> Self {
        self.gamepad_config = Some(gamepad_config);
        self
    }

    pub fn audio_config(mut self, audio_config: AudioConfig) -> Self {
        self.audio_config = Some(audio_config);
        self
    }

    pub fn build(self) -> GameResult<Engine> {
        let window_config = self.window_config.unwrap_or_else(|| WindowConfig::new());
        let graphics_config = self.graphics_config.unwrap_or_else(|| GraphicsConfig::new());
        let timer_config = self.timer_config.unwrap_or_else(|| TimerConfig::new());
        let keyboard_config = self.keyboard_config.unwrap_or_else(|| KeyboardConfig::new());
        let mouse_config = self.mouse_config.unwrap_or_else(|| MouseConfig::new());
        let gamepad_config = self.gamepad_config.unwrap_or_else(|| GamepadConfig::new());
        let audio_config = self.audio_config.unwrap_or_else(|| AudioConfig::new());

        let event_loop = EventLoop::new();

        let window = Window::new(window_config, &event_loop)?;
        let graphics = Graphics::new(graphics_config, window.context_wrapper())?;
        let timer = Timer::new(timer_config)?;
        let keyboard = Keyboard::new()?;
        let mouse = Mouse::new()?;
        let gamepad = Gamepad::new()?;
        let audio = Audio::new()?;

        Ok(Engine {
            window,
            graphics,
            timer,
            keyboard,
            mouse,
            gamepad,
            audio,
            event_loop: Some(event_loop),
            state: State::Ready,
        })
    }

}
