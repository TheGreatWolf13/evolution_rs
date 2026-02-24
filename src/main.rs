#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate lazy_static;

use crate::character::zombie::Zombie;
use crate::client::render::{GLBuffer, GLCap, GLDepth, GLFogMode, GLMatrix, GLRenderMode, GLShading, ViewportBuffer, GL};
use crate::client::Engine;
use crate::entity::EntityTrait;
use crate::math::angle::{AngleDeg, Rot3Deg};
use glfw::{Key, MouseButton};
use glu_sys::GLuint;
use hit_result::HitResult;
use level::level::Level;
use level::level_renderer::LevelRenderer;
use player::Player;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
use typed_floats::tf32::Positive;

mod hit_result;
mod level;
mod phys;
mod player;
mod textures;
mod timer;
mod character;
mod entity;
mod math;
mod client;

use crate::level::chunk;
use crate::math::color::ColorRGBA;
use crate::math::vec3::Vec3;
use crate::timer::Timer;

struct Game {
    engine: Engine,
    fog_color: ColorRGBA,
    timer: Timer,
    level: Rc<RefCell<Level>>,
    level_renderer: Rc<RefCell<LevelRenderer>>,
    player: Player,
    select_buffer: [GLuint; 2000],
    viewport_buffer: ViewportBuffer,
    hit_result: Option<HitResult>,
    zombies: Vec<Zombie>,
}

impl Game {
    pub fn new() -> Game {
        let fog_color = ColorRGBA::hex(0xFF0E0B0A);
        let mut engine = Engine::new(1024, 768);
        let gl = engine.gl();
        gl.enable(GLCap::Texture2D);
        gl.shade_model(GLShading::Smooth);
        gl.clear_color(ColorRGBA::rgba(0.5, 0.8, 1.0, 0.0));
        gl.clear_depth(1.0);
        gl.enable(GLCap::DepthTest);
        gl.depth_func(GLDepth::LessEqual);
        gl.matrix_mode(GLMatrix::Projection);
        gl.load_identity();
        gl.matrix_mode(GLMatrix::ModelView);
        let level = Rc::new(RefCell::new(Level::new(256, 256, 64)));
        let level_renderer = LevelRenderer::new(Rc::clone(&level), gl);
        let player = Player::new(Rc::clone(&level));
        engine.grab_mouse();
        let mut zombies = Vec::with_capacity(100);
        for _ in 0..100 {
            zombies.push(Zombie::new(Rc::clone(&level), (128.0, 0.0, 128.0)));
        }
        Game {
            engine,
            fog_color,
            timer: Timer::new(60.0),
            level,
            level_renderer,
            player,
            select_buffer: [0; 2000],
            viewport_buffer: ViewportBuffer::new(),
            hit_result: None,
            zombies,
        }
    }

    pub fn destroy(&self) {
        self.level.borrow().save();
    }

    pub fn run(&mut self) {
        let mut frames = 0;
        let mut last_time = Instant::now();
        loop {
            if self.engine.is_close_requested() || self.engine.is_key_down(Key::Escape) {
                break;
            }
            self.timer.advance_time();
            let mut i = 0;
            while i < self.timer.ticks {
                self.tick();
                i += 1;
            }
            self.render(self.timer.partial_ticks);
            frames += 1;
            while Instant::now().duration_since(last_time).as_millis() > 1000 {
                println!("{} fps, {}", frames, chunk::UPDATES.load(Ordering::SeqCst));
                chunk::UPDATES.store(0, Ordering::SeqCst);
                last_time = last_time.checked_add(Duration::from_millis(1000)).unwrap();
                frames = 0;
            }
        }
        self.destroy();
    }

    pub fn tick(&mut self) {
        for zombie in &mut self.zombies {
            zombie.tick(&self.engine);
        }
        self.player.tick(&self.engine);
    }

    pub fn move_camera_to_player(&mut self, a: f32) {
        let gl = self.engine.gl();
        gl.translate((0.0, 0.0, -0.3));
        gl.rotate(self.player.rot.x(), Vec3::X);
        gl.rotate(self.player.rot.y(), Vec3::Y);
        gl.rotate(self.player.rot.z(), Vec3::Z);
        let r = -(self.player.ro + (self.player.r - self.player.ro) * a);
        gl.translate(r);
    }

    pub fn setup_camera(&mut self, a: f32) {
        let size = self.engine.get_display_size();
        let gl = self.engine.gl();
        gl.matrix_mode(GLMatrix::Projection);
        gl.load_identity();
        GL::perspective(AngleDeg::new(70.0), size.0 as f64 / size.1 as f64, 0.05, 1000.0);
        gl.matrix_mode(GLMatrix::ModelView);
        gl.load_identity();
        self.move_camera_to_player(a);
    }

