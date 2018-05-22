/* utility functions working with 1D vectors of xyz f32 values using ndarray::arr1<f32>
 */
extern crate ndarray;

use ndarray as nd;

pub fn add(a: &nd::Array1<f32>, b: &nd::Array1<f32>) -> nd::Array1<f32> {
    a + b
}

pub fn sub(a: &nd::Array1<f32>, b: &nd::Array1<f32>) -> nd::Array1<f32> {
    a - b
}

pub fn len(a: &nd::Array1<f32>) -> f32 {
    let len: f32 = a.iter().map(|x| x * x).sum();
    len.sqrt()
}

pub fn norm(a: &nd::Array1<f32>) -> nd::Array1<f32> {
    let len = len(a);
    if len == 0.0 {return nd::arr1(&[0.0, 1.0, 0.0]);}
    a / len
}

pub fn dot(a: &nd::Array1<f32>, b: &nd::Array1<f32>) -> f32 {
    (a * b).scalar_sum()
}

pub fn cross(a: &nd::Array1<f32>, b: &nd::Array1<f32>) -> nd::Array1<f32> {
    nd::arr1(&[a[1] * b[2] - a[2] * b[1],
               a[2] * b[0] - a[0] * b[2],
               a[0] * b[1] - a[1] * b[0]])
}

pub fn rotate_vec(a: &[f32; 3], vecs: &nd::Array2<f32>) -> nd::Array2<f32> {
    rotate_vec_slice(a, &vecs.slice(s![..,..]))
}
pub fn rotate_vec_slice(a: &[f32; 3], vecs: &nd::ArrayView2<f32>) -> nd::Array2<f32> {
    let (cx, sx) = (a[0].cos(), a[0].sin());
    let (cy, sy) = (a[1].cos(), a[1].sin());
    let (cz, sz) = (a[2].cos(), a[2].sin());
    let rx = nd::arr2(&[[1.0,0.0,0.0],
                        [0.0,cx,sx],
                        [0.0,-sx,cx]]);
    let ry = nd::arr2(&[[cy,0.0,-sy],
                        [0.0,1.0,0.0],
                        [sy,0.0,cy]]);
    let rz = nd::arr2(&[[cz,sz,0.0],
                        [-sz,cz,0.0],
                        [0.0,0.0,1.0]]);
    rz.dot(&rx.dot(&ry.dot(&vecs.reversed_axes()))).reversed_axes()
}

/// normalize a 3 column wide slice of an array in place.
///
/// NB in order for this to be able to work on a slice of an array it
/// requires the lefthand colum 'from' to be supplied as an argument
pub fn normalize_slice(vecs: &mut nd::Array2<f32>, from: usize) {
    let n = vecs.shape()[0];
    for i in 0..n {
        let len: f32 = vecs.slice(s![i,from..(from + 3)]).iter().map(|x| x * x).sum();
        if len > 0.0 {
            let len_inv = 1.0 / len.sqrt();
            for j in from..(from + 3) {vecs[[i, j]] *= len_inv;}
        }
    }
}
