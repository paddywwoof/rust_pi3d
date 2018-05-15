/* utility functions working with 1D vectors of xyz f32 values using ndarray::arr1<f32>
 */
extern crate ndarray;

use ndarray as nda;

pub fn add(a: &nda::Array1<f32>, b: &nda::Array1<f32>) -> nda::Array1<f32> {
    a + b
}

pub fn sub(a: &nda::Array1<f32>, b: &nda::Array1<f32>) -> nda::Array1<f32> {
    a - b
}

pub fn len(a: &nda::Array1<f32>) -> f32 {
    let len: f32 = a.iter().map(|x| x * x).sum();
    len.sqrt()
}

pub fn norm(a: &nda::Array1<f32>) -> nda::Array1<f32> {
    let len = len(a);
    if len == 0.0 {return nda::arr1(&[0.0, 1.0, 0.0]);}
    a / len
}

pub fn dot(a: &nda::Array1<f32>, b: &nda::Array1<f32>) -> f32 {
    (a * b).scalar_sum()
}

pub fn cross(a: &nda::Array1<f32>, b: &nda::Array1<f32>) -> nda::Array1<f32> {
    nda::arr1(&[a[1] * b[2] - a[2] * b[1],
               a[2] * b[0] - a[0] * b[2],
               a[0] * b[1] - a[1] * b[0]])
}
