extern crate sdl2;
extern crate gl;

use std;
use gl::types::*;

pub struct Display {
    pub res: ::util::resources::Resources,
    sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: sdl2::video::GLContext,
    pub width: f32,
    pub height: f32,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub keys_pressed: Vec<sdl2::keyboard::Keycode>, // since last frame
    pub keys_down: Vec<sdl2::keyboard::Keycode>, // currently down, not released
    pub mouse_moved: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl Display {
    pub fn loop_running(&mut self) -> bool {
        self.window.gl_swap_window();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.keys_pressed.clear();
        self.mouse_moved = false;
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {return false;},
                sdl2::event::Event::KeyDown {keycode, ..} => {
                    let key = keycode.unwrap();
                    if !self.keys_pressed.contains(&key) {
                        self.keys_pressed.push(key);
                    }
                    if !self.keys_down.contains(&key) {
                        self.keys_down.push(key);
                    }
                }
                sdl2::event::Event::KeyUp {keycode, ..} => {
                    let key = keycode.unwrap();
                    self.keys_down.retain(|&x| x != key);
                }
                sdl2::event::Event::MouseMotion {x, y, ..} => {
                    self.mouse_moved = true;
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                //sdl2::event::Event::MouseButtonDown {} => {
                //}
                sdl2::event::Event::Window {..} => {
                    let (w, h) = self.window.drawable_size();
                    if w != self.width as u32 || h != self.height as u32 {
                        self.width = w as f32;
                        self.height = h as f32;
                        unsafe {
                            gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
                            // TODO change camera lens settings somehow.
                        }
                    }
                }
                _ => {}, // TODO mouse buttons, relative motion and SDL_GetGlobalMouseState?
            }
        }
        true
    }

    pub fn set_background(&mut self, rgba: &[f32]) {
        unsafe {
            gl::ClearColor(rgba[0], rgba[1], rgba[2], rgba[3]);
        }
    }
} // TODO other functions to change background, w, h near, far etc. put gl stuff in reset fn?

pub fn create(name: &str, w: f32, h: f32) -> Display {
    let res = ::util::resources::from_exe_path().unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(2, 1);
    let window = video_subsystem
        .window(name, w as u32, h as u32)
        .opengl().resizable()
        .build().unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::DepthRangef(0.0, 1.0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::VERTEX_PROGRAM_POINT_SIZE);
        gl::DepthFunc(gl::LESS);
        gl::DepthMask(1);
        gl::CullFace(gl::FRONT);
        gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, 
                                                1, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);
        gl::ColorMask(1, 1, 1, 0);
    }

    Display {
        res: res,
        window: window,
        event_pump: sdl.event_pump().unwrap(),
        sdl: sdl,
        gl_context: gl_context,
        width: w,
        height: h,
        near: 1.0,
        far: 1000.0,
        fov: 45.0,
        keys_pressed: vec![],
        keys_down: vec![],
        mouse_moved: false,
        mouse_x: 0,
        mouse_y: 0,
    }
}