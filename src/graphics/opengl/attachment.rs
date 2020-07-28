
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Attachment {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl Attachment {

    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Color(i) => glow::COLOR_ATTACHMENT0 + *i,
            Self::Depth => glow::DEPTH_ATTACHMENT,
            Self::Stencil => glow::STENCIL_ATTACHMENT,
            Self::DepthStencil => glow::DEPTH_STENCIL_ATTACHMENT,
        }
    }

}
