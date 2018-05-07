extern crate gl;
extern crate ndarray;

use std;
use std::f32;
use gl::types::*;
use ndarray as nda;

pub struct Shape {
    pub unif: nda::Array2<f32>,
    pub buf: Vec<::buffer::Buffer>,
    rox: nda::Array2<f32>,
    roy: nda::Array2<f32>,
    roz: nda::Array2<f32>,
    tr1: nda::Array2<f32>,
    pub m_flag: bool,
    pub matrix: nda::Array3<f32>,
}

impl Shape {
    pub fn draw(&mut self, camera: &mut ::camera::Camera) {
        if !camera.mtrx_made {
            camera.make_mtrx();
        }
        self.unif.slice_mut(s![6, ..]).assign(&camera.eye);
        let m = &self.roy.dot(&self.rox.dot(&self.roz.dot(&self.tr1)));
        self.matrix.slice_mut(s![0, .., ..]).assign(m);
        self.matrix.slice_mut(s![1, .., ..]).assign(&m.dot(&camera.mtrx));
        for i in 0..self.buf.len() {
            self.buf[i].draw(&self.matrix, &self);
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
    //pub fn position(&mut self, pos: &nda::Array1<f32>) {
    //    sel
    //}
    pub fn set_light(&mut self, num: usize, posn: &[f32],
                    rgb: &[f32], amb: &[f32], point: bool) {
        self.unif[[7, num]] = if point {1.0} else {0.0};
        self.unif.slice_mut(s![8 + num * 2, ..]).assign(&nda::arr1(posn));
        self.unif.slice_mut(s![9 + num * 2, ..]).assign(&nda::arr1(rgb));
        self.unif.slice_mut(s![10 + num * 2, ..]).assign(&nda::arr1(amb));
    }
}

pub fn create(buf: Vec<::buffer::Buffer>) -> Shape {
    Shape {
        unif: nda::arr2(&[
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
        rox: nda::Array::eye(4),
        roy: nda::Array::eye(4),
        roz: nda::Array::eye(4),
        tr1: nda::Array::eye(4),
        m_flag: true,
        matrix: nda::Array::zeros((3, 4, 4)),
    }
}


