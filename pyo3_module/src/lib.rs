extern crate pyo3;
extern crate pi3d;
extern crate gl;

use pyo3::prelude::*;
use pyo3::PyRawObject;

use std::path::PathBuf;

#[pyclass(module="rpi3d")]
struct Display {
    r_display: pi3d::display::Display,
}

#[pymethods]
impl Display {
    #[new]
    fn new(obj: &PyRawObject, name: &str) {
        obj.init({
            Display {
                r_display: pi3d::display::create(name, 500.0, 500.0, "GLES", 3, 0).unwrap(),
            }
        });
    }

    #[staticmethod]
    fn create() -> PyResult<Py<Display>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut r_display = pi3d::display::create("rpi3d window", 500.0, 500.0, "GLES", 3, 0).unwrap();
        r_display.set_target_fps(1000.0);
        Py::new(py, Display { 
                        r_display,
                })
    }

    fn loop_running(&mut self) -> PyResult<bool> {
        Ok(self.r_display.loop_running())
    }
}

/// Camera stuff
///
#[pyclass(module="rpi3d")]
struct Camera {
    r_camera: pi3d::camera::Camera,
}

#[pymethods]
impl Camera {
    #[new]
    fn new(obj: &PyRawObject, display: &Display) {
        obj.init({
            Camera {
                r_camera: pi3d::camera::create(&display.r_display),
            }
        });
    }

    fn set_3d(&mut self, is_3d: bool) {
        self.r_camera.set_3d(is_3d);
    }
}

/// Shader stuff
///
#[pyclass(module="rpi3d")]
struct Shader {
    r_shader: pi3d::shader::Program,
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

/// Texture stuff
///
#[pyclass(module="rpi3d")]
struct Texture {
    r_texture: pi3d::texture::Texture,
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


/// Shape stuff
///
macro_rules! generate_shape {
    ($supper:ident, $slower:ident, $r_slower:ident,
                            $($att:ident : $typ:ty) , *) => {
        #[pyclass(module="rpi3d")]
        struct $supper {
            $r_slower: pi3d::shape::Shape,
        }

        #[pymethods]
        impl $supper {
            #[new]
            fn new(obj: &PyRawObject, camera: &mut Camera $(,$att: $typ)*) {
                obj.init({
                    $supper {
                        $r_slower: pi3d::shapes::$slower::create(camera.r_camera.reference() $(,$att )*),
                    }
                });
            }
            fn set_draw_details(&mut self, shader: &Shader, textures: Vec<&Texture>) -> PyResult<()>{
                let texlist = textures.iter().map(|t| t.r_texture.id).collect();
                self.$r_slower.set_draw_details(&shader.r_shader, &texlist, 1.0, 0.0, 1.0, 1.0, 0.0);
                Ok(())
            }

            fn draw(&mut self) {
                self.$r_slower.draw();
            }
        }
    };
}

generate_shape!(Plane, plane, r_plane, w:f32, h:f32);


#[pymodule]
fn rpi3d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Display>()?;
    m.add_class::<Shader>()?;
    m.add_class::<Texture>()?;
    m.add_class::<Plane>()?;
    m.add_class::<Camera>()?;
    Ok(())
}