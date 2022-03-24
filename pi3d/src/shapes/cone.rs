use std::cell::RefCell;
use std::rc::Rc;

pub fn create(
    cam: Rc<RefCell<::camera::CameraInternals>>,
    radius: f32,
    height: f32,
    sides: usize,
) -> ::shape::Shape {
    let path: Vec<[f32; 2]> = vec![
        [0.0, height * 0.5],
        [radius * 0.999, -height * 0.499],
        [radius, -height * 0.5],
        [radius * 0.999, -height * 0.5],
        [0.0, -height * 0.5],
    ];

    ::shapes::lathe::create(cam, path, sides, 0.0, 1.0)
}
