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
    draw_method: GLenum,
}

impl Buffer {
    pub fn draw(&self, matrix: &nda::Array3<f32>, shape: &::shape::Shape) {
        //TODO check if no shader has been set and create default, also default light
        self.shader_program.set_used();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.arr_b);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ear_b);
            for (i, v) in self.shader_program.get_attribute_list().iter().enumerate() {
                if *v > -1 {
                    gl::EnableVertexAttribArray(*v as GLuint);
                    gl::VertexAttribPointer(*v as GLuint, 3, gl::FLOAT, gl::FALSE,
                                          self.stride, (i * 12) as *const GLvoid);
                }
            }
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
            gl::DrawElements(self.draw_method, self.element_array_buffer.len() as GLsizei, gl::UNSIGNED_SHORT, 0 as *const GLvoid);
        }
    }

    pub fn set_shader(&mut self, shader_program: ::shader::Program) {
        self.shader_program = shader_program;
    }

    pub fn set_textures(&mut self, textures: &Vec<GLuint>) {
        self.textures = textures.clone();
    }

    pub fn set_draw_details(&mut self, shader_program: ::shader::Program,
        textures: &Vec<GLuint>, ntiles: f32, shiny: f32, umult: f32,
        vmult:f32, bump_factor: f32) {
        self.shader_program = shader_program;
        self.textures = textures.clone();
        self.unib[[0, 0]] = ntiles;
        self.unib[[0, 1]] = shiny;
        self.unib[[3, 0]] = umult;
        self.unib[[3, 1]] = vmult;
        self.unib[[3, 2]] = bump_factor;
    }

    pub fn set_point_size(&mut self, point_size: f32) {
        self.unib[[2, 2]] = point_size;
        self.draw_method = if point_size > 0.0 {gl::POINTS} else {gl::TRIANGLES};
    }

    pub fn set_line_width(&mut self, line_width: f32, strip: bool, closed: bool) {
        self.unib[[3, 2]] = line_width;
        unsafe {gl::LineWidth(line_width);}
        self.draw_method =  if line_width > 0.0 {
                                if strip {
                                    if closed {gl::LINE_LOOP} else {gl::LINE_STRIP}
                                } else {gl::LINES}
                            } else {gl::TRIANGLES};
    }
}

pub fn create(shader_program: ::shader::Program, verts: nda::Array2<f32>,
                  norms: nda::Array2<f32>, texcoords: nda::Array2<f32>,
                  faces: nda::Array2<u16>, calc_norms: bool) -> Buffer {
    let mut stride: GLint = 32; // default vertex, normal and texcoords
    let mut bufw = 8;
    if texcoords.shape()[0] != verts.shape()[0] {
        if (norms.shape()[0] != verts.shape()[0]) && !calc_norms { // just use vertex
            stride = 12;
            bufw = 3;
        } else { // use vertex and normal
            stride = 24;
            bufw = 6;
        }
    }
    let mut array_buffer: nda::Array2<f32> = nda::Array::zeros((verts.shape()[0], bufw));
    //println!("{:?}", array_buffer.len());
    array_buffer.slice_mut(s![.., ..3]).assign(&verts);
    if bufw > 3 {
        if calc_norms {
            calc_normals(&mut array_buffer, &faces);
        } else {
            array_buffer.slice_mut(s![.., 3..6]).assign(&norms);
        }
        if bufw > 6 {
            array_buffer.slice_mut(s![.., 6..8]).assign(&texcoords);
        }
    }
    //println!("{:?}", array_buffer[[23, 7]]);
    let element_array_buffer = faces;
    let mut arr_b: GLuint = 0;
    let mut ear_b: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut arr_b);
        gl::BindBuffer(gl::ARRAY_BUFFER, arr_b);
        gl::BufferData(
          gl::ARRAY_BUFFER, ((array_buffer.len() + 1) * std::mem::size_of::<f32>()) as GLsizeiptr,
          //TODO why does it need this extra float 'padding' on the end of the size?
          array_buffer.as_ptr() as *const GLvoid, gl::DYNAMIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenBuffers(1, &mut ear_b);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ear_b);
        gl::BufferData(
          gl::ELEMENT_ARRAY_BUFFER, (element_array_buffer.len() * std::mem::size_of::<u16>()) as GLsizeiptr,
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
        draw_method: gl::TRIANGLES,
    }
}

fn calc_normals(a_b: &mut nda::Array2<f32>, e_a_b: &nda::Array2<u16>) {
    // update array_buffer in place TODO
    let n_elements = e_a_b.shape()[0];
    for i in 0..n_elements {
        for j in 0..3 { // for each corner of element
            let u: usize = (j + 1) % 3;
            let v: usize = (j + 2) % 3;
            for k in 0..3 { // for each component of vector
                let x: usize = (k + 1) % 3;
                let y: usize = (k + 2) % 3;
                a_b[[e_a_b[[i, j]] as usize, k + 3]] += ( // cross product
                         a_b[[e_a_b[[i, u]] as usize, x]] - a_b[[e_a_b[[i, j]] as usize, x]]
                        ) * (
                         a_b[[e_a_b[[i, v]] as usize, y]] - a_b[[e_a_b[[i, j]] as usize, y]]
                        ) - (
                         a_b[[e_a_b[[i, u]] as usize, y]] - a_b[[e_a_b[[i, j]] as usize, y]]
                        ) * (
                         a_b[[e_a_b[[i, v]] as usize, x]] - a_b[[e_a_b[[i, j]] as usize, x]]
                        );
            }
        }
    } // now normalize
    let n = a_b.shape()[0];
    for i in 0..n { // normal values in array_buffer[[.., 3..6]]
        let len: f32 = a_b.slice(s![i, 3..6]).iter().map(|x| x * x).sum();
        if len > 0.0 {
            let len_inv = 1.0 / len.sqrt();
            for j in 3..6 {a_b[[i, j]] *= len_inv;}
        }
    }
}

