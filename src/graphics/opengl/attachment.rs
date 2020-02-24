
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
            Attachment::Color(i) => glow::COLOR_ATTACHMENT0 + *i,
            Attachment::Depth => glow::DEPTH_ATTACHMENT,
            Attachment::Stencil => glow::STENCIL_ATTACHMENT,
            Attachment::DepthStencil => glow::DEPTH_STENCIL_ATTACHMENT,
        }
    }

}
