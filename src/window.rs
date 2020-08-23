mod icon;
mod dpi;
mod fullscreen;

pub use icon::Icon;
pub use dpi::{LogicalPosition, PhysicalPosition, LogicalSize, PhysicalSize};
pub use fullscreen::FullscreenMode;

use crate::error::{GameError, GameResult};
use crate::filesystem::Filesystem;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use glow::Context;
use std::rc::Rc;

pub struct Window {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, winit::window::Window>>,
    gl: Rc<Context>,
    title: String,
    resizable: bool,
    maximized: bool,
    transparent: bool,
    decorations: bool,
    always_on_top: bool,
    visible: bool,
    focused: bool,
}

impl Window {
    pub(crate) fn new(window_config: WindowConfig, event_loop: &EventLoop<()>, filesystem: &Filesystem) -> GameResult<Self> {
        let mut window_builder = WindowBuilder::new()
            .with_title(&window_config.title)
            .with_window_icon(match window_config.icon {
                Some(path) => {
                    let bytes = filesystem.read(path)?;
                    let icon = Icon::from_bytes(&bytes)?;
                    Some(icon.into())
                }
                None => None,
            })
            .with_fullscreen(match window_config.fullscreen {
                Some(fullscreen_mode) => {
                    let monitor = event_loop.primary_monitor();
                    let fullscreen = fullscreen_mode.into_raw(monitor)?;
                    Some(fullscreen)
                }
                None => None,
            })
            .with_resizable(window_config.resizable)
            .with_maximized(window_config.maximized)
            .with_transparent(window_config.transparent)
            .with_decorations(window_config.decorations)
            .with_always_on_top(window_config.always_on_top)
            .with_visible(window_config.visible);
        if let Some(size) = window_config.inner_size {
            window_builder = window_builder.with_inner_size(winit::dpi::LogicalSize::new(size.width, size.height))
        }
        if let Some(size) = window_config.min_inner_size {
            window_builder = window_builder.with_min_inner_size(winit::dpi::LogicalSize::new(size.width, size.height));
        }
        if let Some(size) = window_config.max_inner_size {
            window_builder = window_builder.with_max_inner_size(winit::dpi::LogicalSize::new(size.width, size.height));
        }
        let context_builder = ContextBuilder::new()
            .with_vsync(window_config.vsync);
        let windowed_context = context_builder.build_windowed(window_builder, event_loop)
            .map_err(|error| GameError::InitError(Box::new(error)))?;
        let context_wrapper = unsafe {
            windowed_context.make_current()
                .map_err(|(_, error)| GameError::InitError(Box::new(error)))?
        };
        let gl = Context::from_loader_function(|symbol| context_wrapper.get_proc_address(symbol).cast());
        Ok(Self {
            context_wrapper: Rc::new(context_wrapper),
            gl: Rc::new(gl),
            title: window_config.title,
            resizable: window_config.resizable,
            maximized: window_config.maximized,
            transparent: window_config.transparent,
            decorations: window_config.decorations,
            always_on_top: window_config.always_on_top,
            visible: window_config.visible,
            focused: false,
        })
    }

    pub(crate) fn context_wrapper(&self) -> Rc<ContextWrapper<PossiblyCurrent, winit::window::Window>> {
        self.context_wrapper.clone()
    }

    pub(crate) fn gl(&self) -> Rc<Context> {
        self.gl.clone()
    }

    pub(crate) fn window(&self) -> &winit::window::Window {
        self.context_wrapper.window()
    }

    pub(crate) fn handle_focus_change_event(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        self.window().set_title(&self.title);
    }

    pub fn set_icon(&mut self, icon: Option<Icon>) {
        self.window().set_window_icon(icon.map(|icon| icon.into()));
    }

    pub fn inner_size(&self) -> LogicalSize {
        let physical_size = self.window().inner_size();
        let scale_factor = self.window().scale_factor();
        let logical_size = physical_size.to_logical(scale_factor);
        LogicalSize::new(logical_size.width, logical_size.height)
    }

    pub fn set_inner_size(&mut self, size: impl Into<LogicalSize>) {
        let size = size.into();
        self.window().set_inner_size(winit::dpi::LogicalSize::new(size.width, size.height));
    }

    pub fn outer_size(&self) -> LogicalSize {
        let physical_size = self.window().outer_size();
        let scale_factor = self.window().scale_factor();
        let logical_size = physical_size.to_logical(scale_factor);
        LogicalSize::new(logical_size.width, logical_size.height)
    }

