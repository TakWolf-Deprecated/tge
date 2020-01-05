mod icon;
mod fullscreen;

pub use fullscreen::FullscreenMode;

use crate::error::{GameError, GameResult};
use crate::math::{Position, Size};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use std::rc::Rc;
use std::path::Path;

pub struct Window {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, winit::window::Window>>,
    title: String,
    resizable: bool,
    maximized: bool,
    transparent: bool,
    decorations: bool,
    always_on_top: bool,
    visible: bool,
}

impl Window {

    pub(crate) fn new(window_config: WindowConfig, event_loop: &EventLoop<()>) -> GameResult<Self> {
        let mut window_builder = WindowBuilder::new()
            .with_title(&window_config.title)
            .with_resizable(window_config.resizable)
            .with_maximized(window_config.maximized)
            .with_transparent(window_config.transparent)
            .with_decorations(window_config.decorations)
            .with_always_on_top(window_config.always_on_top)
            .with_visible(window_config.visible);
        if let Some(path) = window_config.icon {
            let icon = icon::load_icon(path)?;
            window_builder = window_builder.with_window_icon(Some(icon));
        }
        if let Some(size) = window_config.inner_size {
            window_builder = window_builder.with_inner_size(size.into())
        }
        if let Some(size) = window_config.min_inner_size {
            window_builder = window_builder.with_min_inner_size(size.into());
        }
        if let Some(size) = window_config.max_inner_size {
            window_builder = window_builder.with_max_inner_size(size.into());
        }
        if let Some(fullscreen_mode) = window_config.fullscreen {
            let fullscreen = fullscreen_mode.to_winit_enum(event_loop.primary_monitor())?;
            window_builder = window_builder.with_fullscreen(Some(fullscreen));
        }
        let windowed_context = ContextBuilder::new()
            .with_vsync(window_config.vsync)
            .build_windowed(window_builder, event_loop)
            .map_err(|error| GameError::InitError(format!("{}", error)))?;
        let context_wrapper = unsafe {
            windowed_context.make_current().map_err(|(_, error)| GameError::InitError(format!("{}", error)))?
        };
        gl::load_with(|symbol| context_wrapper.context().get_proc_address(symbol).cast());
        Ok(Self {
            context_wrapper: Rc::new(context_wrapper),
            title: window_config.title,
            resizable: window_config.resizable,
            maximized: window_config.maximized,
            transparent: window_config.transparent,
            decorations: window_config.decorations,
            always_on_top: window_config.always_on_top,
            visible: window_config.visible,
        })
    }

    pub(crate) fn context_wrapper(&self) -> Rc<ContextWrapper<PossiblyCurrent, winit::window::Window>> {
        self.context_wrapper.clone()
    }

    pub(crate) fn window(&self) -> &winit::window::Window {
        self.context_wrapper.window()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.title = title.into();
        self.window().set_title(&self.title)
    }

    pub fn set_icon<P: AsRef<Path>>(&mut self, path: Option<P>) -> GameResult {
        let icon = match path {
            Some(path) => Some(icon::load_icon(path)?),
            None => None,
        };
        self.window().set_window_icon(icon);
        Ok(())
    }

    pub fn inner_size(&self) -> Size<u32> {
        self.window().inner_size().into()
    }

    pub fn set_inner_size<S: Into<Size<u32>>>(&mut self, size: S) {
        self.window().set_inner_size(size.into().into());
    }

    pub fn outer_size(&self) -> Size<u32> {
        self.window().outer_size().into()
    }

    pub fn set_min_inner_size<S: Into<Size<u32>>>(&mut self, size: Option<S>) {
        self.window().set_min_inner_size(size.map(|size| size.into().into()));
    }

    pub fn set_max_inner_size<S: Into<Size<u32>>>(&mut self, size: Option<S>) {
        self.window().set_max_inner_size(size.map(|size| size.into().into()));
    }

    pub fn inner_position(&self) -> GameResult<Position<i32>> {
        self.window().inner_position()
            .map(|position| position.into())
            .map_err(|error| GameError::NotSupportedError(format!("{}", error)))
    }

    pub fn outer_position(&self) -> GameResult<Position<i32>> {
        self.window().outer_position()
            .map(|position| position.into())
            .map_err(|error| GameError::NotSupportedError(format!("{}", error)))
    }

    pub fn set_outer_position<P: Into<Position<i32>>>(&mut self, position: P) {
        self.window().set_outer_position(position.into().into())
    }

    pub fn set_ime_position<P: Into<Position<i32>>>(&mut self, position: P) {
        self.window().set_ime_position(position.into().into())
    }

    pub fn fullscreen(&self) -> Option<FullscreenMode> {
        self.window().fullscreen().map(|fullscreen| FullscreenMode::from_winit_enum(fullscreen))
    }

    pub fn is_fullscreen(&self) -> bool {
        self.window().fullscreen().is_some()
    }

    pub fn set_fullscreen(&mut self, fullscreen: Option<FullscreenMode>) -> GameResult {
        let fullscreen = match fullscreen {
            Some(fullscreen_mode) => Some(fullscreen_mode.to_winit_enum(self.window().current_monitor())?),
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

}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    title: String,
    icon: Option<String>,
    inner_size: Option<Size<u32>>,
    min_inner_size: Option<Size<u32>>,
    max_inner_size: Option<Size<u32>>,
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

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn icon<P: Into<String>>(mut self, path: Option<P>) -> Self {
        self.icon = path.map(|path| path.into());
        self
    }

    pub fn inner_size<S: Into<Size<u32>>>(mut self, size: S) -> Self {
        self.inner_size = Some(size.into());
        self
    }

    pub fn min_inner_size<S: Into<Size<u32>>>(mut self, size: Option<S>) -> Self {
        self.min_inner_size = size.map(|size| size.into());
        self
    }

    pub fn max_inner_size<S: Into<Size<u32>>>(mut self, size: Option<S>) -> Self {
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
