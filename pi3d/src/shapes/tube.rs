use ndarray as nd;
use std::cell::RefCell;
use std::f32::consts;
use std::rc::Rc;
use crate::{camera, shape, shapes, buffer, shader};

pub fn create(
    cam: Rc<RefCell<camera::CameraInternals>>,
    radius: f32,
    thickness: f32,
    height: f32,
    sides: usize,
    use_lathe: bool,
) -> shape::Shape {
    let t = thickness * 0.5;
    if use_lathe {
        let path: Vec<[f32; 2]> = vec![
            [radius - t * 0.999, height * 0.5],
            [radius + t * 0.999, height * 0.5],
            [radius + t, height * 0.5],
            [radius + t, height * 0.4999],
            [radius + t, -height * 0.4999],
            [radius + t, -height * 0.5],
            [radius + t * 0.999, -height * 0.5],
            [radius - t * 0.999, -height * 0.5],
            [radius - t, -height * 0.5],
            [radius - t, -height * 0.499],
            [radius - t, height * 0.499],
            [radius - t, height * 0.5],
        ];

        shapes::lathe::create(cam, path, sides, 0.0, 1.0)
    } else {
        let step = consts::PI * 2.0 / sides as f32;
        let otr = radius + t;
        let inr = radius - t;
        let ht = height * 0.5;
        let mut verts = Vec::<f32>::new();
        let mut norms = Vec::<f32>::new();
        let mut uvs = Vec::<f32>::new();
        let mut faces = Vec::<u16>::new();

        let normdirs: [[f32; 2]; 4] = [
            [0.0, 1.0],  //up
            [1.0, 0.0],  //out
            [0.0, -1.0], //down
            [-1.0, 0.0],
        ]; //in
        for i in 0..=sides {
            for (j, (xz, y)) in [
                (inr, ht),
                (otr, ht), // up
                (otr, ht),
                (otr, -ht), // out
                (otr, -ht),
                (inr, -ht), // down
                (inr, -ht),
                (inr, ht),
            ]
            .iter()
            .enumerate()
            {
                // in
                let s = (i as f32 * step).sin();
                let c = (i as f32 * step).cos();
                verts.extend_from_slice(&[xz * s, *y, xz * c]);
                let k: usize = j / 2;
                norms.extend_from_slice(&[normdirs[k][0] * s, normdirs[k][1], normdirs[k][0] * c]);
                if k == 0 || k == 2 {
                    // top or bottom
                    uvs.extend_from_slice(&[
                        0.5 * (1.0 + verts[j * 3] / otr),
                        0.5 * (1.0 + verts[j * 3 + 2] / otr),
                    ]);
                } else {
                    uvs.extend_from_slice(&[i as f32 / sides as f32, 0.5 * (1.0 + y / ht)]);
                }
            }
            if i < sides {
                for (a, b, c) in [
                    (0, 1, 8),
                    (1, 9, 8),
                    (2, 3, 10),
                    (3, 11, 10),
                    (4, 5, 12),
                    (12, 5, 13),
                    (6, 7, 14),
                    (7, 15, 14),
                ]
                .iter()
                {
                    let f_off = 8 * i as u16;
                    faces.extend_from_slice(&[a + f_off, b + f_off, c + f_off]);
                }
            }
        }
        let nverts = verts.len() / 3;
        let nfaces = faces.len() / 3;
        let new_buffer = buffer::create(
            &shader::Program::new(),
            nd::Array::from_shape_vec((nverts, 3usize), verts).unwrap(), //TODO make functions return Result and feedback errors
            nd::Array::from_shape_vec((nverts, 3usize), norms).unwrap(),
            nd::Array::from_shape_vec((nverts, 2usize), uvs).unwrap(),
            nd::Array::from_shape_vec((nfaces, 3usize), faces).unwrap(),
            false,
        );
        shape::create(vec![new_buffer], cam)
    }
}
