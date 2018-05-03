extern crate gl;
extern crate ndarray;

use std;
use std::f32;
use gl::types::*;

pub struct Buffer {
    array_buffer: ndarray::Array2<f32>, // TODO unib uniform array
    element_array_buffer: ndarray::Array2<u16>,
    arr_b: GLuint,
    ear_b: GLuint,
    shader_program: ::shader::Program,
}

impl Buffer {
    pub fn draw(&self, matrix: &ndarray::Array2<f32>) {
       //TODO check if no shader has been set and create default, also default light
       // also needs to have access to Shape.unif
        let stride = (6 * std::mem::size_of::<f32>()) as GLint;
        let offset = (3 * std::mem::size_of::<f32>()) as *const GLvoid;
        self.shader_program.set_used();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.arr_b);
            gl::EnableVertexAttribArray(0); // TODO do in loop, also texcoord (list in shader)
            gl::VertexAttribPointer(self.shader_program.get_attribute_location("vertex"), // TODO don't set if location is -1
                          3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(self.shader_program.get_attribute_location("normal"),
                          3, gl::FLOAT, gl::FALSE, stride, offset);
            // TODO unif, unib, tex0, tex1 ... tex7
            gl::UniformMatrix4fv(self.shader_program.get_uniform_location("modelviewmatrix"),
                  1 as GLsizei, 0 as GLboolean, matrix.as_ptr() as *const GLfloat);
            gl::DrawElements(gl::TRIANGLES, self.element_array_buffer.len() as GLsizei, gl::UNSIGNED_SHORT, 0 as *const GLvoid);
        }
    }

    pub fn set_shader(&mut self, shader_program: ::shader::Program) {
        self.shader_program = shader_program;
    }
}

pub fn create(shader_program: ::shader::Program, verts: ndarray::Array2<f32>,
                  norms: ndarray::Array2<f32>, faces: ndarray::Array2<u16>) -> Buffer { //TODO use gl types
    let mut array_buffer: ndarray::Array2<f32> = ndarray::Array::zeros((verts.shape()[0], 6)); //TODO check shapes, calc normals if not right size
    array_buffer.slice_mut(s![.., ..3]).assign(&verts);
    array_buffer.slice_mut(s![.., 3..6]).assign(&norms);
    let element_array_buffer = faces;
    let mut arr_b: GLuint = 0;
    let mut ear_b: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut arr_b);
        gl::BindBuffer(gl::ARRAY_BUFFER, arr_b);
        gl::BufferData(
          gl::ARRAY_BUFFER, (array_buffer.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
          array_buffer.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenBuffers(1, &mut ear_b);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ear_b);
        gl::BufferData(
          gl::ELEMENT_ARRAY_BUFFER, (element_array_buffer.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
          element_array_buffer.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
    }

    Buffer {
        array_buffer: array_buffer,
        element_array_buffer: element_array_buffer,
        arr_b: arr_b,
        ear_b: ear_b,
        shader_program: shader_program,
    }
}
