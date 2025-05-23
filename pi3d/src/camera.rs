use ndarray as nd;
use std::cell::RefCell;
use std::f32;
use std::rc::Rc;
use crate::util::vec3;
use crate::display;

pub struct CameraInternals {
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

impl CameraInternals {
    pub fn reset(&mut self) {
        let view = look_at_matrix(&self.at, &self.reset_eye, &nd::arr1(&[0.0, 1.0, 0.0]));
        let projection = if self.is_3d {
            projection_matrix(
                self.lens[0],
                self.lens[1],
                self.lens[2] / self.scale,
                self.lens[3],
            )
        } else {
            orthographic_matrix(self.scale, self.width, self.height)
        };
        self.mtrx = view.dot(&projection);
        self.was_moved = true;
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

    pub fn set_lens_from_display(&mut self, display: &display::Display) {
        self.lens = nd::arr1(&[
            display.near,
            display.far,
            display.fov,
            display.width as f32 / display.height as f32,
        ]);
        self.width = display.width as f32;
        self.height = display.height as f32;
        self.reset();
    }
}

pub struct Camera {
    pub cam: Rc<RefCell<CameraInternals>>,
}

impl Camera {
    pub fn reset(&mut self) {
        self.cam.borrow_mut().reset();
    }

    pub fn set_lens_from_display(&mut self, display: &display::Display) {
        self.cam.borrow_mut().set_lens_from_display(display);
    }

    pub fn set_lens(&mut self, lens: &nd::Array1<f32>) {
        let mut cam = self.cam.borrow_mut();
        cam.lens = lens.clone();
        cam.reset();
    }

    pub fn set_3d(&mut self, is_3d: bool) {
        let mut cam = self.cam.borrow_mut();
        cam.is_3d = is_3d;
        cam.reset();
    }

    pub fn set_eye_at(&mut self, eye: &nd::Array1<f32>, at: &nd::Array1<f32>) {
        let mut cam = self.cam.borrow_mut();
        cam.eye = eye.clone();
        cam.at = at.clone();
        cam.reset();
    }

    pub fn set_scale(&mut self, scale: f32) {
        let mut cam = self.cam.borrow_mut();
        cam.scale = scale;
        cam.reset();
    }

    pub fn set_absolute(&mut self, absolute: bool) {
        let mut cam = self.cam.borrow_mut();
        cam.absolute = absolute;
    }

    pub fn get_direction(&mut self) -> [f32; 3] {
        let mut cam = self.cam.borrow_mut();
        if !cam.rotated {
            cam.make_r_mtrx();
        }
        [cam.r_mtrx[[0, 2]], cam.r_mtrx[[1, 2]], cam.r_mtrx[[2, 2]]]
    }

    fn set_rotated_flags(&mut self) {
        let mut cam = self.cam.borrow_mut();
        cam.was_moved = true;
        cam.mtrx_made = false;
        cam.rotated = false;
    }
    pub fn rotate_to_x(&mut self, a: f32) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.rtn[[0]] = a;
            let c = cam.rtn[[0]].cos();
            let s = cam.rtn[[0]].sin();
            cam.rx[[1, 1]] = c;
            cam.rx[[2, 2]] = c;
            cam.rx[[1, 2]] = s;
            cam.rx[[2, 1]] = -s;
        }
        self.set_rotated_flags();
    }
    pub fn rotate_to_y(&mut self, a: f32) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.rtn[[1]] = a;
            let c = cam.rtn[[1]].cos();
            let s = cam.rtn[[1]].sin();
            cam.ry[[0, 0]] = c;
            cam.ry[[2, 2]] = c;
            cam.ry[[0, 2]] = -s;
            cam.ry[[2, 0]] = s;
        }
        self.set_rotated_flags();
    }
    pub fn rotate_to_z(&mut self, a: f32) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.rtn[[2]] = a;
            let c = cam.rtn[[2]].cos();
            let s = cam.rtn[[2]].sin();
            cam.rz[[0, 0]] = c;
            cam.rz[[1, 1]] = c;
            cam.rz[[0, 1]] = s;
            cam.rz[[1, 0]] = -s;
        }
        self.set_rotated_flags();
    }
    pub fn rotate(&mut self, rot: &[f32; 3]) {
        self.rotate_to_x(rot[0]);
        self.rotate_to_y(rot[1]);
        self.rotate_to_z(rot[2]);
    }

    fn set_moved_flags(&mut self) {
        let mut cam = self.cam.borrow_mut();
        cam.was_moved = true;
        cam.mtrx_made = false;
    }
    pub fn position_x(&mut self, pos: f32) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.eye[[0]] = pos;
            cam.t2[[3, 0]] = -cam.eye[[0]];
        }
        self.set_moved_flags();
    }
    pub fn position_y(&mut self, pos: f32) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.eye[[1]] = pos;
            cam.t2[[3, 1]] = -cam.eye[[1]];
        }
        self.set_moved_flags();
    }
    pub fn position_z(&mut self, pos: f32) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.eye[[2]] = pos;
            cam.t2[[3, 2]] = -cam.eye[[2]];
        }
        self.set_moved_flags();
    }
    pub fn position(&mut self, pos: &[f32; 3]) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.eye = nd::arr1(pos);
            cam.t2.slice_mut(s![3, ..3]).assign(&(nd::arr1(pos) * -1.0));
        }
        self.set_moved_flags();
    }
    pub fn offset(&mut self, offs: &[f32; 3]) {
        {
            let mut cam = self.cam.borrow_mut();
            cam.t1
                .slice_mut(s![3, ..3])
                .assign(&(nd::arr1(offs) * -1.0));
        }
        self.set_moved_flags();
    }

    pub fn get_matrix(&self) -> nd::Array2<f32> {
        let cam = self.cam.borrow();
        cam.r_mtrx.clone()
    }

    pub fn reference(&self) -> Rc<RefCell<CameraInternals>> {
        self.cam.clone()
    }
}

pub fn create(display: &display::Display) -> Camera {
    let eye: nd::Array1<f32> = nd::arr1(&[0.0, 0.0, -0.1]);
    let reset_eye = eye.clone();
    let at: nd::Array1<f32> = nd::arr1(&[0.0, 0.0, 0.0]);
    let lens: nd::Array1<f32> = nd::arr1(&[0.0, 0.0, 0.0, 0.0]);
    let mut cam_internals = CameraInternals {
        eye,
        reset_eye,
        at,
        lens,
        scale: 1.0,
        width: display.width as f32,
        height: display.height as f32,
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
    cam_internals.set_lens_from_display(display);
    Camera {
        cam: Rc::new(RefCell::new(cam_internals)),
    }
}

fn look_at_matrix(
    at: &nd::Array1<f32>,
    eye: &nd::Array1<f32>,
    up: &nd::Array1<f32>,
) -> nd::Array2<f32> {
    let mut matrix: nd::Array2<f32> = nd::Array::eye(4);
    let zaxis = vec3::norm(&vec3::sub(at, eye)); // unit vec direction cam pointing
    let xaxis = vec3::norm(&vec3::cross(up, &zaxis)); // local horizontal vec
    let yaxis = vec3::cross(&zaxis, &xaxis); // local vert vec
    for i in 0..3 {
        matrix[[i, 0]] = xaxis[i];
        matrix[[i, 1]] = yaxis[i];
        matrix[[i, 2]] = zaxis[i];
    }
    matrix[[3, 0]] = -vec3::dot(&xaxis, eye); // translations
    matrix[[3, 1]] = -vec3::dot(&yaxis, eye);
    matrix[[3, 2]] = -vec3::dot(&zaxis, eye);

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
