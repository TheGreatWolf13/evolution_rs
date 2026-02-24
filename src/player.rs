use crate::client::Engine;
use crate::math::angle::{Angle, Rot3Deg};
use crate::math::vec3::Vec3;
use crate::{level::level::Level, phys::aabb::AABB};
use glfw::Key;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Player {
    level: Rc<RefCell<Level>>,
    pub ro: Vec3,
    pub r: Vec3,
    pub rd: Vec3,
    pub rot: Rot3Deg,
    pub bb: AABB,
    pub on_ground: bool,
}

impl Player {
    pub fn new(level: Rc<RefCell<Level>>) -> Player {
        let r = Vec3::new(
            rand::random::<f32>() * level.borrow().width as f32,
            (level.borrow().depth + 10) as f32,
            rand::random::<f32>() * level.borrow().height as f32,
        );
        let w = 0.3;
        let h = 0.9;
        Player {
            level,
            ro: Vec3::ZERO,
            r,
            rd: Vec3::ZERO,
            rot: Rot3Deg::ZERO,
            bb: AABB::new(r - (w, h, w), r + (w, h, w)),
            on_ground: false,
        }
    }

    pub fn reset_pos(&mut self) {
        let x = rand::random::<f32>() * self.level.borrow().width as f32;
        let y = (self.level.borrow().depth + 10) as f32;
        let z = rand::random::<f32>() * self.level.borrow().height as f32;
        self.set_pos((x, y, z));
    }

    pub fn set_pos(&mut self, r: impl Into<Vec3>) {
        let r = r.into();
        self.r = r;
        let w = 0.3;
        let h = 0.9;
        self.bb = AABB::new(r - (w, h, w), r + (w, h, w));
    }

    pub fn turn(&mut self, rot: Rot3Deg) {
        self.rot = self.rot + rot * 0.15;
        // *self.rot.x_mut() = self.rot.x().clamp(-90.0, 90.0);
    }

    pub fn tick(&mut self, engine: &Engine) {
        self.ro = self.r;
        let mut xa = 0.0;
        let mut ya = 0.0;
        if engine.is_key_down(Key::R) {
            self.reset_pos();
        }
        if engine.is_key_down(Key::Up) || engine.is_key_down(Key::W) {
            ya -= 1.0;
        }
        if engine.is_key_down(Key::Down) || engine.is_key_down(Key::S) {
            ya += 1.0;
        }
        if engine.is_key_down(Key::Left) || engine.is_key_down(Key::A) {
            xa -= 1.0;
        }
        if engine.is_key_down(Key::Right) || engine.is_key_down(Key::D) {
            xa += 1.0;
        }
        if (engine.is_key_down(Key::Space)) && self.on_ground {
            *self.rd.y_mut() = 0.12;
        }
        let speed = if self.on_ground { 0.02 } else { 0.005 };
        self.move_relative((xa, 0.0, ya), speed);
        *self.rd.y_mut() = (self.rd.y() as f64 - 0.005) as f32;
        self.r#move(self.rd);
        self.rd *= (0.91, 0.98, 0.91);
        if self.on_ground {
            self.rd *= (0.8, 1.0, 0.8);
        }
    }

    pub fn r#move(&mut self, a: impl Into<Vec3>) {
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
        self.r = Vec3::new(
            (self.bb.start.x() + self.bb.end.x()) / 2.0,
            self.bb.start.y() + 1.62,
            (self.bb.start.z() + self.bb.end.z()) / 2.0,
        );
    }

    pub fn move_relative(&mut self, a: impl Into<Vec3>, speed: f32) {
        let mut a = a.into();
        let mut dist = a.horiz_len_sqr();
        if dist < 0.01 {
            return;
        }
        dist = speed / dist.sqrt();
        let (sin, cos) = self.rot.y().sin_cos();
        a *= dist;
        self.rd += (a.x() * cos - a.z() * sin, 0.0, a.z() * cos + a.x() * sin);
    }
}
