extern crate pyo3;
extern crate pi3d;
extern crate gl;

use pyo3::prelude::*;
use pyo3::{PyObject, PyRawObject};
use pyo3::exceptions;
use pyo3::types::{PyTuple, PyList, PyDict, PyAny};
//use gl::types::GLuint;

#[pyclass(module="rpi3d")]
struct Display {
    r_display: pi3d::display::Display,
}

#[pymethods]
impl Display {
    #[new]
    fn new(obj: &PyRawObject, name: &str, w: f32, h: f32, profile: &str, major: u8, minor: u8) {
        obj.init({
            Display {
                r_display: pi3d::display::create(name, w, h, profile, major, minor).unwrap(),
            }
        });
    }

    #[staticmethod]
    fn create(name: &str, w: f32, h: f32, profile: &str, major: u8, minor: u8) -> PyResult<Py<Display>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut r_display = pi3d::display::create(name, w, h, profile, major, minor).unwrap();
        r_display.set_target_fps(1000.0); //TODO set to 60; testing run as fast as poss 
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

trait GetShape {
    fn get_shape(&mut self) -> pi3d::shape::Shape;
}

/// Shape stuff
///
macro_rules! generate_shape {
    ($supper:ident, $slower:ident, $r_slower:ident) => {
        #[pyclass(module="rpi3d")]
        struct $supper {
            $r_slower: pi3d::shape::Shape,
        }

        impl GetShape for $supper {
            fn get_shape(&mut self) -> pi3d::shape::Shape {
                self.$r_slower.clone()
            }
        }

        #[pymethods]
        impl $supper {
            fn set_draw_details(&mut self, shader: &Shader, textures: Vec<&Texture>,
                        ntiles: f32, shiny: f32, umult: f32,
                        vmult:f32, bump_factor: f32) -> PyResult<()>{
                let texlist = textures.iter().map(|t| t.r_texture.id).collect();
                self.$r_slower.set_draw_details(&shader.r_shader, &texlist, ntiles, shiny,
                                    umult, vmult, bump_factor);
                Ok(())
            }
            fn draw(&mut self) {
                self.$r_slower.draw();
            }
            fn set_shader(&mut self, shader: &Shader) {
                self.$r_slower.set_shader(&shader.r_shader);
            }
            fn set_textures(&mut self, textures: Vec<&Texture>) {
                let texlist = textures.iter().map(|t| t.r_texture.id).collect();
                self.$r_slower.set_textures(&texlist);
            }
            fn set_material(&mut self, material: Vec<f32>) {
                self.$r_slower.set_material(&material);
            }
            fn set_normal_shine(&mut self, textures: Vec<&Texture>, ntiles: f32,
                    shiny: f32, umult: f32, vmult:f32, bump_factor: f32, is_uv: bool) {
                let texlist = textures.iter().map(|t| t.r_texture.id).collect();
                self.$r_slower.set_normal_shine(&texlist, ntiles, shiny,
                                    umult, vmult, bump_factor, is_uv);
            }
            fn set_specular(&mut self, specular: Vec<f32>) {
                self.$r_slower.set_specular(&specular);
            }
            fn rotate_inc_x(&mut self, da: f32) {
                self.$r_slower.rotate_inc_x(da);
            }
            fn rotate_inc_y(&mut self, da: f32) {
                self.$r_slower.rotate_inc_y(da);
            }
            fn rotate_inc_z(&mut self, da: f32) {
                self.$r_slower.rotate_inc_z(da);
            }
            fn rotate_to_x(&mut self, a: f32) {
                self.$r_slower.rotate_to_x(a);
            }
            fn rotate_to_y(&mut self, a: f32) {
                self.$r_slower.rotate_to_y(a);
            }
            fn rotate_to_z(&mut self, a: f32) {
                self.$r_slower.rotate_to_z(a);
            }
            fn position_x(&mut self, pos: f32) {
                self.$r_slower.position_x(pos);
            }
            fn position_y(&mut self, pos: f32) {
                self.$r_slower.position_y(pos);
            }
            fn position_z(&mut self, pos: f32) {
                self.$r_slower.position_z(pos);
            }
            fn position(&mut self, pos: Vec<f32>) {
                self.$r_slower.position(&pos);
            }
            fn offset(&mut self, offs: Vec<f32>) {
                self.$r_slower.offset(&offs);
            }
            fn scale(&mut self, scale: Vec<f32>) {
                self.$r_slower.scale(&scale);
            }
            fn set_light(&mut self, num: usize, posn: Vec<f32>,
                            rgb: Vec<f32>, amb: Vec<f32>, point: bool) {
                self.$r_slower.set_light(num, &posn, &rgb, &amb, point);
            }
            fn set_fog(&mut self, shade: Vec<f32>, dist: f32, alpha: f32) {
                self.$r_slower.set_fog(&shade, dist, alpha);
            }
            fn set_blend(&mut self, blend: bool) {
                self.$r_slower.set_blend(blend);
            }
            #[args(child="*")]
            fn add_child(&mut self, child: &PyTuple) {// testing with &PyTuple - this is still unsatisfactory
              if child.len() == 1 {
                let r_ln: Result<&mut Lines, _> = child.get_item(0).try_into_mut();
                match r_ln {
                  Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                  _ => {let r_sp: Result<&mut Sphere, _> = child.get_item(0).try_into_mut();
                    match r_sp {
                      Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                      _ => {let r_cu: Result<&mut Cuboid, _> = child.get_item(0).try_into_mut();
                        match r_cu {
                          Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                          _ => {let r_po: Result<&mut Points, _> = child.get_item(0).try_into_mut();
                            match r_po{
                              Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                              _ => {let r_pl: Result<&mut Plane, _> = child.get_item(0).try_into_mut();
                                match r_pl {
                                  Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                                  _ => {let r_la: Result<&mut Lathe, _> = child.get_item(0).try_into_mut();
                                    match r_la {
                                      Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                                      _ => {let r_ps: Result<&mut PyString, _> = child.get_item(0).try_into_mut();
                                        match r_ps {
                                          Ok(s) => {self.$r_slower.add_child(s.get_shape()); },
                                          _ => {println!("not lines, sphere, cuboid, points, plane");},
                                        }
                                      },
                                    }
                                  },
                                }
                              },
                            }
                          },
                        }
                      },
                    }
                  },
                }
              }
            }
            fn rotate_child_x(&mut self, child_index: usize, da: f32)  -> PyResult<()>{
                if child_index >= self.$r_slower.children.len() {
                    return Err(PyErr::new::<exceptions::IndexError, _>("There isn't a child at that ix"));
                }
                self.$r_slower.children[child_index].rotate_inc_x(da);
                Ok(())
            }
            fn rotate_child_y(&mut self, child_index: usize, da: f32)  -> PyResult<()>{
                if child_index >= self.$r_slower.children.len() {
                    return Err(PyErr::new::<exceptions::IndexError, _>("There isn't a child at that ix"));
                }
                self.$r_slower.children[child_index].rotate_inc_y(da);
                Ok(())
            }
            fn rotate_child_z(&mut self, child_index: usize, da: f32)  -> PyResult<()>{
                if child_index >= self.$r_slower.children.len() {
                    return Err(PyErr::new::<exceptions::IndexError, _>("There isn't a child at that ix"));
                }
                self.$r_slower.children[child_index].rotate_inc_z(da);
                Ok(())
            }
        }
    };
}

generate_shape!(Plane, plane, r_plane);
#[pymethods]
impl Plane {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, w:f32, h:f32) {
        obj.init({
            Plane {
                r_plane: pi3d::shapes::plane::create(camera.r_camera.reference(), w, h),
            }
        });
    }
}
generate_shape!(Cuboid, cuboid, r_cuboid);
#[pymethods]
impl Cuboid {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, w: f32, h: f32, d: f32, tw: f32, th: f32, td: f32) {
        obj.init({
            Cuboid {
                r_cuboid: pi3d::shapes::cuboid::create(camera.r_camera.reference(), w, h, d, tw, th, td),
            }
        });
    }
}
generate_shape!(Lathe, lathe, r_lathe);
#[pymethods]
impl Lathe {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, path: Vec<Vec<f32>>, sides: usize, rise: f32, loops: f32) {
        let vec_arr: Vec<[f32; 2]> = path.iter().map(|v| [v[0], v[1]]).collect(); //TODO error if wrong dim 
        obj.init({
            Lathe {
                r_lathe: pi3d::shapes::lathe::create(camera.r_camera.reference(), vec_arr, sides, rise, loops),
            }
        });
    }
}
generate_shape!(Lines, lines, r_lines);
#[pymethods]
impl Lines {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, verts: Vec<f32>, line_width: f32, closed: bool) {
        obj.init({
            Lines {
                r_lines: pi3d::shapes::lines::create(camera.r_camera.reference(), &verts, line_width, closed),
            }
        });
    }
}
generate_shape!(Points, points, r_points);
#[pymethods]
impl Points {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, verts: Vec<f32>, point_size: f32) {
        obj.init({
            Points {
                r_points: pi3d::shapes::points::create(camera.r_camera.reference(), &verts, point_size),
            }
        });
    }
}
generate_shape!(Sphere, sphere, r_sphere);
#[pymethods]
impl Sphere {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, radius: f32, slices: usize, sides: usize, hemi: f32, invert: bool) {
        obj.init({
            Sphere {
                r_sphere: pi3d::shapes::sphere::create(camera.r_camera.reference(), radius, slices, sides, hemi, invert),
            }
        });
    }
}

