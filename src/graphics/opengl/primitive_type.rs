
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PrimitiveType {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl PrimitiveType {

    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Points => glow::POINTS,
            Self::LineStrip => glow::LINE_STRIP,
            Self::LineLoop => glow::LINE_LOOP,
            Self::Lines => glow::LINES,
            Self::LineStripAdjacency => glow::LINE_STRIP_ADJACENCY,
            Self::LinesAdjacency => glow::LINES_ADJACENCY,
            Self::TriangleStrip => glow::TRIANGLE_STRIP,
            Self::TriangleFan => glow::TRIANGLE_FAN,
            Self::Triangles => glow::TRIANGLES,
            Self::TriangleStripAdjacency => glow::TRIANGLE_STRIP_ADJACENCY,
            Self::TrianglesAdjacency => glow::TRIANGLES_ADJACENCY,
            Self::Patches => glow::PATCHES,
        }
    }

}
