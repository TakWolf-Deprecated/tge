
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
            winit::event::TouchPhase::Started => TouchPhase::Start,
            winit::event::TouchPhase::Moved => TouchPhase::Move,
            winit::event::TouchPhase::Ended => TouchPhase::End,
            winit::event::TouchPhase::Cancelled => TouchPhase::Cancel,
        }
    }

}
