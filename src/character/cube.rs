use super::{polygon::Polygon, vertex::Vertex};
use crate::client::render::{GLDrawMode, GL};
use crate::math::angle::Rot3Deg;
use crate::math::vec3::Vec3;

pub struct Cube {
    vertices: Vec<Vertex>,
    polygons: Vec<Polygon>,
    x_tex_offs: i32,
    y_tex_offs: i32,
    pub pos: Vec3,
    pub rot: Rot3Deg,
}

impl Cube {
    pub fn new(x_tex_offs: i32, y_tex_offs: i32) -> Cube {
        Cube {
            vertices: vec![],
            polygons: vec![],
            x_tex_offs,
            y_tex_offs,
            pos: Vec3::ZERO,
            rot: Rot3Deg::ZERO,
        }
    }

    pub fn add_box(&mut self, pos: impl Into<Vec3>, w: i32, h: i32, d: i32) {
        let pos = pos.into();
        self.vertices = Vec::with_capacity(8);
        self.polygons = Vec::with_capacity(6);
        let pos1 = pos + (w as f32, h as f32, d as f32);
        let u0 = Vertex::new(pos, 0.0, 0.0);
        let u1 = Vertex::new((pos1.x(), pos.y(), pos.z()), 0.0, 8.0);
        let u2 = Vertex::new((pos1.x(), pos1.y(), pos.z()), 8.0, 8.0);
        let u3 = Vertex::new((pos.x(), pos1.y(), pos.z()), 8.0, 0.0);
        let l0 = Vertex::new((pos.x(), pos.y(), pos1.z()), 0.0, 0.0);
        let l1 = Vertex::new((pos1.x(), pos.y(), pos1.z()), 0.0, 8.0);
        let l2 = Vertex::new(pos1, 8.0, 8.0);
        let l3 = Vertex::new((pos.x(), pos1.y(), pos1.z()), 8.0, 0.0);
        self.vertices.push(u0.clone());
        self.vertices.push(u1.clone());
        self.vertices.push(u2.clone());
        self.vertices.push(u3.clone());
        self.vertices.push(l0.clone());
        self.vertices.push(l1.clone());
        self.vertices.push(l2.clone());
        self.vertices.push(l3.clone());
        self.polygons.push(Polygon::from_uvs(
            vec![l1.clone(), u1.clone(), u2.clone(), l2.clone()],
            self.x_tex_offs + d + w,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w + d,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![u0.clone(), l0.clone(), l3.clone(), u3.clone()],
            self.x_tex_offs + 0,
            self.y_tex_offs + d,
            self.x_tex_offs + d,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![l1.clone(), l0.clone(), u0.clone(), u1.clone()],
            self.x_tex_offs + d,
            self.y_tex_offs + 0,
            self.x_tex_offs + d + w,
            self.y_tex_offs + d,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![u2.clone(), u3.clone(), l3.clone(), l2.clone()],
            self.x_tex_offs + d + w,
            self.y_tex_offs + 0,
            self.x_tex_offs + d + w + w,
            self.y_tex_offs + d,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![u1.clone(), u0.clone(), u3.clone(), u2.clone()],
            self.x_tex_offs + d,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![l0.clone(), l1.clone(), l2.clone(), l3.clone()],
            self.x_tex_offs + d + w + d,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w + d + w,
            self.y_tex_offs + d + h,
        ));
    }

    pub fn set_pos(&mut self, pos: impl Into<Vec3>) {
        let pos = pos.into();
        self.pos = pos;
    }

    pub fn render(&self, gl: &mut GL) {
        gl.push_matrix();
        gl.translate(self.pos);
        gl.rotate(self.rot.z(), Vec3::Z);
        gl.rotate(self.rot.y(), Vec3::Y);
        gl.rotate(self.rot.x(), Vec3::X);
        let mut drawing = gl.begin(GLDrawMode::Quads);
        for polygon in &self.polygons {
            polygon.render(&mut drawing);
        }
        drawing.end();
        gl.pop_matrix();
    }
}