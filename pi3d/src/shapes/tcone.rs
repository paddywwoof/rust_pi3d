use std::cell::RefCell;
use std::rc::Rc;

pub fn create(
    cam: Rc<RefCell<::camera::CameraInternals>>,
    radius_bot: f32,
    radius_top: f32,
    height: f32,
    sides: usize,
) -> ::shape::Shape {
    let path: Vec<[f32; 2]> = vec![
        [0.0, height * 0.5],
        [radius_top * 0.999, height * 0.5],
        [radius_top, height * 0.5],
        [radius_top, height * 0.499],
        [radius_bot, -height * 0.499],
        [radius_bot, -height * 0.5],
        [radius_bot * 0.999, -height * 0.5],
        [0.0, -height * 0.5],
    ];

    ::shapes::lathe::create(cam, path, sides, 0.0, 1.0)
}
