use crate::client::render::GL;
use glfw::{Action, Context, CursorMode, Glfw, GlfwReceiver, Key, MouseButton, OpenGlProfileHint, PWindow, SwapInterval, WindowEvent, WindowHint, WindowMode};
use std::collections::{HashSet, VecDeque};

pub mod render;

pub struct Engine {
    gl: GL,
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    is_closed_requested: bool,
    last_mouse_x: Option<f64>,
    last_mouse_y: Option<f64>,
    keys_pressed: HashSet<Key>,
    mouse_button_events: VecDeque<(MouseButton, bool)>,
    cur_button_event: Option<(MouseButton, bool)>,
}

impl Engine {
    pub fn new(width: u32, height: u32) -> Engine {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(1, 1));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Any));
        let (mut window, events) = glfw.create_window(width, height, "Game", WindowMode::Windowed).expect("Failed to create GLFW window");
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        glfw.set_swap_interval(SwapInterval::None);
        Engine {
            gl: GL::new(),
            glfw,
            window,
            events,
            is_closed_requested: false,
            last_mouse_x: None,
            last_mouse_y: None,
            keys_pressed: HashSet::new(),
            mouse_button_events: VecDeque::new(),
            cur_button_event: None,
        }
    }

    pub fn gl(&mut self) -> &mut GL {
        &mut self.gl
    }

    pub fn get_display_size(&self) -> (i32, i32) {
        self.window.get_size()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    self.gl.viewport(0, 0, width as u32, height as u32);
                }
                WindowEvent::Key(key, _, Action::Press, _) => {
                    self.keys_pressed.insert(key);
                }
                WindowEvent::Key(key, _, Action::Release, _) => {
                    self.keys_pressed.remove(&key);
                }
                WindowEvent::MouseButton(button, Action::Press, _) => {
                    self.mouse_button_events.push_back((button, true));
                }
                WindowEvent::MouseButton(button, Action::Release, _) => {
                    self.mouse_button_events.push_back((button, false));
                }
                _ => {}
            }
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_close_requested(&self) -> bool {
        self.is_closed_requested
    }

    pub fn grab_mouse(&mut self) {
        self.window.focus();
        self.window.set_cursor_mode(CursorMode::Disabled);
    }

    pub fn mouse_dx(&mut self) -> i32 {
        let cursor_pos: (f64, f64) = self.window.get_cursor_pos();
        let ret = if let Some(mouse_x) = self.last_mouse_x {
            (cursor_pos.0 - mouse_x) as i32
        } //
        else {
            0
        };
        self.last_mouse_x = Some(cursor_pos.0);
        ret
    }

    pub fn mouse_dy(&mut self) -> i32 {
        let cursor_pos = self.window.get_cursor_pos();
        let ret = if let Some(mouse_y) = self.last_mouse_y {
            (cursor_pos.1 - mouse_y) as i32
        } //
        else {
            0
        };
        self.last_mouse_y = Some(cursor_pos.1);
        ret
    }

    pub fn mouse_next(&mut self) -> bool {
        if self.mouse_button_events.is_empty() {
            return false;
        }
        self.cur_button_event = self.mouse_button_events.pop_front();
        true
    }

    pub fn mouse_event_button(&self) -> Option<MouseButton> {
        self.cur_button_event.map(|(b, _)| b)
    }

    pub fn mouse_event_button_state(&self) -> bool {
        self.cur_button_event.map(|(_, state)| state).unwrap_or(false)
    }
}
