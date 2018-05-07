extern crate ndarray;

use ndarray as nda;

pub fn create(w: f32, h: f32, d: f32, tw: f32, th: f32, td: f32) -> ::shape::Shape {
    let wh = w * 0.5; let hh = h * 0.5; let dh = d * 0.5;
    let verts: nda::Array2<f32> = nda::arr2(
      &[[-wh, hh, dh], [wh, hh, dh],   [wh, -hh, dh],  [-wh, -hh, dh],
        [wh, hh, dh],  [wh, hh, -dh],  [wh, -hh, -dh], [wh, -hh, dh],
        [-wh, hh, dh], [-wh, hh, -dh], [wh, hh, -dh],  [wh, hh, dh],
        [wh, -hh, dh], [wh, -hh, -dh], [-wh, -hh, -dh],[-wh, -hh, dh],
        [-wh, -hh, dh],[-wh, -hh, -dh],[-wh, hh, -dh], [-wh, hh, dh],
        [-wh, hh, -dh],[wh, hh, -dh],  [wh, -hh, -dh], [-wh,-hh,-dh]]);
    let norms: nda::Array2<f32> = nda::arr2(
      &[[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],[0.0,- 1.0, 0.0],[0.0, -1.0, 0.0],[0.0, -1.0, 0.0],
        [-1.0, 0.0, 0.0],[-1.0, 0.0, 0.0],[-1.0, 0.0, 0.0],[-1.0, 0.0, 0.0],
        [0.0, 0.0, -1.0],[0.0, 0.0, -1.0],[0.0, 0.0, -1.0],[0.0, 0.0, -1.0]]);
    let tw = tw * 0.5;
    let th = th * 0.5;
    let td = td * 0.5;
    let texcoords: nda::Array2<f32> = nda::arr2(
      &[[0.5 + tw, 0.5 - th], [0.5 - tw, 0.5 - th], [0.5 - tw, 0.5 + th], [0.5 + tw, 0.5 + th],   // tw x th
        [0.5 + td, 0.5 - th], [0.5 - td, 0.5 - th], [0.5 - td, 0.5 + th], [0.5 + td, 0.5 + th],   // td x th
        [0.5 - tw, 0.5 - th], [0.5 + tw, 0.5 - th], [0.5 + tw, 0.5 + th], [0.5 - tw, 0.5 + th],   // tw x th
        [0.5 + tw, 0.5 + td], [0.5 - tw, 0.5 + td], [0.5 - tw, 0.5 - td], [0.5 + tw, 0.5 - td],   // tw x td
        [0.5 - td, 0.5 + th], [0.5 + td, 0.5 + th], [0.5 + td, 0.5 - th], [0.5 - td, 0.5 - th],   // td x th
        [0.5 - tw, 0.5 - th], [0.5 + tw, 0.5 - th], [0.5 + tw, 0.5 + th], [0.5 - tw, 0.5 + th]]); // tw x th
    let faces: nda::Array2<u16> = nda::arr2(
      &[[1, 0, 3],   [3, 2, 1],   [5, 4, 7],    [7, 6, 5],
        [9, 8, 11],  [11, 10, 9], [14, 13, 12], [12, 15, 14],
        [17, 16, 19],[19, 18, 17],[20, 21, 22], [22, 23, 20]]);
    let mut new_buffer = ::buffer::create(::shader::Program::new(), verts, norms, texcoords, faces);
    ::shape::create(vec![new_buffer])
}
