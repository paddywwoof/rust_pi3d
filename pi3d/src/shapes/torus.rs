use std::f32::consts;
use std::rc::Rc;
use std::cell::RefCell;

pub fn create(cam: Rc<RefCell<::camera::CameraInternals>>,
              radius: f32, thickness: f32, ringrots: usize, sides: usize) -> ::shape::Shape {

    let st = consts::PI * 2.0 / ringrots as f32;
    let path: Vec<[f32; 2]>  = (0..=ringrots)
        .map(|i| {
            let r = st * i as f32;
            [radius - thickness * r.cos(), thickness * r.sin()]
        })
        .collect();

    ::shapes::lathe::create(cam, path, sides, 0.0, 1.0)
}