/// Font stuff
///
#[pyclass(module="rpi3d")]
struct Font {
    r_font: pi3d::util::font::TextureFont,
}

#[pymethods]
impl Font {
    #[new]
    fn new(obj: &PyRawObject, file_name: &str, glyphs: &str,
              add_glyphs: &str, size: f32) {
        obj.init({
            Font {
                r_font: pi3d::util::font::create(file_name, glyphs, add_glyphs, size),
            }
        });
    }
}
generate_shape!(PyString, string, r_pystring);
#[pymethods]
impl PyString {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut Camera, font: &Font, string: &str, justify: f32) {
        obj.init({
            PyString {
                r_pystring: pi3d::shapes::string::create(camera.r_camera.reference(), &font.r_font, string, justify),
            }
        });
    }
}

#[pymodule]
fn rpi3d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Display>()?;
    m.add_class::<Shader>()?;
    m.add_class::<Texture>()?;
    m.add_class::<Camera>()?;
    m.add_class::<Plane>()?;
    m.add_class::<Cuboid>()?;
    m.add_class::<Lathe>()?;
    m.add_class::<Lines>()?;
    m.add_class::<Points>()?;
    m.add_class::<Sphere>()?;
    m.add_class::<Font>()?;
    m.add_class::<PyString>()?;
    Ok(())
}