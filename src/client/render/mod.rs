use crate::math::angle::Angle;
use crate::math::color::ColorRGBA;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::textures::Texture;
use bitmask_enum::bitmask;
use glfw::ffi::glfwSetErrorCallback;
use glu_sys::{glBegin, glBindTexture, glBlendFunc, glClear, glClearColor, glClearDepth, glColor4f, glDepthFunc, glDisable, glEnable, glEnd, glFogf, glFogfv, glFogi, glGenTextures, glGetError, glGetIntegerv, glInitNames, glLoadIdentity, glMatrixMode, glPopMatrix, glPopName, glPushMatrix, glPushName, glRenderMode, glRotatef, glScalef, glSelectBuffer, glShadeModel, glTexCoord2f, glTranslatef, glVertex3f, glViewport, gluPerspective, gluPickMatrix, GLbitfield, GLclampd, GLdouble, GLenum, GLfloat, GLint, GLsizei, GLuint, GL_FOG_COLOR, GL_FOG_DENSITY, GL_FOG_MODE, GL_VIEWPORT};
use std::ffi::{c_char, c_int};
use std::marker::PhantomData;
use strum::FromRepr;
use typed_floats::Positive;

pub struct GL {
    _a: (),
}

pub struct GLDrawing<'a> {
    _a: PhantomData<&'a mut GL>,
}

impl<'a> Drop for GLDrawing<'a> {
    fn drop(&mut self) {
        unsafe {
            glEnd();
        }
    }
}

impl<'a> GLDrawing<'a> {
    pub fn end(self) {
        //Just drop
    }

    pub fn vertex(&mut self, pos: impl Into<Vec3>) {
        let pos = pos.into();
        unsafe {
            glVertex3f(pos.x(), pos.y(), pos.z());
        }
    }
}

impl GL {
    pub(super) fn new() -> Self {
        unsafe {
            glfwSetErrorCallback(Some(Self::log_error));
        }
        GL {
            _a: ()
        }
    }

    unsafe extern "C" fn log_error(code: c_int, desc: *const c_char) {
        eprintln!("RENDER ERROR {code}: {:?}", desc)
    }

    pub fn init_names(&mut self) {
        unsafe {
            glInitNames()
        }
    }

    pub fn push_name(&mut self, name: u32) {
        unsafe {
            glPushName(name);
            //StackOverflow
            debug_assert_eq!(GLError::from_repr(glGetError()).unwrap(), GLError::NoError);
        }
    }

    pub fn pop_name(&mut self) {
        unsafe {
            glPopName();
            //StackUnderflow
            debug_assert_eq!(GLError::from_repr(glGetError()).unwrap(), GLError::NoError);
        }
    }

    pub fn enable(&mut self, cap: GLCap) {
        unsafe {
            glEnable(cap as GLenum);
        }
    }

    pub fn disable(&mut self, cap: GLCap) {
        unsafe {
            glDisable(cap as GLenum);
        }
    }

    pub fn blend_func(&mut self, src: GLSource, dst: GLDestination) {
        unsafe {
            glBlendFunc(src as GLenum, dst as GLenum);
        }
    }

    pub fn load_identity(&mut self) {
        unsafe {
            glLoadIdentity();
        }
    }

    pub fn render_mode(&mut self, mode: GLRenderMode) -> GLint {
        unsafe {
            let ret = glRenderMode(mode as GLenum);
            //InvalidOperation
            debug_assert_eq!(GLError::from_repr(glGetError()).unwrap(), GLError::NoError);
            ret
        }
    }

    pub fn select_buffer(&mut self, buffer: &mut [GLuint]) {
        unsafe {
            glSelectBuffer(buffer.len() as GLsizei, buffer.as_mut_ptr());
            //InvalidOperation
            debug_assert_eq!(GLError::from_repr(glGetError()).unwrap(), GLError::NoError);
        }
    }

    pub fn matrix_mode(&mut self, mode: GLMatrix) {
        unsafe {
            glMatrixMode(mode as GLenum);
        }
    }

    pub fn translate(&mut self, a: impl Into<Vec3>) {
        let a = a.into();
        unsafe {
            glTranslatef(a.x(), a.y(), a.z());
        }
    }

    pub fn rotate(&mut self, angle: impl Angle, vec: impl Into<Vec3>) {
        let vec = vec.into();
        unsafe {
            glRotatef(*angle.to_degrees(), vec.x(), vec.y(), vec.z());
        }
    }

    pub fn perspective(y_fov: impl Angle, aspect: GLdouble, z_near: GLdouble, z_far: GLdouble) {
        unsafe {
            gluPerspective(*y_fov.to_degrees() as GLdouble, aspect, z_near, z_far);
        }
    }

