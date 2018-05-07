extern crate gl;
extern crate ndarray;

use std;
use std::f32;
use gl::types::*;
use ndarray as nda;

pub struct Buffer {
    unib: nda::Array2<f32>,
    array_buffer: nda::Array2<f32>,
    element_array_buffer: nda::Array2<u16>,
    arr_b: GLuint,
    ear_b: GLuint,
    shader_program: ::shader::Program,
    stride: GLint,
    textures: Vec<GLuint>,
}

impl Buffer {
    pub fn draw(&self, matrix: &nda::Array3<f32>, shape: &::shape::Shape) {
        //TODO check if no shader has been set and create default, also default light
        self.shader_program.set_used();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.arr_b);
            for (i, v) in self.shader_program.get_attribute_list().iter().enumerate() {
                if *v > -1 {
                    gl::EnableVertexAttribArray(*v as GLuint);
                    gl::VertexAttribPointer(*v as GLuint, 3, gl::FLOAT, gl::FALSE,
                                          self.stride, (i * 12) as *const GLvoid);
                }
            }
            // TODO unif, unib, tex0, tex1 ... tex7
            gl::UniformMatrix4fv(self.shader_program.get_uniform_location("modelviewmatrix\0"),
                3 as GLsizei, 0 as GLboolean, matrix.as_ptr() as *const GLfloat);
            gl::Uniform3fv(self.shader_program.get_uniform_location("unif\0"),
                20 as GLsizei, shape.unif.as_ptr() as *const GLfloat);
            gl::Uniform3fv(self.shader_program.get_uniform_location("unib\0"),
                4 as GLsizei, self.unib.as_ptr() as *const GLfloat);
            for (i, tex) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                //assert texture.tex(), 'There was an empty texture in your Buffer.'
                gl::BindTexture(gl::TEXTURE_2D, *tex);
                gl::Uniform1i(self.shader_program.get_uniform_location(&format!("tex{}\0", i)),
                    i as GLint);
            }

            // finally draw it TODO gl::TRANGLES should be variable lines or points
            gl::DrawElements(gl::TRIANGLES, self.element_array_buffer.len() as GLsizei, gl::UNSIGNED_SHORT, 0 as *const GLvoid);
        }
    }

    pub fn set_shader(&mut self, shader_program: ::shader::Program) {
        self.shader_program = shader_program;
    }

    pub fn set_textures(&mut self, textures: &Vec<GLuint>) {
        self.textures = textures.clone();
    }
}

pub fn create(shader_program: ::shader::Program, verts: nda::Array2<f32>,
                  norms: nda::Array2<f32>, texcoords: nda::Array2<f32>,
                  faces: nda::Array2<u16>) -> Buffer {
    let mut stride: GLint = 32; // default vertex, normal and texcoords
    let mut bufw = 8;
    if texcoords.shape()[0] != verts.shape()[0] {
        if norms.shape()[0] != verts.shape()[0] { // just use vertex
            stride = 12;
            bufw = 3;
        } else { // use vertex and normal
            stride = 24;
            bufw = 6;
        }
    }
        
    let mut array_buffer: nda::Array2<f32> = nda::Array::zeros((verts.shape()[0], bufw));
    array_buffer.slice_mut(s![.., ..3]).assign(&verts);
    if bufw > 3 {
        array_buffer.slice_mut(s![.., 3..6]).assign(&norms); //TODO pass calc normals flag to this function and make function to do it
        if bufw > 6 {
            array_buffer.slice_mut(s![.., 6..8]).assign(&texcoords);
        }
    }
    let element_array_buffer = faces;
    let mut arr_b: GLuint = 0;
    let mut ear_b: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut arr_b);
        gl::BindBuffer(gl::ARRAY_BUFFER, arr_b);
        gl::BufferData(
          gl::ARRAY_BUFFER, (array_buffer.len() * 4) as GLsizeiptr, // TODO, does size of f32 ever vary from 4 bytes?
          array_buffer.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenBuffers(1, &mut ear_b);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ear_b);
        gl::BufferData(
          gl::ELEMENT_ARRAY_BUFFER, (element_array_buffer.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
          element_array_buffer.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
    }

    Buffer {
        unib: nda::arr2(&[[0.0, 0.0, 0.0],  //00 ntile, shiny, blend
                          [0.5, 0.5, 0.5],  //01 material RGB
                          [1.0, 1.0, 0.0],  //02 umult, vmult, point_size
                          [0.0, 0.0, 1.0]]),//03 u_off, v_off, line_width/bump
        array_buffer: array_buffer,
        element_array_buffer: element_array_buffer,
        arr_b: arr_b,
        ear_b: ear_b,
        shader_program: shader_program,
        stride: stride,
        textures: vec![],
    }
}
