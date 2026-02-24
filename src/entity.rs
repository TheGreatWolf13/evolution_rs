use crate::client::Engine;
use crate::math::angle::{Angle, Rot3Deg};
use crate::math::vec3::Vec3;
use crate::{level::level::Level, phys::aabb::AABB};
use std::{cell::RefCell, rc::Rc};

pub struct Entity {
    level: Rc<RefCell<Level>>,
    pub ro: Vec3,
    pub r: Vec3,
    pub rd: Vec3,
    pub rot: Rot3Deg,
    pub bb: AABB,
    pub on_ground: bool,
    pub height_offset: f32,
}

impl Entity {
    pub fn new(level: Rc<RefCell<Level>>) -> Entity {
        let mut e = Entity {
            level,
            ro: Vec3::ZERO,
            r: Vec3::ZERO,
            rd: Vec3::ZERO,
            rot: Rot3Deg::ZERO,
            bb: AABB::new(Vec3::ZERO, Vec3::ZERO),
            on_ground: false,
            height_offset: 0.0,
        };
        e.reset_pos();
        e
    }
}

impl EntityTrait for Entity {
    fn reset_pos(&mut self) {
        let x = rand::random::<f32>() * self.level.borrow().width as f32;
        let y = (self.level.borrow().depth + 10) as f32;
        let z = rand::random::<f32>() * self.level.borrow().height as f32;
        self.set_pos((x, y, z));
    }

    fn set_pos(&mut self, r: impl Into<Vec3>) {
        let r = r.into();
        self.r = r.into();
        let w = 0.3f32;
        let h = 0.9f32;
        self.bb = AABB::new(r - (w, h, w), r + (w, h, w));
    }

    fn turn(&mut self, rot: Rot3Deg) {
        self.rot = self.rot + rot * 0.15;
    }

    fn tick(&mut self, _engine: &Engine) {
        self.ro = self.r;
    }

    fn r#move(&mut self, a: impl Into<Vec3>) {
        let mut a = a.into();
        let a_org = a;
        let aabbs = self.level.borrow().get_cubes(self.bb.expand(a));
        for aabb in &aabbs {
            *a.y_mut() = aabb.clip_y_collide(&self.bb, a.y());
        }
        self.bb.r#move((0.0, a.y(), 0.0));
        for aabb in &aabbs {
            *a.x_mut() = aabb.clip_x_collide(&self.bb, a.x());
        }
        self.bb.r#move((a.x(), 0.0, 0.0));
        for aabb in &aabbs {
            *a.z_mut() = aabb.clip_z_collide(&self.bb, a.z());
        }
        self.bb.r#move((0.0, 0.0, a.z()));
        self.on_ground = a_org.y() != a.y() && a_org.y() < 0.0;
        if a_org.x() != a.x() {
            *self.rd.x_mut() = 0.0;
        }
        if a_org.y() != a.y() {
            *self.rd.y_mut() = 0.0;
        }
        if a_org.z() != a.z() {
            *self.rd.z_mut() = 0.0;
        }
        self.r = Vec3::new((self.bb.start.x() + self.bb.end.x()) / 2.0, self.bb.start.y() + self.height_offset, (self.bb.start.z() + self.bb.end.z()) / 2.0)
    }

    fn move_relative(&mut self, a: impl Into<Vec3>, speed: f32) {
        let mut a = a.into();
        let mut dist = a.x() * a.x() + a.z() * a.z();
        if dist < 0.01 {
            return;
        }
        dist = speed / dist.sqrt();
        let (sin, cos) = self.rot.y().sin_cos();
        a *= dist;
        self.rd += (a.x() * cos - a.z() * sin, 0.0, a.z() * cos + a.x() * sin);
    }
}

pub trait EntityTrait {
    fn reset_pos(&mut self);
    fn set_pos(&mut self, r: impl Into<Vec3>);
    fn turn(&mut self, rot: Rot3Deg);
    fn tick(&mut self, engine: &Engine);
    fn r#move(&mut self, a: impl Into<Vec3>);
    fn move_relative(&mut self, a: impl Into<Vec3>, speed: f32);
}