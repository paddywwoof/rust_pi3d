extern crate gl;
extern crate ndarray;

use std;
use std::f32;
use gl::types::*;

pub struct Shape {
    unif: ndarray::Array2<f32>,
    pub buf: Vec<::buffer::Buffer>,
    rox: ndarray::Array2<f32>,
    roy: ndarray::Array2<f32>,
    roz: ndarray::Array2<f32>,
    tr1: ndarray::Array2<f32>,
}

impl Shape {
    pub fn draw(&self) {
        let matrix = self.roy.dot(&self.rox.dot(&self.roz.dot(&self.tr1)));
        for i in 0..self.buf.len() {
            self.buf[i].draw(&matrix);
        }
    }
    pub fn set_shader(&mut self, shader_program: &::shader::Program) {
        for i in 0..self.buf.len() {
            self.buf[i].set_shader(shader_program.clone());
        }
    }
    pub fn rotate_inc_x(&mut self, da: f32) {
        self.unif[[1, 0]] += da;
        let c = self.unif[[1, 0]].cos();
        let s = self.unif[[1, 0]].sin();
        self.rox[[1, 1]] = c; self.rox[[2, 2]] = c;
        self.rox[[1, 2]] = s; self.rox[[2, 1]] = -s;
    }
    pub fn rotate_inc_y(&mut self, da: f32) {
        self.unif[[1, 1]] += da;
        let c = self.unif[[1, 1]].cos();
        let s = self.unif[[1, 1]].sin();
        self.roy[[0, 0]] = c; self.roy[[2, 2]] = c;
        self.roy[[0, 2]] = s; self.roy[[2, 0]] = -s;
    }
    pub fn rotate_inc_z(&mut self, da: f32) {
        self.unif[[1, 2]] += da;
        let c = self.unif[[1, 2]].cos();
        let s = self.unif[[1, 2]].sin();
        self.roz[[0, 0]] = c; self.roz[[1, 1]] = c;
        self.roz[[0, 1]] = s; self.roz[[1, 0]] = -s;
    }
    pub fn position_x(&mut self, pos: f32) {
        self.unif[[0, 0]] = pos;
        self.tr1[[3, 0]] = self.unif[[0, 0]];
    }
    pub fn position_y(&mut self, pos: f32) {
        self.unif[[0, 1]] = pos;
        self.tr1[[3, 1]] = self.unif[[0, 1]];
    }
    pub fn position_z(&mut self, pos: f32) {
        self.unif[[0, 2]] = pos;
        self.tr1[[3, 2]] = self.unif[[0, 2]];
    }
}

pub fn create(buf: Vec<::buffer::Buffer>) -> Shape {
    Shape {
        unif: ndarray::arr2(&[[0.0, 0.0, 0.0],
                              [0.0, 0.0, 0.0],
                              [1.0, 1.0, 1.0],
                              [0.0, 0.0, 0.0]]),
        buf: buf,
        rox: ndarray::Array::eye(4),
        roy: ndarray::Array::eye(4),
        roz: ndarray::Array::eye(4),
        tr1: ndarray::Array::eye(4),
    }
}


pub fn cuboid(w: f32, h: f32, d: f32) -> Shape {
    let wh = w * 0.5; let hh = h * 0.5; let dh = d * 0.5;
    let verts: ndarray::Array2<f32> = ndarray::arr2(
                                &[[-wh, -hh, -dh],
                                   [wh, -hh, -dh],
                                   [wh,  hh, -dh],
                                  [-wh,  hh, -dh],
                                  [-wh, -hh,  dh],
                                   [wh, -hh,  dh],
                                   [wh,  hh,  dh],
                                  [-wh,  hh,  dh]]);
    let norms: ndarray::Array2<f32> = ndarray::arr2(
                                &[[1.0, 0.0, 0.0],
                                  [0.0, 1.0, 0.0],
                                  [0.0, 0.0, 1.0],
                                  [1.0, 0.0, 1.0],
                                  [1.0, 1.0, 0.0],
                                  [0.0, 1.0, 1.0],
                                  [1.0, 1.0, 1.0],
                                  [0.0, 0.0, 1.0]]);
    let faces: ndarray::Array2<u16> = ndarray::arr2(
                                &[[0, 3, 2], [0, 2, 1],
                                  [4, 7, 3], [4, 3, 0],
                                  [0, 1, 5], [0, 5, 4],
                                  [1, 2, 6], [1, 6, 5],
                                  [5, 6, 7], [5, 7, 4],
                                  [3, 7, 6], [3, 6 ,2]]);
    let mut new_buffer = ::buffer::create(::shader::Program::new(), verts, norms, faces);
    create(vec![new_buffer])
}
