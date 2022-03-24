extern crate ndarray;

use ndarray as nd;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create(
    cam: Rc<RefCell<::camera::CameraInternals>>,
    verts: &Vec<f32>,
    point_size: f32,
) -> ::shape::Shape {
    //TODO sort out reason for extra vertex (uv point)
    let nverts = verts.len() / 3;
    let nfaces = nverts / 3 + 1;
    let mut faces = Vec::<u16>::new();
    for a in 0..nfaces {
        for i in 0..3 {
            let v = a * 3 + i;
            faces.push(if v < nverts { v } else { nverts - 1 } as u16);
        }
    }
    let norms = nd::Array2::<f32>::zeros((nverts, 3));
    let tex_coords = nd::Array2::<f32>::zeros((nverts, 2));

    let mut new_buffer = ::buffer::create(
        &::shader::Program::new(),
        nd::Array::from_shape_vec((nverts, 3usize), verts.to_vec()).unwrap(), //TODO make functions return Result and feedback errors
        norms,
        tex_coords,
        nd::Array::from_shape_vec((nfaces, 3usize), faces).unwrap(),
        false,
    );

    new_buffer.set_point_size(point_size);

    ::shape::create(vec![new_buffer], cam)
}
