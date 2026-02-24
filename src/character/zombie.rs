use super::cube::Cube;
use crate::client::render::{GLCap, GLTextureMode, GL};
use crate::client::Engine;
use crate::math::angle::{AngleDeg, Rot3Deg};
use crate::math::vec3::Vec3;
use crate::textures::Texture;
use crate::{entity::{Entity, EntityTrait}, level::level::Level, timer::PROGRAM_START};
use std::{cell::RefCell, f64::consts::PI, rc::Rc, time::Instant};

pub struct Zombie {
    pub entity: Entity,
    pub head: Cube,
    pub body: Cube,
    pub arm0: Cube,
    pub arm1: Cube,
    pub leg0: Cube,
    pub leg1: Cube,
    pub rot: AngleDeg,
    pub time_offs: f32,
    pub speed: f32,
    pub rot_a: AngleDeg,
}

impl Zombie {
    pub fn new(level: Rc<RefCell<Level>>, r: impl Into<Vec3>) -> Zombie {
        let r = r.into();
        let mut entity = Entity::new(level);
        entity.r = r;
        let mut head = Cube::new(0, 0);
        head.add_box((-4.0, -8.0, -4.0), 8, 8, 8);
        let mut body = Cube::new(16, 16);
        body.add_box((-4.0, 0.0, -2.0), 8, 12, 4);
        let mut arm0 = Cube::new(40, 16);
        arm0.add_box((-3.0, -2.0, -2.0), 4, 12, 4);
        arm0.set_pos((-5.0, 2.0, 0.0));
        let mut arm1 = Cube::new(40, 16);
        arm1.add_box((-1.0, -2.0, -2.0), 4, 12, 4);
        arm1.set_pos((5.0, 2.0, 0.0));
        let mut leg0 = Cube::new(0, 16);
        leg0.add_box((-2.0, 0.0, -2.0), 4, 12, 4);
        leg0.set_pos((-2.0, 12.0, 0.0));
        let mut leg1 = Cube::new(0, 16);
        leg1.add_box((-2.0, 0.0, -2.0), 4, 12, 4);
        leg1.set_pos((2.0, 12.0, 0.0));
        Zombie {
            entity,
            head,
            body,
            arm0,
            arm1,
            leg0,
            leg1,
            rot: AngleDeg::new(rand::random::<f32>() * 360.0),
            time_offs: rand::random::<f64>() as f32 * 1239813.0,
            speed: 1.0,
            rot_a: AngleDeg::new((rand::random::<f32>() + 1.0) * 0.01),
        }
    }

    pub fn render(&mut self, gl: &mut GL, partial_ticks: f32) {
        gl.enable(GLCap::Texture2D);
        let texture = Texture::load(gl, "char.png", GLTextureMode::Nearest);
        gl.bind_texture(texture);
        gl.push_matrix();
        let time = Instant::now().duration_since(*PROGRAM_START).as_nanos() as f64 / 1.0E9 * 10.0 * self.speed as f64 + self.time_offs as f64;
        let size = 7.0 / 120.0;
        let yy = (-(time * 0.6662).sin().abs() * 5.0 - 23.0) as f32;
        let this = &mut self.entity;
        let r = this.ro + (this.r - this.ro) * partial_ticks;
        gl.translate(r);
        gl.scale((1.0, -1.0, 1.0));
        gl.scale((size, size, size));
        gl.translate((0.0, yy, 0.0));
        gl.rotate(self.rot + AngleDeg::new(180.0), Vec3::Y);
        *self.head.rot.y_mut() = AngleDeg::new((((time * 0.83).sin()) * (180.0 / PI)) as f32);
        *self.head.rot.x_mut() = AngleDeg::new(((time.sin() * 0.8) * (180.0 / PI)) as f32);
        *self.arm0.rot.x_mut() = AngleDeg::new((((time * 0.6662 + PI).sin() * 2.0) * (180.0 / PI)) as f32);
        *self.arm0.rot.z_mut() = AngleDeg::new((((time * 0.2312).sin() + 1.0) * (180.0 / PI)) as f32);
        *self.arm1.rot.x_mut() = AngleDeg::new((((time * 0.6662).sin() * 2.0) * (180.0 / PI)) as f32);
        *self.arm1.rot.z_mut() = AngleDeg::new((((time * 0.2812).sin() - 1.0) * (180.0 / PI)) as f32);
        *self.leg0.rot.x_mut() = AngleDeg::new((((time * 0.6662).sin() * 1.4) * (180.0 / PI)) as f32);
        *self.leg1.rot.x_mut() = AngleDeg::new((((time * 0.6662 + PI).sin() * 1.4) * (180.0 / PI)) as f32);
        self.head.render(gl);
        self.body.render(gl);
        self.arm0.render(gl);
        self.arm1.render(gl);
        self.leg0.render(gl);
        self.leg1.render(gl);
        gl.pop_matrix();
        gl.disable(GLCap::Texture2D);
    }
}

impl EntityTrait for Zombie {
    fn reset_pos(&mut self) {
        self.entity.reset_pos();
    }

    fn set_pos(&mut self, r: impl Into<Vec3>) {
        self.entity.set_pos(r);
    }

    fn turn(&mut self, rot: Rot3Deg) {
        self.entity.turn(rot);
    }

    fn tick(&mut self, _engine: &Engine) {
        let this = &mut self.entity;
        this.ro = this.r;
        self.rot += self.rot_a;
        self.rot_a *= 0.99;
        self.rot_a += AngleDeg::new((rand::random::<f32>() - rand::random::<f32>()) * rand::random::<f32>() * rand::random::<f32>() * 0.01);
        let (sin, cos) = self.rot.sin_cos();
        if this.on_ground && rand::random::<f64>() < 0.01 {
            *this.rd.y_mut() = 0.12;
        }
        this.move_relative(
            (sin, 0.0, cos),
            if this.on_ground {
                0.02
            } //
            else {
                0.005
            });
        *this.rd.y_mut() = (this.rd.y() as f64 - 0.005) as f32;
        this.r#move(this.rd);
        this.rd *= (0.91, 0.98, 0.91);
        if this.r.y() > 100.0 {
            this.reset_pos();
        }
        if this.on_ground {
            this.rd *= (0.8, 1.0, 0.8);
        }
    }

    fn r#move(&mut self, a: impl Into<Vec3>) {
        self.entity.r#move(a);
    }

    fn move_relative(&mut self, a: impl Into<Vec3>, speed: f32) {
        self.entity.move_relative(a, speed);
    }
}