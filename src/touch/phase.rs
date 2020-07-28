
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TouchPhase {
    Start,
    Move,
    End,
    Cancel,
}

impl From<winit::event::TouchPhase> for TouchPhase {

    fn from(phase: winit::event::TouchPhase) -> Self {
        match phase {
            winit::event::TouchPhase::Started => Self::Start,
            winit::event::TouchPhase::Moved => Self::Move,
            winit::event::TouchPhase::Ended => Self::End,
            winit::event::TouchPhase::Cancelled => Self::Cancel,
        }
    }

}