    pub fn set_min_inner_size(&mut self, size: Option<impl Into<LogicalSize>>) {
        self.window().set_min_inner_size(size.map(|size| {
            let size = size.into();
            winit::dpi::LogicalSize::new(size.width, size.height)
        }));
    }

    pub fn set_max_inner_size(&mut self, size: Option<impl Into<LogicalSize>>) {
        self.window().set_max_inner_size(size.map(|size| {
            let size = size.into();
            winit::dpi::LogicalSize::new(size.width, size.height)
        }));
    }

    pub fn inner_position(&self) -> GameResult<LogicalPosition> {
        let physical_position = self.window().inner_position()
            .map_err(|error| GameError::NotSupportedError(Box::new(error)))?;
        let scale_factor = self.window().scale_factor();
        let logical_position = physical_position.to_logical(scale_factor);
        Ok(LogicalPosition::new(logical_position.x, logical_position.y))
    }

    pub fn outer_position(&self) -> GameResult<LogicalPosition> {
        let physical_position = self.window().outer_position()
            .map_err(|error| GameError::NotSupportedError(Box::new(error)))?;
        let scale_factor = self.window().scale_factor();
        let logical_position = physical_position.to_logical(scale_factor);
        Ok(LogicalPosition::new(logical_position.x, logical_position.y))
    }

    pub fn set_outer_position(&mut self, position: impl Into<LogicalPosition>) {
        let position = position.into();
        self.window().set_outer_position(winit::dpi::LogicalPosition::new(position.x, position.y));
    }

    pub fn set_ime_position(&mut self, position: impl Into<LogicalPosition>) {
        let position = position.into();
        self.window().set_ime_position(winit::dpi::LogicalPosition::new(position.x, position.y));
    }

    pub fn scale_factor(&self) -> f32 {
        self.window().scale_factor() as f32
    }

    pub fn fullscreen(&self) -> Option<FullscreenMode> {
        self.window().fullscreen()
            .map(|fullscreen| FullscreenMode::from_raw(fullscreen))
    }

    pub fn is_fullscreen(&self) -> bool {
        self.window().fullscreen().is_some()
    }

    pub fn set_fullscreen(&mut self, fullscreen: Option<FullscreenMode>) -> GameResult {
        let fullscreen = match fullscreen {
            Some(fullscreen_mode) => {
                let monitor = self.window().current_monitor();
                Some(fullscreen_mode.into_raw(monitor)?)
            }
            None => None,
        };
        self.window().set_fullscreen(fullscreen);
        Ok(())
    }

    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
        self.window().set_resizable(self.resizable);
    }

    pub fn is_maximized(&self) -> bool {
        self.maximized
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
        self.window().set_maximized(self.maximized);
    }

    pub fn is_transparent(&self) -> bool {
        self.transparent
    }

    pub fn is_decorations(&self) -> bool {
        self.decorations
    }

    pub fn set_decorations(&mut self, decorations: bool) {
        self.decorations = decorations;
        self.window().set_decorations(self.decorations);
    }

    pub fn is_always_on_top(&self) -> bool {
        self.always_on_top
    }

    pub fn set_always_on_top(&mut self, always_on_top: bool) {
        self.always_on_top = always_on_top;
        self.window().set_always_on_top(self.always_on_top);
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.window().set_visible(self.visible);
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    title: String,
    icon: Option<String>,
    inner_size: Option<LogicalSize>,
    min_inner_size: Option<LogicalSize>,
    max_inner_size: Option<LogicalSize>,
    fullscreen: Option<FullscreenMode>,
    resizable: bool,
    maximized: bool,
    transparent: bool,
    decorations: bool,
    always_on_top: bool,
    visible: bool,
    vsync: bool,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self {
            title: "tge".to_owned(),
            icon: None,
            inner_size: None,
            min_inner_size: None,
            max_inner_size: None,
            fullscreen: None,
            resizable: true,
            maximized: false,
            transparent: false,
            decorations: true,
            always_on_top: false,
            visible: true,
            vsync: false,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn icon(mut self, path: Option<impl Into<String>>) -> Self {
        self.icon = path.map(|path| path.into());
        self
    }

    pub fn inner_size(mut self, size: impl Into<LogicalSize>) -> Self {
        self.inner_size = Some(size.into());
        self
    }

    pub fn min_inner_size(mut self, size: Option<impl Into<LogicalSize>>) -> Self {
        self.min_inner_size = size.map(|size| size.into());
        self
    }

    pub fn max_inner_size(mut self, size: Option<impl Into<LogicalSize>>) -> Self {
        self.max_inner_size = size.map(|size| size.into());
        self
    }

    pub fn fullscreen(mut self, fullscreen: Option<FullscreenMode>) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    pub fn decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }
}
