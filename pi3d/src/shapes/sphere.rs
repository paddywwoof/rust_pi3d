use std::cell::RefCell;
use std::f32::consts;
use std::rc::Rc;
use crate::{camera, shape, shapes};

pub fn create(
    cam: Rc<RefCell<camera::CameraInternals>>,
    radius: f32,
    slices: usize,
    sides: usize,
    hemi: f32,
    invert: bool,
) -> shape::Shape {
    let mut path = Vec::<[f32; 2]>::new();
    // extra points added at poles to reduce distortion (mainly normals)
    let st = ((consts::PI - 0.002) * (1.0 - hemi)) / slices as f32; // angular step
    path.push([0.0, radius]);
    for r in 0..(slices + 1) {
        path.push([
            radius * (0.001 + st * r as f32).sin(),
            radius * (0.001 + st * r as f32).cos(),
        ]);
    }
    path.push([
        radius * (0.002 + st * slices as f32).sin(),
        radius * (0.002 + st * slices as f32).cos(),
    ]);
    if invert {
        path.reverse();
    }
    shapes::lathe::create(cam, path, sides, 0.0, 1.0)
}
