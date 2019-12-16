extern crate ndarray;

use std::rc::Rc;
use std::cell::RefCell;
use ndarray as nd;

pub fn create(cam: Rc<RefCell<::camera::CameraInternals>>,
              w: f32, h: f32) -> ::shape::Shape {
    let wh = w * 0.5; let hh = h * 0.5;
    //TODO sort out reason for extra vertex (uv point)
    let verts: nd::Array2<f32> = nd::arr2(
        &[[-wh, hh, 0.0], [wh, hh, 0.0], [wh, -hh, 0.0], [-wh, -hh, 0.0],
          [-wh, hh, 0.0], [wh, hh, 0.0], [wh, -hh, 0.0], [-wh, -hh, 0.0], [0.0, 0.0, 0.0]
        ]);
    let norms: nd::Array2<f32> = nd::arr2(
        &[[-wh, hh, 0.0], [wh, hh, 0.0], [wh, -hh, 0.0], [-wh, -hh, 0.0],
          [-wh, hh, 0.0], [wh, hh, 0.0], [wh, -hh, 0.0], [-wh, -hh, 0.0], [0.0, 0.0, 0.0]
        ]);
    let texcoords: nd::Array2<f32> = nd::arr2(
        &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
          [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]
        ]);
    let faces: nd::Array2<u16> = nd::arr2(
        &[[3, 0, 1], [1, 2, 3], [7, 6, 5], [5, 4, 7]]);

    let new_buffer = ::buffer::create(&::shader::Program::new(), verts, norms, texcoords, faces, true);
    ::shape::create(vec![new_buffer], cam)
}
