use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub u: f32,
    pub v: f32,
}

impl Vertex {
    pub fn new(pos: impl Into<Vec3>, u: f32, v: f32) -> Vertex {
        let pos = pos.into();
        Vertex {
            pos,
            u,
            v,
        }
    }

    pub fn from_vertex(vertex: &Vertex, u: f32, v: f32) -> Vertex {
        Vertex {
            pos: vertex.pos,
            u,
            v,
        }
    }

    pub fn remap(&self, u: f32, v: f32) -> Vertex {
        Vertex::from_vertex(self, u, v)
    }
}