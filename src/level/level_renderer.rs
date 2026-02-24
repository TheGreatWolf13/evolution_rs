use super::{
    chunk::{self, Chunk},
    frustrum::Frustum,
    level::Level,
    level_listener::LevelListener,
    tesselator::Tesselator,
    tile,
};
use crate::client::render::GLCap::Blend;
use crate::client::render::GLDestination::One;
use crate::client::render::GLSource::SrcAlpha;
use crate::client::render::{GLTextureMode, GL};
use crate::math::vec3::Vec3;
use crate::textures::Texture;
use crate::{hit_result::HitResult, player::Player};
use glu_sys::glColor4f;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::UNIX_EPOCH;
use std::{cell::RefCell, time::SystemTime};

pub struct LevelRenderer {
    level: Rc<RefCell<Level>>,
    chunks: Vec<Option<Chunk>>,
    x_chunks: i32,
    y_chunks: i32,
    z_chunks: i32,
    tesselator: Tesselator,
    terrain_texture: Texture,
}

impl LevelRenderer {
    pub fn new(level: Rc<RefCell<Level>>, gl: &mut GL) -> Rc<RefCell<LevelRenderer>> {
        let x_chunks = level.borrow().width / 16;
        let y_chunks = level.borrow().depth / 16;
        let z_chunks = level.borrow().height / 16;
        let mut chunks: Vec<Option<Chunk>> = std::iter::repeat_with(|| None).take((x_chunks * y_chunks * z_chunks) as usize).collect();
        for x in 0..x_chunks {
            for y in 0..y_chunks {
                for z in 0..z_chunks {
                    let x0 = x * 16;
                    let y0 = y * 16;
                    let z0 = z * 16;
                    let mut x1 = (x + 1) * 16;
                    let mut y1 = (y + 1) * 16;
                    let mut z1 = (z + 1) * 16;
                    if x1 > level.borrow().width {
                        x1 = level.borrow().width;
                    }
                    if y1 > level.borrow().depth {
                        y1 = level.borrow().depth;
                    }
                    if z1 > level.borrow().height {
                        z1 = level.borrow().height;
                    }
                    chunks[((x + y * x_chunks) * z_chunks + z) as usize] = Some(Chunk::new(Rc::clone(&level), x0, y0, z0, x1, y1, z1));
                }
            }
        }
        let lr = Rc::new(RefCell::new(LevelRenderer {
            level,
            chunks,
            x_chunks,
            y_chunks,
            z_chunks,
            tesselator: Tesselator::new(),
            terrain_texture: Texture::load(gl, "terrain.png", GLTextureMode::Nearest),
        }));
        lr.borrow().level.borrow_mut().add_listener(Rc::clone(&lr) as Rc<RefCell<dyn LevelListener>>);
        lr
    }

    pub fn render(&mut self, gl: &mut GL, _player: &Player, layer: i32) {
        chunk::REBUILT_THIS_FRAME.store(0, Ordering::SeqCst);
        let frustum = Frustum::get_frustum();
        for chunk in &mut self.chunks {
            if let Some(chunk) = chunk {
                if frustum.lock().unwrap().cube_in_frustum_aabb(&chunk.aabb) {
                    chunk.render(gl, self.terrain_texture, layer);
                }
            }
        }
    }

    pub fn pick(&mut self, gl: &mut GL, player: &Player) {
        let r = 3.0;
        let box_aabb = player.bb.grow(Vec3::splat(r));
        let x0 = box_aabb.start.x() as i32;
        let x1 = (box_aabb.end.x() + 1.0) as i32;
        let y0 = box_aabb.start.y() as i32;
        let y1 = (box_aabb.end.y() + 1.0) as i32;
        let z0 = box_aabb.start.z() as i32;
        let z1 = (box_aabb.end.z() + 1.0) as i32;
        gl.init_names();
        for x in x0..x1 {
            gl.push_name(x as u32);
            for y in y0..y1 {
                gl.push_name(y as u32);
                for z in z0..z1 {
                    gl.push_name(z as u32);
                    if self.level.borrow().is_solid_tile(x, y, z) {
                        gl.push_name(0);
                        for i in 0..6 {
                            gl.push_name(i);
                            self.tesselator.init();
                            tile::ROCK.lock().unwrap().render_face(&mut self.tesselator, x, y, z, i as i32);
                            self.tesselator.flush();
                            gl.pop_name();
                        }
                        gl.pop_name();
                    }
                    gl.pop_name();
                }
                gl.pop_name();
            }
            gl.pop_name();
        }
    }

    pub fn render_hit(&mut self, gl: &mut GL, h: &HitResult) {
        gl.enable(Blend);
        gl.blend_func(SrcAlpha, One);
        let current_time_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        unsafe {
            glColor4f(1.0, 1.0, 1.0, ((current_time_millis as f64 / 100.0).sin() * 0.2 + 0.4) as f32);
        }
        self.tesselator.init();
        tile::ROCK.lock().unwrap().render_face(&mut self.tesselator, h.x, h.y, h.z, h.f);
        self.tesselator.flush();
        gl.disable(Blend);
    }

    pub fn set_dirty(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
        let mut x0 = x0 / 16;
        let mut x1 = x1 / 16;
        let mut y0 = y0 / 16;
        let mut y1 = y1 / 16;
        let mut z0 = z0 / 16;
        let mut z1 = z1 / 16;
        if x0 < 0 {
            x0 = 0;
        }
        if y0 < 0 {
            y0 = 0;
        }
        if z0 < 0 {
            z0 = 0;
        }
        if x1 >= self.x_chunks {
            x1 = self.x_chunks - 1;
        }
        if y1 >= self.y_chunks {
            y1 = self.y_chunks - 1;
        }
        if z1 >= self.z_chunks {
            z1 = self.z_chunks - 1;
        }
        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    if let Some(chunk) = &mut self.chunks[((x + y * self.x_chunks) * self.z_chunks + z) as usize] {
                        chunk.set_dirty();
                    }
                }
            }
        }
    }
}

impl LevelListener for LevelRenderer {
    fn tile_changed(&mut self, x: i32, y: i32, z: i32) {
        self.set_dirty(x - 1, y - 1, z - 1, x + 1, y + 1, z + 1);
    }

    fn light_column_changed(&mut self, x: i32, z: i32, y0: i32, y1: i32) {
        self.set_dirty(x - 1, y0 - 1, z - 1, x + 1, y1 + 1, z + 1);
    }

    fn all_changed(&mut self) {
        let x1 = self.level.borrow().width;
        let y1 = self.level.borrow().depth;
        let z1 = self.level.borrow().height;
        self.set_dirty(0, 0, 0, x1, y1, z1);
    }
}
