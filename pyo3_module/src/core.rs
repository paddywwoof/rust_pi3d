extern crate gl;
extern crate pi3d;
extern crate pyo3;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::PyObject; //, PyRawObject};

use numpy::{IntoPyArray, PyArray3};

use std::cell::RefCell;
use std::rc::Rc;

#[pyclass(unsendable)] // think SDL requires this to stay in main thread
pub struct Display {
    pub r_display: Rc<RefCell<pi3d::display::Display>>,
}

#[pymethods]
impl Display {
    #[new]
    #[args(
        name = "\"\"",
        w = "0.0",
        h = "0.0",
        profile = "\"GLES\"",
        major = "2",
        minor = "0"
    )]
    fn new(name: &str, w: f32, h: f32, profile: &str, major: u8, minor: u8) -> Self {
        let (wnew, hnew, fullscreen) = if w <= 0.0 || h <= 0.0 {
            (100.0, 100.0, true)
        } else {
            (w, h, false)
        };
        /*let dispnew = Arc::new(Mutex::new(
            pi3d::display::create(name, wnew, hnew, profile, major, minor).unwrap()
        ));*/
        let dispnew = Rc::new(RefCell::new(
            pi3d::display::create(name, wnew, hnew, profile, major, minor).unwrap(),
        ));
        if fullscreen {
            dispnew.borrow_mut().set_fullscreen(true);
        }
        Display { r_display: dispnew }
    }

    #[staticmethod]
    #[args(
        name = "\"\"",
        w = "0.0",
        h = "0.0",
        profile = "\"GLES\"",
        major = "2",
        minor = "0"
    )]
    fn create(
        name: &str,
        w: f32,
        h: f32,
        profile: &str,
        major: u8,
        minor: u8,
    ) -> PyResult<Py<Display>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let (wnew, hnew, fullscreen) = if w <= 0.0 || h <= 0.0 {
            (100.0, 100.0, true)
        } else {
            (w, h, false)
        };
        let r_display = Rc::new(RefCell::new(
            pi3d::display::create(name, wnew, hnew, profile, major, minor).unwrap(),
        ));
        if fullscreen {
            r_display.borrow_mut().set_fullscreen(true);
        }
        r_display.borrow_mut().set_target_fps(1000.0); //TODO set to 60; testing run as fast as poss
        r_display.borrow_mut().set_mouse_relative(true);
        Py::new(py, Display { r_display })
    }

    fn loop_running(&mut self) -> PyResult<bool> {
        Ok(self.r_display.borrow_mut().loop_running())
    }
}

/// Camera stuff
///
#[pyclass(unsendable)]
pub struct Camera {
    pub r_camera: pi3d::camera::Camera,
}

#[pymethods]
impl Camera {
    #[new]
    fn new(display: &Display) -> Self {
        Camera {
            r_camera: pi3d::camera::create(&display.r_display.borrow()),
        }
    }
    fn reset(&mut self) {
        self.r_camera.reset();
    }
    fn set_3d(&mut self, is_3d: bool) {
        self.r_camera.set_3d(is_3d);
    }
    fn position(&mut self, pos: Vec<f32>) {
        if pos.len() != 3 {
            return;
        }
        self.r_camera.position(&[pos[0], pos[1], pos[2]]);
    }
    fn rotate(&mut self, rot: Vec<f32>) {
        if rot.len() != 3 {
            return;
        }
        self.r_camera.rotate(&[rot[0], rot[1], rot[2]]);
    }
    fn get_direction(&mut self) -> Vec<f32> {
        self.r_camera.get_direction().to_vec()
    }
}

/// Shader stuff
///
#[pyclass]
pub struct Shader {
    pub r_shader: pi3d::shader::Program,
}

#[pymethods]
impl Shader {
    #[new]
    fn new(name: &str) -> Self {
        Shader {
            r_shader: pi3d::shader::Program::from_res(name).unwrap(),
        }
    }
}

/// Keyboard stuff
///
#[pyclass(unsendable)]
struct Keyboard {
    r_display: Rc<RefCell<pi3d::display::Display>>,
}

#[pymethods]
impl Keyboard {
    #[new]
    fn new(display: &Display) -> Self {
        Keyboard {
            r_display: display.r_display.clone(),
        }
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
#[pyclass(unsendable)]
struct Mouse {
    r_display: Rc<RefCell<pi3d::display::Display>>,
}

#[pymethods]
impl Mouse {
    #[new]
    fn new(display: &Display) -> Self {
        Mouse {
            r_display: display.r_display.clone(),
        }
    }
    /// also need velocity, values depend on mouse relative (also visibility of cursor)
    fn position(&self) -> (i32, i32) {
        let disp = self.r_display.borrow();
        (disp.mouse_x, disp.mouse_y)
    }
}

/// Texture stuff
///
#[pyclass]
pub struct Texture {
    pub r_texture: pi3d::texture::Texture,
}

#[pymethods]
impl Texture {
    #[new]
    fn new(file_name: &str) -> Self {
        Texture {
            r_texture: pi3d::texture::create_from_file(file_name),
        }
    }
    fn print_id(&self) {
        println!("texid={}", self.r_texture.id);
    }
    #[getter]
    fn get_image(&mut self) -> PyResult<Py<PyArray3<u8>>> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        Ok(self.r_texture.image.clone().into_pyarray(py).to_owned())
    }
    #[setter]
    fn set_image(&mut self, im_arr: &PyArray3<u8>) -> PyResult<()> {
        unsafe {
            let new_im_arr = im_arr.as_array().to_owned();
            let new_shape = new_im_arr.shape();
            let old_shape = self.r_texture.image.shape();
            if new_shape[0] != old_shape[0] || new_shape[1] != old_shape[1] {
                return Err(PyErr::new::<exceptions::RuntimeError, _>(
                    "array wrong shape",
                ));
            }
            //TODO fix different 3rd dim size (1,3,4)
            self.r_texture.image = new_im_arr;
        }
        self.r_texture.update_ndarray();
        Ok(())
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
    m.add_class::<::util::PostProcess>()?;

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

    m.add_class::<::shapes::RefShape>()?;
    Ok(())
}
