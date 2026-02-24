use crate::client::render::{GLTexTarget, GLTextureMode, GL};
use glu_sys::{glTexParameteri, gluBuild2DMipmaps, GLint, GLuint, GLvoid, GL_RGBA, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_UNSIGNED_BYTE};
use image::ImageReader;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref ID_MAP: Mutex<HashMap<String, Texture>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Copy, Clone)]
pub struct Texture(u32, GLTexTarget);

impl Texture {
    pub fn id(&self) -> GLuint {
        self.0
    }

    pub fn target(&self) -> GLTexTarget {
        self.1
    }

    pub fn load(gl: &mut GL, resource_name: &str, mode: GLTextureMode) -> Texture {
        if ID_MAP.lock().unwrap().contains_key(resource_name) {
            return *ID_MAP.lock().unwrap().get(resource_name).unwrap();
        }
        let mut id = [0; 1];
        gl.gen_textures(&mut id);
        let text = Texture(id[0], GLTexTarget::Texture2D);
        ID_MAP.lock().unwrap().insert(resource_name.to_string(), text);
        println!("{} -> {:?}", resource_name, text);
        unsafe {
            gl.bind_texture(text);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, mode as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, mode as GLint);
            let img = ImageReader::open(resource_name).unwrap().decode().unwrap();
            let img = img.to_rgba8();
            let (w, h) = img.dimensions();
            let pixels = img.as_raw().clone();
            gluBuild2DMipmaps(GL_TEXTURE_2D, GL_RGBA as GLint, w as GLint, h as GLint, GL_RGBA, GL_UNSIGNED_BYTE, pixels.as_ptr() as *const GLvoid);
        }
        text
    }
}

