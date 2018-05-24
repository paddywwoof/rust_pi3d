extern crate ndarray;

use std::f32;
use ::util::vec3;
use ndarray as nd;

pub struct Camera {
    pub eye: nd::Array1<f32>,
    reset_eye: nd::Array1<f32>,
    at: nd::Array1<f32>,
    lens: nd::Array1<f32>,
    scale: f32,
    width: f32,
    height: f32,
    is_3d: bool,
    was_moved: bool,
    rotated: bool,
    absolute: bool,
    pub mtrx_made: bool,
    pub mtrx: nd::Array2<f32>,
    rtn: nd::Array1<f32>,
    r_mtrx: nd::Array2<f32>,
    rx: nd::Array2<f32>,
    ry: nd::Array2<f32>,
    rz: nd::Array2<f32>,
    t1: nd::Array2<f32>,
    t2: nd::Array2<f32>,
}

impl Camera {
    pub fn reset(&mut self) {
        let view = look_at_matrix(&self.at, &self.reset_eye, &nd::arr1(&[0.0, 1.0, 0.0]));
        let projection = if self.is_3d {
            projection_matrix(self.lens[0], self.lens[1], self.lens[2] / self.scale, self.lens[3])
        } else {
            orthographic_matrix(self.scale, self.width, self.height)
        };
        self.mtrx = view.dot(&projection);
        self.was_moved = true;
    }

    pub fn set_lens(&mut self, lens: &nd::Array1<f32>) {
        self.lens = lens.clone();
        self.reset();
    }

    pub fn set_3d(&mut self, is_3d: bool) {
        self.is_3d = is_3d;
        self.reset();
    }

    pub fn set_eye_at(&mut self, eye: &nd::Array1<f32>, at: &nd::Array1<f32>) {
        self.eye = eye.clone();
        self.at = at.clone();
        self.reset();
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.reset();
    }

    pub fn get_direction(&mut self) -> [f32; 3] {
        if !self.rotated {
            self.make_r_mtrx();
        }
        [self.r_mtrx[[0, 2]], self.r_mtrx[[1, 2]], self.r_mtrx[[2, 2]]]
    }

    fn set_rotated_flags(&mut self) {
        self.was_moved = true;
        self.mtrx_made = false;
        self.rotated = false;
    }
    pub fn rotate_to_x(&mut self, a: f32) {
        self.rtn[[0]] = a;
        let c = self.rtn[[0]].cos();
        let s = self.rtn[[0]].sin();
        self.rx[[1, 1]] = c; self.rx[[2, 2]] = c;
        self.rx[[1, 2]] = s; self.rx[[2, 1]] = -s;
        self.set_rotated_flags();
    }
    pub fn rotate_to_y(&mut self, a: f32) {
        self.rtn[[1]] = a;
        let c = self.rtn[[1]].cos();
        let s = self.rtn[[1]].sin();
        self.ry[[0, 0]] = c; self.ry[[2, 2]] = c;
        self.ry[[0, 2]] = -s; self.ry[[2, 0]] = s;
        self.set_rotated_flags();
    }
    pub fn rotate_to_z(&mut self, a: f32) {
        self.rtn[[2]] = a;
        let c = self.rtn[[2]].cos();
        let s = self.rtn[[2]].sin();
        self.rz[[0, 0]] = c; self.rz[[1, 1]] = c;
        self.rz[[0, 1]] = s; self.rz[[1, 0]] = -s;
        self.set_rotated_flags();
    }
    pub fn rotate(&mut self, rot: &[f32; 3]) {
        self.rotate_to_x(rot[0]);
        self.rotate_to_y(rot[1]);
        self.rotate_to_z(rot[2]);
    }

    fn set_moved_flags(&mut self) {
        self.was_moved = true;
        self.mtrx_made = false;
    }
    pub fn position_x(&mut self, pos: f32) {
        self.eye[[0]] = pos;
        self.t2[[3, 0]] = -self.eye[[0]];
        self.set_moved_flags();
    }
    pub fn position_y(&mut self, pos: f32) {
        self.eye[[1]] = pos;
        self.t2[[3, 1]] = -self.eye[[1]];
        self.set_moved_flags();
    }
    pub fn position_z(&mut self, pos: f32) {
        self.eye[[2]] = pos;
        self.t2[[3, 2]] = -self.eye[[2]];
        self.set_moved_flags();
    }
    pub fn position(&mut self, pos: &[f32; 3]) {
        self.eye = nd::arr1(pos);
        self.t2.slice_mut(s![3, ..3]).assign(&(&self.eye * -1.0));
        self.was_moved = true;
        self.mtrx_made = false;
    }
    pub fn offset(&mut self, offs: &[f32; 3]) {
        self.t1.slice_mut(s![3, ..3]).assign(&(nd::arr1(offs) * -1.0));
        self.was_moved = true;
        self.mtrx_made = false;
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
            self.r_mtrx = nd::Array::eye(4);
        }
        self.r_mtrx = self.r_mtrx.dot(&self.ry.dot(&self.rx.dot(&self.rz)));
        self.rotated = true;
    }

    pub fn get_matrix(&self) -> &nd::Array2<f32> {
        &self.r_mtrx
    }
}

pub fn create(display: &::display::Display) -> Camera {
    let eye: nd::Array1<f32> = nd::arr1(&[0.0, 0.0, -0.1]);
    let reset_eye = eye.clone();
    let at: nd::Array1<f32> = nd::arr1(&[0.0, 0.0, 0.0]);
    let lens: nd::Array1<f32> = nd::arr1(&[display.near, display.far, display.fov, display.width / display.height]);
    let mut cam = Camera {
        eye: eye,
        reset_eye: reset_eye,
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
        rtn: nd::arr1(&[0.0, 0.0, 0.0]),
        mtrx: nd::Array::eye(4),
        r_mtrx: nd::Array::eye(4),
        rx: nd::Array::eye(4),
        ry: nd::Array::eye(4),
        rz: nd::Array::eye(4),
        t1: nd::Array::eye(4),
        t2: nd::Array::eye(4),
    };
    cam.reset();
    cam
}

fn look_at_matrix(at: &nd::Array1<f32>, eye: &nd::Array1<f32>, up: &nd::Array1<f32>) -> nd::Array2<f32> {
    let mut matrix: nd::Array2<f32> = nd::Array::eye(4);
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

fn projection_matrix(near: f32, far: f32, fov: f32, aspect_ratio: f32) -> nd::Array2<f32> {
    let mut matrix = nd::Array::eye(4);
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

fn orthographic_matrix(scale: f32, width: f32, height: f32) -> nd::Array2<f32> {
    let mut matrix = nd::Array::eye(4);
    if (width != 0.0) && (height != 0.0) {
        matrix[[0, 0]] = 2.0 * scale / width;
        matrix[[1, 1]] = 2.0 * scale / height;
    }
    matrix[[2, 2]] = 2.0 / 10000.0; // TODO use const value rather than magic number
    matrix[[3, 2]] = -1.0;
    matrix[[3, 3]] = 1.0;
    matrix
}

