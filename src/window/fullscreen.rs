use crate::error::{GameError, GameResult};
use winit::window::Fullscreen;
use winit::monitor::{VideoMode, MonitorHandle};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum FullscreenMode {
    Exclusive,
    Borderless,
}

impl FullscreenMode {

    pub(crate) fn from_raw(fullscreen: Fullscreen) -> Self {
        match fullscreen {
            Fullscreen::Exclusive(_) => FullscreenMode::Exclusive,
            Fullscreen::Borderless(_) => FullscreenMode::Borderless,
        }
    }

    pub(crate) fn into_raw(self, monitor: MonitorHandle) -> GameResult<Fullscreen> {
        match self {
            FullscreenMode::Exclusive => get_preferred_video_mode(monitor)
                .map(|video_mode| Fullscreen::Exclusive(video_mode)),
            FullscreenMode::Borderless => Ok(Fullscreen::Borderless(monitor)),
        }
    }

}

fn get_preferred_video_mode(monitor: MonitorHandle) -> GameResult<VideoMode> {
    let mut preferred_video_mode: Option<VideoMode> = None;
    for (_, video_mode) in monitor.video_modes().enumerate() {
        if let Some(current_video_mode) = &preferred_video_mode {
            let current_size = current_video_mode.size();
            let size = video_mode.size();
            if current_size.width * current_size.height < size.width * size.height {
                preferred_video_mode = Some(video_mode);
            }
        } else {
            preferred_video_mode = Some(video_mode);
        }
    }
    preferred_video_mode.ok_or_else(|| GameError::NotSupportedError("no available video mode".to_owned()))
}
