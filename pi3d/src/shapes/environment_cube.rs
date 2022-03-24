extern crate image;
extern crate ndarray;

use ndarray as nd;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// generate an enviroment cube which uses six textures
///
/// * `disp` reference to the display object which has file path functionality
/// * `size` cube width
/// * `stem` stem of the name, i.e. before '_back' '_front' etc
/// * `suffix` i.e. jpg, png etc
///
/// NB this function returns a tuple of Shape and Vec of Texture object
/// as they will need to live as long as the enviroment cube is used
///
pub fn create(
    cam: Rc<RefCell<::camera::CameraInternals>>,
    size: f32,
    stem: &str,
    suffix: &str,
) -> (::shape::Shape, HashMap<String, ::texture::Texture>) {
    let parts = vec!["front", "right", "top", "bottom", "left", "back"];
    let mut bufs = Vec::<::buffer::Buffer>::new();
    let ww = size / 2.0;
    let hh = size / 2.0;
    let dd = size / 2.0;
    //TODO why does the uv array need a 'sacrificial' entry on the end?
    bufs.push(::buffer::create(
        &::shader::Program::new(),
        nd::arr2(&[
            [-ww, hh, dd],
            [ww, hh, dd],
            [ww, -hh, dd],
            [-ww, -hh, dd],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.002, 0.002],
            [0.998, 0.002],
            [0.998, 0.998],
            [0.002, 0.998],
            [0.0, 0.0],
        ]),
        nd::arr2(&[[3, 0, 1], [2, 3, 1]]),
        false,
    )); //front

    bufs.push(::buffer::create(
        &::shader::Program::new(),
        nd::arr2(&[
            [ww, hh, dd],
            [ww, hh, -dd],
            [ww, -hh, -dd],
            [ww, -hh, dd],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.002, 0.002],
            [0.998, 0.002],
            [0.998, 0.998],
            [0.002, 0.998],
            [0.0, 0.0],
        ]),
        nd::arr2(&[[3, 0, 1], [2, 3, 1]]),
        false,
    )); //right

    bufs.push(::buffer::create(
        &::shader::Program::new(),
        nd::arr2(&[
            [-ww, hh, dd],
            [-ww, hh, -dd],
            [ww, hh, -dd],
            [ww, hh, dd],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.002, 0.998],
            [0.002, 0.002],
            [0.998, 0.002],
            [0.998, 0.998],
            [0.0, 0.0],
        ]),
        nd::arr2(&[[3, 0, 1], [2, 3, 1]]),
        false,
    )); //top

    bufs.push(::buffer::create(
        &::shader::Program::new(),
        nd::arr2(&[
            [ww, -hh, dd],
            [ww, -hh, -dd],
            [-ww, -hh, -dd],
            [-ww, -hh, dd],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.998, 0.002],
            [0.998, 0.998],
            [0.002, 0.998],
            [0.002, 0.002],
            [0.0, 0.0],
        ]),
        nd::arr2(&[[3, 0, 1], [2, 3, 1]]),
        false,
    )); //bottom

    bufs.push(::buffer::create(
        &::shader::Program::new(),
        nd::arr2(&[
            [-ww, -hh, dd],
            [-ww, -hh, -dd],
            [-ww, hh, -dd],
            [-ww, hh, dd],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.998, 0.998],
            [0.002, 0.998],
            [0.002, 0.002],
            [0.998, 0.002],
            [0.0, 0.0],
        ]),
        nd::arr2(&[[3, 0, 1], [2, 3, 1]]),
        false,
    )); //left

    bufs.push(::buffer::create(
        &::shader::Program::new(),
        nd::arr2(&[
            [-ww, hh, -dd],
            [ww, hh, -dd],
            [ww, -hh, -dd],
            [-ww, -hh, -dd],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, 0.0],
        ]),
        nd::arr2(&[
            [0.998, 0.002],
            [0.002, 0.002],
            [0.002, 0.998],
            [0.998, 0.998],
            [0.0, 0.0],
        ]),
        nd::arr2(&[[3, 1, 0], [2, 1, 3]]),
        false,
    )); //back

    let mut tex_list = HashMap::<String, ::texture::Texture>::new();

    for i in 0..bufs.len() {
        //let path_str = path_buf.to_str().unwrap();
        let fname = format!("{}_{}.{}", &stem, &parts[i], &suffix);
        let tex = ::texture::create_from_file(&fname);
        bufs[i].set_textures(&vec![tex.id]);
        tex_list.insert(parts[i].to_string(), tex);
    }
    let mut new_shape = ::shape::create(bufs, cam);
    new_shape.set_fog(&[0.5, 0.5, 0.5], 5000.0, 1.0);
    (new_shape, tex_list)
}
