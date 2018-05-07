extern crate ndarray;

use std::f32;
use ::util::vec3;
use ndarray as nda;

pub struct Camera {
    pub eye: nda::Array1<f32>,
    at: nda::Array1<f32>,
    lens: nda::Array1<f32>,
    scale: f32,
    width: f32,
    height: f32,
    is_3d: bool,
    was_moved: bool,
    rotated: bool,
    absolute: bool,
    pub mtrx_made: bool,
    pub mtrx: nda::Array2<f32>,
    rtn: nda::Array2<f32>,
    r_mtrx: nda::Array2<f32>,
    rx: nda::Array2<f32>,
    ry: nda::Array2<f32>,
    rz: nda::Array2<f32>,
    t1: nda::Array2<f32>,
    t2: nda::Array2<f32>,
}

impl Camera {
    pub fn reset(&mut self) {
        let mut view = look_at_matrix(&self.at, &self.eye, &nda::arr1(&[0.0, 1.0, 0.0]));
        let mut projection = if self.is_3d {
          projection_matrix(self.lens[0], self.lens[1], self.lens[2] / self.scale, self.lens[3])
        } else {
          orthographic_matrix(self.scale, self.width, self.height)
        };
        self.mtrx = view.dot(&projection);
        self.was_moved = true;
    }

    pub fn set_lens(&mut self, lens: &nda::Array1<f32>) {
        self.lens = lens.clone();
        self.reset();
    }

    pub fn set_3d(&mut self, is_3d: bool) {
        self.is_3d = is_3d;
        self.reset();
    }

    pub fn set_eye_at(&mut self, eye: &nda::Array1<f32>, at: &nda::Array1<f32>) {
        self.eye = eye.clone();
        self.at = at.clone();
        self.reset();
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.reset();
    }

    pub fn position() {
    }

    pub fn rotate() {
    }

    pub fn make_mtrx(&mut self) {
        if !self.rotated {
            self.make_r_mtrx();
        }
        self.mtrx = self.t2.dot(&self.r_mtrx.dot(&self.t1.dot(&self.mtrx)));
        self.mtrx_made = true;
    }

    fn make_r_mtrx(&mut self) {
        if self.absolute {
            self.r_mtrx = nda::Array::eye(4);
        }
        self.r_mtrx = self.r_mtrx.dot(&self.ry.dot(&self.rx.dot(&self.rz)));
        self.rotated = true;
    }

    pub fn get_matrix(&self) -> &nda::Array2<f32> {
        &self.r_mtrx
    }
}

pub fn create(display: &::display::Display) -> Camera {
    let eye: nda::Array1<f32> = nda::arr1(&[0.0, 0.0, -0.1]);
    let at: nda::Array1<f32> = nda::arr1(&[0.0, 0.0, 0.0]);
    let lens: nda::Array1<f32> = nda::arr1(&[display.near, display.far, display.fov, display.width / display.height]);
    let mut cam = Camera {
        eye: eye,
        at: at,
        lens: lens,
        scale: 1.0,
        width: display.width,
        height: display.height,
        is_3d: true,
        was_moved: true,
        rotated: false,
        absolute: true,
        mtrx_made: true,
        rtn: nda::Array::eye(4),
        mtrx: nda::Array::eye(4),
        r_mtrx: nda::Array::eye(4),
        rx: nda::Array::eye(4),
        ry: nda::Array::eye(4),
        rz: nda::Array::eye(4),
        t1: nda::Array::eye(4),
        t2: nda::Array::eye(4),
    };
    cam.reset();
    cam
}

fn look_at_matrix(at: &nda::Array1<f32>, eye: &nda::Array1<f32>, up: &nda::Array1<f32>) -> nda::Array2<f32> {
    let mut matrix: nda::Array2<f32> = nda::Array::eye(4);
    let zaxis = vec3::norm(&vec3::sub(&at, &eye));     // unit vec direction cam pointing
    let xaxis = vec3::norm(&vec3::cross(&up, &zaxis)); // local horizontal vec
    let yaxis = vec3::cross(&zaxis, &xaxis);           // local vert vec
    for i in 0..3 {
        matrix[[i, 0]] = xaxis[i]; matrix[[i, 1]] = yaxis[i]; matrix[[i, 2]] = zaxis[i];
    } 
    matrix[[3, 0]] = -vec3::dot(&xaxis, &eye); // translations
    matrix[[3, 1]] = -vec3::dot(&yaxis, &eye);
    matrix[[3, 2]] = -vec3::dot(&zaxis, &eye);

    matrix
}

fn projection_matrix(near: f32, far: f32, fov: f32, aspect_ratio: f32) -> nda::Array2<f32> {
    let mut matrix = nda::Array::eye(4);
    if (aspect_ratio != 0.0) && (fov != 0.0) && (near != far) {
        let size = 1.0 / (fov * 0.5).tan();
        matrix[[0, 0]] = size / aspect_ratio;
        matrix[[1, 1]] = size;
        matrix[[2, 2]] = (far + near) / (far - near);
        matrix[[2, 3]] = 1.0;
        matrix[[3, 2]] = -(2.0 * far * near) / (far - near);
    }
    matrix
}

fn orthographic_matrix(scale: f32, width: f32, height: f32) -> nda::Array2<f32> {
    let mut matrix = nda::Array::eye(4);
    if (width != 0.0) && (height != 0.0) {
        matrix[[0, 0]] = 2.0 * scale / width;
        matrix[[1, 1]] = 2.0 * scale / height;
    }
    matrix[[2, 2]] = 2.0 / 10000.0; // TODO use const value rather than magic number
    matrix[[3, 2]] = -1.0;
    matrix[[3, 3]] = 1.0;
    matrix
}

