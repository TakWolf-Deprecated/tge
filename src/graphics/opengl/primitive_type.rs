
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
            PrimitiveType::Points => glow::POINTS,
            PrimitiveType::LineStrip => glow::LINE_STRIP,
            PrimitiveType::LineLoop => glow::LINE_LOOP,
            PrimitiveType::Lines => glow::LINES,
            PrimitiveType::LineStripAdjacency => glow::LINE_STRIP_ADJACENCY,
            PrimitiveType::LinesAdjacency => glow::LINES_ADJACENCY,
            PrimitiveType::TriangleStrip => glow::TRIANGLE_STRIP,
            PrimitiveType::TriangleFan => glow::TRIANGLE_FAN,
            PrimitiveType::Triangles => glow::TRIANGLES,
            PrimitiveType::TriangleStripAdjacency => glow::TRIANGLE_STRIP_ADJACENCY,
            PrimitiveType::TrianglesAdjacency => glow::TRIANGLES_ADJACENCY,
            PrimitiveType::Patches => glow::PATCHES,
        }
    }

}