    pub fn pick_matrix(pos: impl Into<Vec2>, size: impl Into<Vec2>, viewport: &mut ViewportBuffer) {
        let pos = pos.into();
        let size = size.into();
        unsafe {
            gluPickMatrix(pos.x() as GLdouble, pos.y() as GLdouble, size.x() as GLdouble, size.y() as GLdouble, viewport.0.as_mut_ptr());
        }
    }

    pub fn shade_model(&mut self, mode: GLShading) {
        unsafe {
            glShadeModel(mode as GLenum);
        }
    }

    pub fn depth_func(&mut self, func: GLDepth) {
        unsafe {
            glDepthFunc(func as GLenum);
        }
    }

    pub fn clear_color(&mut self, color: ColorRGBA) {
        unsafe {
            glClearColor(color.r(), color.g(), color.b(), color.a());
        }
    }

    pub fn clear_depth(&mut self, depth: f32) {
        debug_assert!(0.0 <= depth && depth <= 1.0);
        unsafe {
            glClearDepth(depth as GLclampd);
        }
    }

    pub fn get_viewport(&mut self, viewport: &mut ViewportBuffer) {
        unsafe {
            glGetIntegerv(GL_VIEWPORT, viewport.0.as_mut_ptr());
        }
    }

    pub fn clear(&mut self, buffers: GLBuffer) {
        unsafe {
            glClear(buffers.bits as GLbitfield);
        }
    }

    pub fn fog_mode(&mut self, mode: GLFogMode) {
        unsafe {
            glFogi(GL_FOG_MODE, mode as GLint);
        }
    }

    pub fn fog_density(&mut self, density: Positive<f32>) {
        unsafe {
            glFogf(GL_FOG_DENSITY, f32::from(density) as GLfloat);
        }
    }

    pub fn fog_color(&mut self, color: ColorRGBA) {
        unsafe {
            glFogfv(GL_FOG_COLOR, ((&color) as *const ColorRGBA) as *const GLfloat);
        }
    }

    pub fn push_matrix(&mut self) {
        unsafe {
            glPushMatrix();
            //StackOverflow
            debug_assert_eq!(GLError::from_repr(glGetError()).unwrap(), GLError::NoError);
        }
    }

    pub fn pop_matrix(&mut self) {
        unsafe {
            glPopMatrix();
            //StackUnderflow
            debug_assert_eq!(GLError::from_repr(glGetError()).unwrap(), GLError::NoError);
        }
    }

    pub fn scale(&mut self, scale: impl Into<Vec3>) {
        let scale = scale.into();
        unsafe {
            glScalef(scale.x(), scale.y(), scale.z());
        }
    }

    pub fn bind_texture(&mut self, texture: Texture) {
        unsafe {
            glBindTexture(texture.target() as GLenum, texture.id());
        }
    }

    pub fn gen_textures(&mut self, buffer: &mut [u32]) {
        unsafe {
            glGenTextures(buffer.len() as GLsizei, buffer.as_mut_ptr());
        }
    }

    #[must_use]
    pub fn begin<'a>(&'a mut self, mode: GLDrawMode) -> GLDrawing<'a> {
        unsafe {
            glBegin(mode as GLenum);
        }
        GLDrawing {
            _a: PhantomData,
        }
    }

    pub fn color(color: ColorRGBA) {
        unsafe {
            glColor4f(color.r(), color.g(), color.b(), color.a());
        }
    }

    pub fn tex_coord(s: f32, t: f32) {
        unsafe {
            glTexCoord2f(s as GLfloat, t as GLfloat);
        }
    }

    pub fn viewport(&mut self, x: i32, y: i32, width: u32, height: u32) {
        unsafe {
            glViewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei);
        }
    }
}

pub struct ViewportBuffer([GLint; 4]);

impl ViewportBuffer {
    pub fn new() -> Self {
        ViewportBuffer([0; 4])
    }

