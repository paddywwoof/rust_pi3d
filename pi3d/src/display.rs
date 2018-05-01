extern crate sdl2;
extern crate gl;
extern crate ndarray;

use std;

use ::util::resources::Resources;

pub struct Display {
    pub res: Resources,
    sdl: sdl2::Sdl,
    //video_sys: sdl2::VideoSubsystem,
    //gl_attr: sdl2::video::gl_attr::GLAttr,
    pub window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl Display {
    pub fn loop_running(&mut self) -> bool {
        self.window.gl_swap_window();
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {return false;},
                _ => {},
            }
        }
        true
    }
}

pub fn create(name: &str) -> Display {
    let res = Resources::from_exe_path().unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(2, 1);
    let window = video_subsystem
        .window(name, 900, 900)
        .opengl().resizable()
        .build().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 900, 900);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::DepthRangef(0.0, 1.0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
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
        //gl_attr: gl_attr,
        window: window,
        event_pump: sdl.event_pump().unwrap(),
        sdl: sdl,
        //video_sys: video_subsystem,
    }
}
