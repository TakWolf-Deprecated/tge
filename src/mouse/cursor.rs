
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl Default for CursorIcon {
    
    fn default() -> Self {
        CursorIcon::Default
    }
    
}

impl Into<winit::window::CursorIcon> for CursorIcon {
    
    fn into(self) -> winit::window::CursorIcon {
        match self {
            CursorIcon::Default => winit::window::CursorIcon::Default,
            CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
            CursorIcon::Hand => winit::window::CursorIcon::Hand,
            CursorIcon::Arrow => winit::window::CursorIcon::Arrow,
            CursorIcon::Move => winit::window::CursorIcon::Move,
            CursorIcon::Text => winit::window::CursorIcon::Text,
            CursorIcon::Wait => winit::window::CursorIcon::Wait,
            CursorIcon::Help => winit::window::CursorIcon::Help,
            CursorIcon::Progress => winit::window::CursorIcon::Progress,
            CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
            CursorIcon::ContextMenu => winit::window::CursorIcon::ContextMenu,
            CursorIcon::Cell => winit::window::CursorIcon::Cell,
            CursorIcon::VerticalText => winit::window::CursorIcon::VerticalText,
            CursorIcon::Alias => winit::window::CursorIcon::Alias,
            CursorIcon::Copy => winit::window::CursorIcon::Copy,
            CursorIcon::NoDrop => winit::window::CursorIcon::NoDrop,
            CursorIcon::Grab => winit::window::CursorIcon::Grab,
            CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
            CursorIcon::AllScroll => winit::window::CursorIcon::AllScroll,
            CursorIcon::ZoomIn => winit::window::CursorIcon::ZoomIn,
            CursorIcon::ZoomOut => winit::window::CursorIcon::ZoomOut,
            CursorIcon::EResize => winit::window::CursorIcon::EResize,
            CursorIcon::NResize => winit::window::CursorIcon::NResize,
            CursorIcon::NeResize => winit::window::CursorIcon::NeResize,
            CursorIcon::NwResize => winit::window::CursorIcon::NwResize,
            CursorIcon::SResize => winit::window::CursorIcon::SResize,
            CursorIcon::SeResize => winit::window::CursorIcon::SeResize,
            CursorIcon::SwResize => winit::window::CursorIcon::SwResize,
            CursorIcon::WResize => winit::window::CursorIcon::WResize,
            CursorIcon::EwResize => winit::window::CursorIcon::EwResize,
            CursorIcon::NsResize => winit::window::CursorIcon::NsResize,
            CursorIcon::NeswResize => winit::window::CursorIcon::NeswResize,
            CursorIcon::NwseResize => winit::window::CursorIcon::NwseResize,
            CursorIcon::ColResize => winit::window::CursorIcon::ColResize,
            CursorIcon::RowResize => winit::window::CursorIcon::RowResize,
        }
    }
    
}
