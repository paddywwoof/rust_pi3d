extern crate sdl2;
extern crate gl;
extern crate ndarray;
extern crate pi3d;

use std::f32;
use gl::types::*;
use pi3d::util::resources::Resources;

fn main() {
    let res = Resources::from_exe_path().unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(2, 1);
    let window = video_subsystem
        .window("experimental window", 900, 900)
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


    let shader_program = pi3d::shader::Program::from_res(
          &res, "shaders/triangle").unwrap();
    shader_program.set_used();
    let vertices: Vec<f32> = vec![-0.25, -0.25, -0.25, 1.0, 0.0, 0.0,
                                   0.25, -0.25, -0.25, 0.0, 1.0, 0.0,
                                   0.25,  0.25, -0.25, 0.0, 0.0, 1.0,
                                  -0.25,  0.25, -0.25, 1.0, 0.0, 1.0,
                                  -0.25, -0.25,  0.25, 1.0, 1.0, 0.0,
                                   0.25, -0.25,  0.25, 0.0, 1.0, 1.0,
                                   0.25,  0.25,  0.25, 1.0, 1.0, 1.0,
                                  -0.25,  0.25,  0.25, 0.0, 0.0, 1.0];
    let faces: Vec<GLushort> = vec![0, 3, 2, 0, 2, 1,
                                  4, 7, 3, 4, 3, 0,
                                  0, 1, 5, 0, 5, 4,
                                  1, 2, 6, 1, 6, 5,
                                  5, 6, 7, 5, 7, 4,
                                  3, 7, 6, 3, 6 ,2];
    let mut vbo: GLuint = 0;
    let mut eab: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
        gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                    vertices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenBuffers(1, &mut eab);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, eab);
        gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER, (faces.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                    faces.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
    }
    let stride = (6 * std::mem::size_of::<f32>()) as GLint;
    let offset = (3 * std::mem::size_of::<f32>()) as *const GLvoid;
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(shader_program.get_attribute_location("vertex"), 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(shader_program.get_attribute_location("normal"), 3, gl::FLOAT, gl::FALSE, stride, offset);
    }

    let mut rotx = ndarray::Array::eye(4);
    let mut roty = ndarray::Array::eye(4);
    let mut rotz = ndarray::Array::eye(4);
    let mut disp = ndarray::Array::eye(4);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut angle: f32 = 0.0;
    
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        angle += 0.02;
        let c = angle.cos();
        let s = angle.sin();
        rotx[[1, 1]] = c; rotx[[2, 2]] = c; rotx[[1, 2]] = s; rotx[[2, 1]] = -s;
        roty[[0, 0]] = c; roty[[2, 2]] = c; roty[[0, 2]] = s; roty[[2, 0]] = -s;
        rotz[[0, 0]] = c; rotz[[1, 1]] = c; rotz[[0, 1]] = s; rotz[[1, 0]] = -s;
        disp[[3, 0]] = angle * 0.087 % 2.2 - 1.1;
        disp[[3, 1]] = angle * 0.12 % 1.98 - 0.9;
        disp[[3, 2]] = 0.5;
        let matrix = roty.dot(&rotx.dot(&rotz.dot(&disp)));
        unsafe { // Shape, Buffer draw
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UniformMatrix4fv(shader_program.get_uniform_location("modelviewmatrix"), 1 as GLsizei, 0 as GLboolean, // 3 in pi3d
                  matrix.as_ptr() as *const GLfloat);

            gl::DrawElements(gl::TRIANGLES, faces.len() as GLsizei, gl::UNSIGNED_SHORT, 0 as *const GLvoid);
        }
        window.gl_swap_window();
    }
}
