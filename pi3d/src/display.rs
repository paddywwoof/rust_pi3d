extern crate gl;
extern crate sdl2;

use gl::types::*;
use std;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

const GL_POINT_SPRITE: GLenum = 0x8861; // needed for NVIDIA driver

pub struct Display {
    sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    _gl_context: sdl2::video::GLContext,
    pub width: f32,
    pub height: f32,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub keys_pressed: Vec<sdl2::keyboard::Keycode>, // since last frame
    pub keys_down: Vec<sdl2::keyboard::Keycode>,    // currently down, not released
    pub mouse_moved: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    mouse_relative: bool,
    start: Instant,
    fps: f32,
    target_frame_tm: u32,
    resized: bool,
}

impl Display {
    pub fn loop_running(&mut self) -> bool {
        let mut del = self.start.elapsed().subsec_nanos();
        if del < self.target_frame_tm {
            let delay = Duration::new(0, (self.target_frame_tm - del) as u32);
            sleep(delay);
            del = self.target_frame_tm;
        }
        self.fps = self.fps * 0.99 + 1e7 / del as f32; // bit of smoothing 1e9 * 0.01
        self.start = Instant::now();
        self.window.gl_swap_window();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.keys_pressed.clear();
        self.mouse_moved = false;
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    return false;
                }
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let key = keycode.unwrap();
                    // ESC is hard coded so that window doesn't get stuck with no cursor
                    if self.mouse_relative && key == sdl2::keyboard::Keycode::Escape {
                        return false;
                    }
                    if !self.keys_pressed.contains(&key) {
                        self.keys_pressed.push(key);
                    }
                    if !self.keys_down.contains(&key) {
                        self.keys_down.push(key);
                    }
                }
                sdl2::event::Event::KeyUp { keycode, .. } => {
                    let key = keycode.unwrap();
                    self.keys_down.retain(|&x| x != key);
                }
                sdl2::event::Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    self.mouse_moved = true;
                    if self.mouse_relative {
                        self.mouse_x += xrel;
                        self.mouse_y += yrel;
                    } else {
                        self.mouse_x = x;
                        self.mouse_y = y;
                    }
                }
                //sdl2::event::Event::MouseButtonDown {} => { //TODO
                //}
                sdl2::event::Event::Window { .. } => {
                    let (w, h) = self.window.drawable_size();
                    if w != self.width as u32 || h != self.height as u32 {
                        self.width = w as f32;
                        self.height = h as f32;
                        unsafe {
                            gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
                            /* to change camera lens settings you need to check Display::was_resized()
                            in the main loop and if true call:
                            Camera.set_lens_from_display(display: &::display::Display)
                            if 2d camera also used it also needs to be set.
                            */
                        }
                        self.resized = true;
                    }
                }
                _ => {}
            }
        }
        true
    }

    pub fn set_background(&mut self, rgba: &[f32]) {
        unsafe {
            gl::ClearColor(rgba[0], rgba[1], rgba[2], rgba[3]);
        }
    }

    pub fn set_mouse_relative(&mut self, mode: bool) {
        self.sdl.mouse().set_relative_mouse_mode(mode);
    }

    pub fn set_target_fps(&mut self, target_fps: f32) {
        self.target_frame_tm = if target_fps < 1.0 {
            99999999 // min speed 999 ms per frame
        } else if target_fps < 1e9 {
            1000000000 / target_fps as u32
        } else {
            1
        }; // max speed 1ns per frame
    }

    pub fn fps(&mut self) -> f32 {
        self.fps
    }

    pub fn was_resized(&mut self) -> bool {
        let previous_value = self.resized;
        self.resized = false; // calling this method resets flag
        previous_value
    }

    pub fn get_size(&mut self) -> (f32, f32) {
        (self.width.clone(), self.height.clone())
    }

    pub fn set_fullscreen(&mut self, on: bool) {
        match self.window.set_fullscreen(if on {
            sdl2::video::FullscreenType::Desktop
        } else {
            sdl2::video::FullscreenType::Off
        }) {
            Ok(_) => {
                let (w, h) = self.window.drawable_size();
                unsafe {
                    gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
                }
                self.resized = true;
                self.width = w as f32;
                self.height = h as f32;
            }
            Err(e) => {
                println!("Error toggleing fullscreen - {:?}", e);
            }
        }
    }

    pub fn set_opacity(&mut self, alpha: f32) {
        self.window.set_opacity(alpha).unwrap();
    }
} // TODO other functions to change background, w, h near, far etc. put gl stuff in reset fn?

pub fn create(
    name: &str,
    width: f32,
    height: f32,
    profile: &str,
    major: u8,
    minor: u8,
) -> Result<Display, Box<dyn Error>> {
    let sdl = sdl2::init()?; //.unwrap();
    let video_subsystem = sdl.video()?; //.unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(match profile {
        "GLES" => sdl2::video::GLProfile::GLES,
        _ => sdl2::video::GLProfile::Core,
    });
    gl_attr.set_context_version(major, minor);
    gl_attr.set_double_buffer(true);
    let window = video_subsystem
        .window(name, width as u32, height as u32)
        .opengl()
        .resizable()
        .build()?; //.unwrap();
    let _gl_context = window.gl_create_context()?; //.unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);
        gl::DepthRangef(0.0, 1.0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::VERTEX_PROGRAM_POINT_SIZE);
        gl::Enable(gl::PROGRAM_POINT_SIZE);
        gl::Enable(GL_POINT_SPRITE);
        gl::DepthFunc(gl::LESS);
        gl::DepthMask(1);
        gl::BlendFuncSeparate(
            gl::SRC_ALPHA,
            gl::ONE_MINUS_SRC_ALPHA,
            1,
            gl::ONE_MINUS_SRC_ALPHA,
        );
        gl::Enable(gl::BLEND);
        gl::ColorMask(1, 1, 1, 0);
    }

    Ok(Display {
        window,
        event_pump: sdl.event_pump()?, //.unwrap(),
        sdl,
        _gl_context,
        width,
        height,
        near: 1.0,
        far: 1000.0,
        fov: 1.0,
        keys_pressed: vec![],
        keys_down: vec![],
        mouse_moved: false,
        mouse_x: 0,
        mouse_y: 0,
        mouse_relative: true,
        start: Instant::now(),
        fps: 0.0,
        target_frame_tm: 20000000, //ns -> 50fps default target
        resized: false,
    })
}
