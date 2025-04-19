use ndarray as nd;
use std::cell::RefCell;
use std::f32::consts;
use std::rc::Rc;
use crate::{camera, shape, buffer, shader};

pub fn create(
    cam: Rc<RefCell<camera::CameraInternals>>,
    path: Vec<[f32; 2]>,
    sides: usize,
    rise: f32,
    loops: f32,
) -> shape::Shape {
    let s = path.len(); // iterations along path
    let rl = sides * loops as usize; // iterations around axis

    let mut pn: u16 = 0; // keep track of vert index for faces
    let mut pp: u16 = 0; // ditto
    let tcx = 1.0 / sides as f32; // UV step horizontally
    let pr = (consts::PI / sides as f32) * 2.0; // angle rotated per vertex
    let rdiv = rise / rl as f32; // increment axially each step

    // Find length of the path
    let mut path_len = 0.0;
    for p in 1..s {
        path_len += ((path[p][0] - path[p - 1][0]).powf(2.0)
            + (path[p][1] - path[p - 1][1]).powf(2.0))
        .powf(0.5);
    }

    let mut verts = Vec::<f32>::new();
    let mut norms = Vec::<f32>::new();
    let mut faces = Vec::<u16>::new();
    let mut tex_coords = Vec::<f32>::new();

    let mut opx = path[0][0];
    let mut opy = path[0][1];

    let mut tcy = 0.0; // UV val vertically
    for p in 0..s {
        let px = path[p][0]; // for brevity
        let mut py = path[p][1]; // and clarity (and need to increment!)
        let step_len = ((px - opx).powf(2.0) + (py - opy).powf(2.0)).powf(0.5);
        if p > 0 {
            tcy += step_len / path_len;
        }
        let dx = (px - opx) / step_len;
        let dy = (py - opy) / step_len;

        for r in 0..(rl + 1) {
            let sinr = (pr * r as f32).sin();
            let cosr = (pr * r as f32).cos();
            verts.extend_from_slice(&[px * sinr, py, px * cosr]);
            norms.extend_from_slice(&[-sinr * dy, dx, -cosr * dy]);
            tex_coords.extend_from_slice(&[1.0 - tcx * r as f32, tcy]);
            py += rdiv;
        }
        if p < (s - 1) {
            pn += rl as u16 + 1;
            for r in 0..rl {
                faces.extend_from_slice(&[pp + r as u16 + 1, pp + r as u16, pn + r as u16]);
                faces.extend_from_slice(&[pn + r as u16, pn + r as u16 + 1, pp + r as u16 + 1]);
            }
            pp += rl as u16 + 1;
        }
        opx = px;
        opy = py;
    }
    let nverts = verts.len() / 3;
    let nfaces = faces.len() / 3;
    let new_buffer = buffer::create(
        &shader::Program::new(),
        nd::Array::from_shape_vec((nverts, 3usize), verts).unwrap(), //TODO make functions return Result and feedback errors
        nd::Array::from_shape_vec((nverts, 3usize), norms).unwrap(),
        nd::Array::from_shape_vec((nverts, 2usize), tex_coords).unwrap(),
        nd::Array::from_shape_vec((nfaces, 3usize), faces).unwrap(),
        false,
    );
    shape::create(vec![new_buffer], cam)
}