    pub fn clear(&mut self) {
        for i in 0..4 {
            self.0[i] = 0;
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum GLDrawMode {
    Points = 0,
    Lines = 1,
    LineStrip = 3,
    LineLoop = 2,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
    Quads = 7,
    QuadStrip = 8,
    Polygon = 9,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum GLTextureMode {
    Nearest = 9728,
    Linear = 9729,
    NearestMipmapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum GLTexTarget {
    Texture1D = 3552,
    Texture2D = 3553,
    Texture3D = 32879,
    Texture1DArray = 35864,
    Texture2DArray = 35866,
    TextureRectangle = 34037,
    TextureCubemap = 34067,
    TextureCubemapArray = 36873,
    TextureBuffer = 35882,
    Texture2DMultisample = 37120,
    Texture2DMultisampleArray = 37122,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromRepr)]
#[allow(unused)]
pub enum GLError {
    NoError = 0,
    InvalidEnum = 1280,
    InvalidValue = 1281,
    InvalidOperation = 1282,
    InvalidFramebufferOperation = 1286,
    OutOfMemory = 1285,
    StackUnderflow = 1284,
    StackOverflow = 1283,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub enum GLFogMode {
    Linear = 9729,
    Exp = 2048,
    Exp2 = 2049,
}

#[bitmask(u32)]
#[allow(unused)]
pub enum GLBuffer {
    ColorBuffer = 16384,
    DepthBuffer = 256,
    AccumBuffer = 512,
    StencilBuffer = 1024,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum GLDepth {
    Never = 512,
    Less = 513,
    Equal = 514,
    LessEqual = 515,
    Greater = 516,
    NotEqual = 517,
    GreaterEqual = 518,
    Always = 519,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum GLShading {
    Flat = 7424,
    Smooth = 7425,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(unused)]
pub enum GLMatrix {
    ModelView = 5888,
    Projection = 5889,
    Texture = 5890,
    Color = 6144,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum GLRenderMode {
    Render = 7168,
    Select = 7170,
    Feedback = 7169,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum GLCap {
    AlphaTest = 3008,
    AutoNormal = 3456,
    Blend = 3042,
    ClipPlane0 = 12288,
    ClipPlane1 = 12289,
    ClipPlane2 = 12290,
    ClipPlane3 = 12291,
    ClipPlane4 = 12292,
    ClipPlane5 = 12293,
    ColorLogicOp = 3058,
    ColorMaterial = 2903,
    ColorSum = 33880,
    ColorTable = 32976,
    Convolution1D = 32784,
    Convolution2D = 32785,
    CullFace = 2884,
    DepthTest = 2929,
    Dither = 3024,
    Fog = 2912,
    Histogram = 32804,
    IndexLogicOp = 3057,
    Light0 = 16384,
    Light1 = 16385,
    Light2 = 16386,
    Light3 = 16387,
    Light4 = 16388,
    Light5 = 16389,
    Light6 = 16390,
    Light7 = 16391,
    Lighting = 2896,
    LineSmooth = 2848,
    LineStipple = 2852,
    Map1Color4 = 3472,
    Map1Index = 3473,
    Map1Normal = 3474,
    Map1TextureCoord1 = 3475,
    Map1TextureCoord2 = 3476,
    Map1TextureCoord3 = 3477,
    Map1TextureCoord4 = 3478,
    Map1Vertex3 = 3479,
    Map1Vertex4 = 3480,
    Map2Color4 = 3504,
    Map2Index = 3505,
    Map2Normal = 3506,
    Map2TextureCoord1 = 3507,
    Map2TextureCoord2 = 3508,
    Map2TextureCoord3 = 3509,
    Map2TextureCoord4 = 3510,
    Map2Vertex3 = 3511,
    Map2Vertex4 = 3512,
    MinMax = 32814,
    Multisample = 32925,
    Normalize = 2977,
    PointSmooth = 2832,
    PointSprite = 34913,
    PolygonOffsetFill = 32823,
    PolygonOffsetLine = 10754,
    PolygonOffsetPoint = 10753,
    PolygonSmooth = 2881,
    PolygonStipple = 2882,
    PostColorMatrixColorTable = 32978,
    PostConvolutionColorTable = 32977,
    RescaleNormal = 32826,
    SampleAlphaToCoverage = 32926,
    SampleAlphaToOne = 32927,
    SampleCoverage = 32928,
    Separable2D = 32786,
    ScissorTest = 3089,
    StencilTest = 2960,
    Texture1D = 3552,
    Texture2D = 3553,
    Texture3D = 32879,
    TextureCubeMap = 34067,
    TextureGenQ = 3171,
    TextureGenR = 3170,
    TextureGenS = 3168,
    TextureGenT = 3169,
    VertexProgramPointSize = 34370,
    VertexProgramTwoSide = 34371,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum GLSource {
    Zero = 0,
    One = 1,
    SrcColor = 768,
    OneMinusSrcColor = 769,
    DstColor = 774,
    OneMinusDstColor = 775,
    SrcAlpha = 770,
    OneMinusSrcAlpha = 771,
    DstAlpha = 772,
    OneMinusDstAlpha = 773,
    ConstantColor = 32769,
    OneMinusConstantColor = 32770,
    ConstantAlpha = 32771,
    OneMinusConstantAlpha = 32772,
    SrcAlphaSaturate = 776,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum GLDestination {
    Zero = 0,
    One = 1,
    SrcColor = 768,
    OneMinusSrcColor = 769,
    DstColor = 774,
    OneMinusDstColor = 775,
    SrcAlpha = 770,
    OneMinusSrcAlpha = 771,
    DstAlpha = 772,
    OneMinusDstAlpha = 773,
    ConstantColor = 32769,
    OneMinusConstantColor = 32770,
    ConstantAlpha = 32771,
    OneMinusConstantAlpha = 32772,
}