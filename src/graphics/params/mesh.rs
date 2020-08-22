use super::{PrimitiveType, Vertex};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MeshDrawParams {
    pub primitive: Option<PrimitiveType>,
    pub vertices: Option<Vec<Vertex>>,
    pub elements: Option<Vec<u16>>,
}

impl MeshDrawParams {
    pub fn primitive(mut self, primitive: PrimitiveType) -> Self {
        self.primitive = Some(primitive);
        self
    }

    pub fn vertices(mut self, vertices: impl Into<Vec<Vertex>>) -> Self {
        self.vertices = Some(vertices.into());
        self
    }

    pub fn elements(mut self, elements: impl Into<Option<Vec<u16>>>) -> Self {
        self.elements = elements.into();
        self
    }
}
