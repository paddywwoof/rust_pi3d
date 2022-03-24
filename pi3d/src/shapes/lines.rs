extern crate ndarray;

use ndarray as nd;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create(
    cam: Rc<RefCell<::camera::CameraInternals>>,
    verts: &Vec<f32>,
    line_width: f32,
    closed: bool,
) -> ::shape::Shape {
    //TODO sort out reason for extra vertex (uv point)
    let norms = nd::Array2::<f32>::zeros((0, 3));
    let tex_coords = nd::Array2::<f32>::zeros((0, 2));
    let nverts = verts.len() / 3;
    let nfaces = nverts / 3 + 1;
    let mut faces = Vec::<u16>::new();
    for a in 0..nfaces {
        for i in 0..3 {
            let v = a * 3 + i;
            faces.push(if v < nverts { v } else { nverts - 1 } as u16);
        }
    }

    let mut new_buffer = ::buffer::create(
        &::shader::Program::new(),
        nd::Array::from_shape_vec((nverts, 3usize), verts.to_vec()).unwrap(), //TODO make functions return Result and feedback errors
        norms,
        tex_coords,
        nd::Array::from_shape_vec((nfaces, 3usize), faces).unwrap(),
        false,
    );

    new_buffer.set_line_width(line_width, true, closed);

    ::shape::create(vec![new_buffer], cam)
}