    fn setup_pick_camera(&mut self, a: f32, x: i32, y: i32) {
        let size = self.engine.get_display_size();
        let gl = self.engine.gl();
        gl.matrix_mode(GLMatrix::Projection);
        gl.load_identity();
        self.viewport_buffer.clear();
        gl.get_viewport(&mut self.viewport_buffer);
        GL::pick_matrix((x as f32, y as f32), (5.0, 5.0), &mut self.viewport_buffer);
        GL::perspective(AngleDeg::new(70.0), size.0 as f64 / size.1 as f64, 0.05, 1000.0);
        gl.matrix_mode(GLMatrix::ModelView);
        gl.load_identity();
        self.move_camera_to_player(a);
    }

    pub fn pick(&mut self, partial_ticks: f32) {
        for i in 0..2000 {
            self.select_buffer[i] = 0;
        }
        let size = self.engine.get_display_size();
        let gl = self.engine.gl();
        gl.select_buffer(&mut self.select_buffer);
        gl.render_mode(GLRenderMode::Select);
        self.setup_pick_camera(partial_ticks, size.0 / 2, size.1 / 2);
        let gl = self.engine.gl();
        self.level_renderer.borrow_mut().pick(gl, &self.player);
        let hits = gl.render_mode(GLRenderMode::Render);
        let mut closest = 0;
        let mut names = [0i32; 10];
        let mut hit_name_count = 0;
        let mut pos = 0;
        for i in 0..hits {
            let name_count = self.select_buffer[pos];
            pos += 1;
            let min_z = self.select_buffer[pos];
            pos += 1;
            pos += 1;
            let dist = min_z;
            if dist < closest || i == 0 {
                closest = dist;
                hit_name_count = name_count;
                for j in 0..name_count {
                    names[j as usize] = self.select_buffer[pos] as i32;
                    pos += 1;
                }
            } //
            else {
                pos += name_count as usize;
            }
        }
        self.hit_result = if hit_name_count > 0 {
            Some(HitResult::new(names[0], names[1], names[2], names[3], names[4]))
        } //
        else {
            None
        };
    }

    pub fn render(&mut self, partial_ticks: f32) {
        let rot = Rot3Deg::new(
            AngleDeg::new(self.engine.mouse_dy() as f32),
            AngleDeg::new(self.engine.mouse_dx() as f32),
            AngleDeg::ZERO,
        );
        self.player.turn(rot);
        self.pick(partial_ticks);
        while self.engine.mouse_next() {
            if self.engine.mouse_event_button() == Some(MouseButton::Left) && self.engine.mouse_event_button_state() {
                if let Some(hit_result) = &self.hit_result {
                    self.level.borrow_mut().set_tile(hit_result.x, hit_result.y, hit_result.z, 0);
                }
            }
            if self.engine.mouse_event_button() != Some(MouseButton::Left) || !self.engine.mouse_event_button_state() || self.hit_result.is_none() {
                continue;
            }
            if let Some(hit_result) = &self.hit_result {
                let mut x = hit_result.x;
                let mut y = hit_result.y;
                let mut z = hit_result.z;
                if hit_result.f == 0 {
                    y -= 1;
                }
                if hit_result.f == 1 {
                    y += 1;
                }
                if hit_result.f == 2 {
                    z -= 1;
                }
                if hit_result.f == 3 {
                    z += 1;
                }
                if hit_result.f == 4 {
                    x -= 1;
                }
                if hit_result.f == 5 {
                    x += 1;
                }
                self.level.borrow_mut().set_tile(x, y, z, 1);
            }
        }
        if self.engine.is_key_down(Key::Enter) {
            self.level.borrow().save();
        }
        let gl = self.engine.gl();
        gl.clear(GLBuffer::ColorBuffer.or(GLBuffer::DepthBuffer));
        self.setup_camera(partial_ticks);
        let gl = self.engine.gl();
        gl.enable(GLCap::CullFace);
        gl.enable(GLCap::Fog);
        gl.fog_mode(GLFogMode::Exp);
        gl.fog_density(Positive::new(0.2).unwrap());
        gl.fog_color(self.fog_color);
        gl.disable(GLCap::Fog);
        self.level_renderer.borrow_mut().render(gl, &self.player, 0);
        for zombie in &mut self.zombies {
            zombie.render(gl, partial_ticks);
        }
        gl.enable(GLCap::Fog);
        self.level_renderer.borrow_mut().render(gl, &self.player, 1);
        gl.disable(GLCap::Texture2D);
        if let Some(hit_result) = &self.hit_result {
            self.level_renderer.borrow_mut().render_hit(gl, hit_result);
        }
        gl.disable(GLCap::Fog);
        self.engine.update();
    }
}

pub fn main() {
    let mut rd = Game::new();
    rd.run();
}
