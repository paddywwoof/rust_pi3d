extern crate pyo3;
extern crate pi3d;
extern crate gl;

use pyo3::prelude::*;
use pyo3::{PyObject, PyRawObject};


use std::cell::RefCell;
use std::rc::Rc;


#[pyclass(module="rpi3d")]
pub struct Display {
    r_display: Rc<RefCell<pi3d::display::Display>>,
}

#[pymethods]
impl Display {
    #[new]
    fn new(obj: &PyRawObject, name: &str, w: f32, h: f32, profile: &str, major: u8, minor: u8) {
        obj.init({
            Display {
                r_display: Rc::new(RefCell::new(
                    pi3d::display::create(name, w, h, profile, major, minor).unwrap()
                )),
            }
        });
    }

    #[staticmethod]
    fn create(name: &str, w: f32, h: f32, profile: &str, major: u8, minor: u8) -> PyResult<Py<Display>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let r_display = Rc::new(RefCell::new(
            pi3d::display::create(name, w, h, profile, major, minor).unwrap()
        ));
        r_display.borrow_mut().set_target_fps(1000.0); //TODO set to 60; testing run as fast as poss 
        r_display.borrow_mut().set_mouse_relative(true);
        Py::new(py, Display { 
                        r_display,
                })
    }

    fn loop_running(&mut self) -> PyResult<bool> {
        Ok(self.r_display.borrow_mut().loop_running())
    }
}

/// Camera stuff
///
#[pyclass(module="rpi3d")]
pub struct Camera {
    pub r_camera: pi3d::camera::Camera,
}

#[pymethods]
impl Camera {
    #[new]
    fn new(obj: &PyRawObject, display: &Display) {
        obj.init({
            Camera {
                r_camera: pi3d::camera::create(&display.r_display.borrow()),
            }
        });
    }
    fn reset(&mut self) {
        self.r_camera.reset();
    }
    fn set_3d(&mut self, is_3d: bool) {
        self.r_camera.set_3d(is_3d);
    }
    fn position(&mut self, pos: Vec<f32>) {
        if pos.len() != 3 {return;}
        self.r_camera.position(&[pos[0], pos[1], pos[2]]);
    }
    fn rotate(&mut self, rot: Vec<f32>) {
        if rot.len() != 3 {return;}
        self.r_camera.rotate(&[rot[0], rot[1], rot[2]]);
    }
    fn get_direction(&mut self) -> Vec<f32> {
        self.r_camera.get_direction().to_vec()
    }
}

/// Shader stuff
///
#[pyclass(module="rpi3d")]
pub struct Shader {
    pub r_shader: pi3d::shader::Program,
}

#[pymethods]
impl Shader {
    #[new]
    fn new(obj: &PyRawObject, name: &str) {
        obj.init({
            Shader {
                r_shader: pi3d::shader::Program::from_res(name).unwrap(),
            }
        });
    }
}

/// Keyboard stuff
/// 
#[pyclass(module="rpi3d")]
struct Keyboard {
    r_display: Rc<RefCell<pi3d::display::Display>>,
}

#[pymethods]
impl Keyboard {
    #[new]
    fn new(obj: &PyRawObject, display: &Display) {
        obj.init({
            Keyboard {
                r_display: display.r_display.clone(),
            }
        });
    }
    /// crude char reading as per pi3d
    fn read_code(&self) -> String {
        let disp = self.r_display.borrow();
        if disp.keys_pressed.len() > 0 {
            return disp.keys_pressed.last().unwrap().name();
        }
        "".to_string()
    }
}


/// Mouse stuff
/// 
#[pyclass(module="rpi3d")]
struct Mouse {
    r_display: Rc<RefCell<pi3d::display::Display>>,
}

#[pymethods]
impl Mouse {
    #[new]
    fn new(obj: &PyRawObject, display: &Display) {
        obj.init({
            Mouse {
                r_display: display.r_display.clone(),
            }
        });
    }
    /// also need velocity, values depend on mouse relative (also visibility of cursor)
    fn position(&self) -> (i32, i32) {
        let disp = self.r_display.borrow();
        (disp.mouse_x, disp.mouse_y)
    }
}


/// Texture stuff
///
#[pyclass(module="rpi3d")]
pub struct Texture {
    pub r_texture: pi3d::texture::Texture,
}

#[pymethods]
impl Texture {
    #[new]
    fn new(obj: &PyRawObject, file_name: &str) {
        /*let current_dir: Vec<PathBuf> = std::env::split_paths(&std::env::current_dir()
                                            .unwrap()).collect();
        let file_path = std::env::join*/
        obj.init({
            Texture {
                r_texture: pi3d::texture::create_from_file(file_name),
            }
        });
    }
}

#[pymodule]
fn rpi3d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Display>()?;
    m.add_class::<Camera>()?;
    m.add_class::<Shader>()?;
    m.add_class::<Keyboard>()?;
    m.add_class::<Mouse>()?;
    m.add_class::<Texture>()?;

    m.add_class::<::util::Font>()?;

    m.add_class::<::shapes::Shape>()?;
    m.add_class::<::shapes::Cone>()?;
    m.add_class::<::shapes::Cuboid>()?;
    m.add_class::<::shapes::Cylinder>()?;
    m.add_class::<::shapes::ElevationMap>()?;
    m.add_class::<::shapes::EnvironmentCube>()?;
    m.add_class::<::shapes::Lathe>()?;
    m.add_class::<::shapes::Lines>()?;
    m.add_class::<::shapes::MergeShape>()?;
    m.add_class::<::shapes::Model>()?;
    m.add_class::<::shapes::Plane>()?;
    m.add_class::<::shapes::Points>()?;
    m.add_class::<::shapes::PyString>()?;
    m.add_class::<::shapes::Sphere>()?;
    m.add_class::<::shapes::TCone>()?;
    m.add_class::<::shapes::Torus>()?;
    m.add_class::<::shapes::Tube>()?;
    Ok(())
}