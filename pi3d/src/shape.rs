extern crate gl;
extern crate ndarray;

use std::f32;
use ndarray as nd;
use gl::types::*;

pub struct Shape {
    pub unif: nd::Array2<f32>,
    pub buf: Vec<::buffer::Buffer>,
    tr1: nd::Array2<f32>, //TODO offset and scale matrices
    rox: nd::Array2<f32>,
    roy: nd::Array2<f32>,
    roz: nd::Array2<f32>,
    scl: nd::Array2<f32>,
    tr2: nd::Array2<f32>,
    pub m_flag: bool,
    pub matrix: nd::Array3<f32>,
}

impl Shape {
    pub fn draw(&mut self, camera: &mut ::camera::Camera) {
        if !camera.mtrx_made {
            camera.make_mtrx();
        }
        self.unif.slice_mut(s![6, ..]).assign(&camera.eye);
        let m = &self.tr2.dot(
                 &self.scl.dot(
                  &self.roy.dot(
                   &self.rox.dot(
                    &self.roz.dot(
                     &self.tr1)))));
        self.matrix.slice_mut(s![0, .., ..]).assign(m);
        self.matrix.slice_mut(s![1, .., ..]).assign(&m.dot(&camera.mtrx));
        for i in 0..self.buf.len() {
            self.buf[i].draw(&self.matrix, &self);
        }
    }

    pub fn set_shader(&mut self, shader_program: &::shader::Program) {
        for i in 0..self.buf.len() {
            self.buf[i].set_shader(&shader_program.clone());
        }
    }
    pub fn set_draw_details(&mut self, shader_program: &::shader::Program,
        textures: &Vec<GLuint>, ntiles: f32, shiny: f32, umult: f32,
        vmult:f32, bump_factor: f32) {
        for i in 0..self.buf.len() {
            self.buf[i].set_draw_details(&shader_program.clone(), textures,
                ntiles, shiny, umult, vmult, bump_factor);
        }
    }
    pub fn set_material(&mut self, material: &[f32]) {
        for i in 0..self.buf.len() {
            self.buf[i].set_material(&material);
        }
        if material.len() > 3 {
            self.unif[[5, 2]] = material[3];
        }
    }

    pub fn rotate_inc_x(&mut self, da: f32) {
        let a = self.unif[[1, 0]] + da;
        self.rotate_to_x(a);
    }
    pub fn rotate_inc_y(&mut self, da: f32) {
        let a = self.unif[[1, 1]] + da;
        self.rotate_to_y(a);
    }
    pub fn rotate_inc_z(&mut self, da: f32) {
        let a = self.unif[[1, 2]] + da;
        self.rotate_to_z(a);
    }
    pub fn rotate_to_x(&mut self, a: f32) {
        self.unif[[1, 0]] = a;
        let c = self.unif[[1, 0]].cos();
        let s = self.unif[[1, 0]].sin();
        self.rox[[1, 1]] = c; self.rox[[2, 2]] = c;
        self.rox[[1, 2]] = s; self.rox[[2, 1]] = -s;
    }
    pub fn rotate_to_y(&mut self, a: f32) {
        self.unif[[1, 1]] = a;
        let c = self.unif[[1, 1]].cos();
        let s = self.unif[[1, 1]].sin();
        self.roy[[0, 0]] = c; self.roy[[2, 2]] = c;
        self.roy[[0, 2]] = s; self.roy[[2, 0]] = -s;
    }
    pub fn rotate_to_z(&mut self, a: f32) {
        self.unif[[1, 2]] = a;
        let c = self.unif[[1, 2]].cos();
        let s = self.unif[[1, 2]].sin();
        self.roz[[0, 0]] = c; self.roz[[1, 1]] = c;
        self.roz[[0, 1]] = s; self.roz[[1, 0]] = -s;
    }

    pub fn position_x(&mut self, pos: f32) {
        self.unif[[0, 0]] = pos;
        self.tr1[[3, 0]] = self.unif[[0, 0]] - self.unif[[3, 0]];
    }
    pub fn position_y(&mut self, pos: f32) {
        self.unif[[0, 1]] = pos;
        self.tr1[[3, 1]] = self.unif[[0, 1]] - self.unif[[3, 1]];
    }
    pub fn position_z(&mut self, pos: f32) {
        self.unif[[0, 2]] = pos;
        self.tr1[[3, 2]] = self.unif[[0, 2]] - self.unif[[3, 2]];
    }
    pub fn position(&mut self, pos: &[f32; 3]) {
        self.position_x(pos[0]);
        self.position_y(pos[1]);
        self.position_z(pos[2]);
    }
    pub fn offset(&mut self, offs: &[f32; 3]) {
        self.unif.slice_mut(s![3, ..]).assign(&nd::arr1(offs));
        self.tr2.slice_mut(s![3, ..3]).assign(&nd::arr1(offs));
    }
    pub fn scale(&mut self, scale: &[f32; 3]) {
        self.unif.slice_mut(s![2, ..]).assign(&nd::arr1(scale));
        self.scl.slice_mut(s![3, ..3]).assign(&nd::arr1(scale));
    }

    pub fn set_light(&mut self, num: usize, posn: &[f32],
                    rgb: &[f32], amb: &[f32], point: bool) {
        self.unif[[7, num]] = if point {1.0} else {0.0};
        self.unif.slice_mut(s![8 + num * 2, ..]).assign(&nd::arr1(posn));
        self.unif.slice_mut(s![9 + num * 2, ..]).assign(&nd::arr1(rgb));
        self.unif.slice_mut(s![10 + num * 2, ..]).assign(&nd::arr1(amb));
    }
}

pub fn create(buf: Vec<::buffer::Buffer>) -> Shape {
    Shape {
        unif: nd::arr2(&[
                [0.0, 0.0, 0.0], //00 location
                [0.0, 0.0, 0.0], //01 rotation
                [1.0, 1.0, 1.0], //02 scale
                [0.0, 0.0, 0.0], //03 offset
                [0.4, 0.4, 0.6], //04 fog shade
                [10.0, 0.6, 1.0], //05 fog dist, fog alpha, shape alpha
                [0.0, 0.0, -0.1], //06 camera position (eye location) TODO pick up from camera default
                [0.0, 0.0, 0.0], //07 point light flags: light0, light1, unused
                [10.0, -10.0, -5.0], //08 light0 position or direction vector
                [1.0, 1.0, 1.0], //09 light0 RGB strength
                [0.1, 0.1, 0.2], //10 light0 ambient RBG strength
                [0.0, 0.0, 0.0], //11 light1 position or direction vector - TODO shaders to use light > 0
                [0.0, 0.0, 0.0], //12 light1 RGB strength
                [0.0, 0.0, 0.0], //13 light1 ambient RBG strength
                [0.0, 0.0, 0.0], //14 defocus [dist from, dist to, amount] also 2D x, y
                [0.0, 0.0, 0.0], //15 defocus [blur width, blur height, unused] also 2D w, h, tot_h
                [0.0, 0.0, 0.0], //16 available for custom shaders
                [0.0, 0.0, 0.0], //17 available
                [0.0, 0.0, 0.0], //18 available
                [0.0, 0.0, 0.0], //19 available
                ]),//
        buf: buf,
        tr1: nd::Array::eye(4),
        rox: nd::Array::eye(4),
        roy: nd::Array::eye(4),
        roz: nd::Array::eye(4),
        scl: nd::Array::eye(4),
        tr2: nd::Array::eye(4),
        m_flag: true,
        matrix: nd::Array::zeros((3, 4, 4)),
    }
}


