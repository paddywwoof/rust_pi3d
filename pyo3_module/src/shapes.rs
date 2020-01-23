extern crate pyo3;
extern crate pi3d;
extern crate gl;
extern crate numpy;

use pyo3::prelude::*;
use pyo3::exceptions;
use numpy::{IntoPyArray, PyArray2};
use gl::types::GLuint;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::clone::Clone;

///
#[pyclass]
pub struct Shape {
    pub r_shape: pi3d::shape::Shape,
    _texlist: HashMap<String, pi3d::texture::Texture>,
}

#[pymethods]
impl Shape {
    #[args(ntiles="1.0", shiny="0.0", umult="1.0", vmult="1.0", bump_factor="1.0")]
    fn set_draw_details(&mut self, shader: &::core::Shader, textures: Vec<&::core::Texture>,
                ntiles: f32, shiny: f32, umult: f32,
                vmult:f32, bump_factor: f32) -> PyResult<()>{
        let texlist = textures.iter().map(|t| t.r_texture.id).collect();
        self.r_shape.set_draw_details(&shader.r_shader, &texlist, ntiles, shiny,
                            umult, vmult, bump_factor);
        Ok(())
    }
    fn draw(&mut self) {
        self.r_shape.draw();
    }
    fn set_shader(&mut self, shader: &::core::Shader) {
        self.r_shape.set_shader(&shader.r_shader);
    }
    fn set_textures(&mut self, textures: Vec<&::core::Texture>) {
        let texlist = textures.iter().map(|t| t.r_texture.id).collect();
        self.r_shape.set_textures(&texlist);
    }
    fn set_material(&mut self, material: Vec<f32>) {
        self.r_shape.set_material(&material);
    }
    #[args(ntiles="1.0", shinetex="None", shiny="0.0", bump_factor="1.0", is_uv="true")]
    fn set_normal_shine(&mut self, normtex: &::core::Texture, ntiles: f32,
            shinetex: Option<&::core::Texture>, shiny: f32, bump_factor: f32, is_uv: bool) {
        let mut texlist: Vec<GLuint> = vec![normtex.r_texture.id];
        match shinetex {
            Some(tex) => {
                texlist.push(tex.r_texture.id);
            },
            None => {},
        };
        self.r_shape.set_normal_shine(&texlist, ntiles, shiny,
                            1.0, 1.0, bump_factor, is_uv);
    }
    fn set_specular(&mut self, specular: Vec<f32>) {
        self.r_shape.set_specular(&specular);
    }
    fn rotate_inc_x(&mut self, da: f32) {
        self.r_shape.rotate_inc_x(da);
    }
    fn rotate_inc_y(&mut self, da: f32) {
        self.r_shape.rotate_inc_y(da);
    }
    fn rotate_inc_z(&mut self, da: f32) {
        self.r_shape.rotate_inc_z(da);
    }
    fn rotate_to_x(&mut self, a: f32) {
        self.r_shape.rotate_to_x(a);
    }
    fn rotate_to_y(&mut self, a: f32) {
        self.r_shape.rotate_to_y(a);
    }
    fn rotate_to_z(&mut self, a: f32) {
        self.r_shape.rotate_to_z(a);
    }
    fn position_x(&mut self, pos: f32) {
        self.r_shape.position_x(pos);
    }
    fn position_y(&mut self, pos: f32) {
        self.r_shape.position_y(pos);
    }
    fn position_z(&mut self, pos: f32) {
        self.r_shape.position_z(pos);
    }
    fn position(&mut self, pos: Vec<f32>) {
        self.r_shape.position(&pos);
    }
    fn offset(&mut self, offs: Vec<f32>) {
        self.r_shape.offset(&offs);
    }
    fn scale(&mut self, scale: Vec<f32>) {
        self.r_shape.scale(&scale);
    }
    #[args(point="false")]
    fn set_light(&mut self, num: usize, posn: Vec<f32>,
                    rgb: Vec<f32>, amb: Vec<f32>, point: bool) {
        self.r_shape.set_light(num, &posn, &rgb, &amb, point);
    }
    fn set_fog(&mut self, shade: Vec<f32>, dist: f32, alpha: f32) {
        self.r_shape.set_fog(&shade, dist, alpha);
    }
    fn set_blend(&mut self, blend: bool) {
        self.r_shape.set_blend(blend);
    }
    fn add_child(&mut self, child: &RefShape) {
        self.r_shape.add_child(child.r_shape_ref.clone());
    }
    /*fn rotate_child_x(&mut self, child_index: usize, da: f32)  -> PyResult<()>{
        if child_index >= self.r_shape.children.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>("There isn't a child at that ix"));
        }
        self.r_shape.children[child_index].rotate_inc_x(da);
        Ok(())
    }
    fn rotate_child_y(&mut self, child_index: usize, da: f32)  -> PyResult<()>{
        if child_index >= self.r_shape.children.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>("There isn't a child at that ix"));
        }
        self.r_shape.children[child_index].rotate_inc_y(da);
        Ok(())
    }
    fn rotate_child_z(&mut self, child_index: usize, da: f32)  -> PyResult<()>{
        if child_index >= self.r_shape.children.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>("There isn't a child at that ix"));
        }
        self.r_shape.children[child_index].rotate_inc_z(da);
        Ok(())
    }*/

    fn add_shapes(&mut self, new_shapes: Vec<&Shape>,
                    loc: Vec<Vec<f32>>, rot: Vec<Vec<f32>>, scl: Vec<Vec<f32>>,
                    num: Vec<usize>) {
        if new_shapes.len() != loc.len() || loc.len() != rot.len() ||
             rot.len() != scl.len() || scl.len() != num.len() {
            return;
        }
        let new_shapes_vec: Vec<&pi3d::shape::Shape> = new_shapes.iter().map(|s| &s.r_shape).collect();
        let new_loc: Vec<[f32; 3]> = loc.iter().map(|v| [v[0], v[1], v[2]]).collect();
        let new_rot: Vec<[f32; 3]> = rot.iter().map(|v| [v[0], v[1], v[2]]).collect();
        let new_scl: Vec<[f32; 3]> = scl.iter().map(|v| [v[0], v[1], v[2]]).collect();
        pi3d::shapes::merge_shape::add_shapes(&mut self.r_shape, new_shapes_vec,
                                              new_loc, new_rot, new_scl, num);
    }
    fn cluster(&mut self, new_shape: &Shape, map: &ElevationMap,
                 xpos: f32, zpos: f32, w: f32, d: f32, minscl: f32, maxscl: f32, count: usize) {
        pi3d::shapes::merge_shape::cluster(&mut self.r_shape, &new_shape.r_shape, &map.r_elevation_map,
                                           xpos, zpos, w, d, minscl, maxscl, count);
    }

    fn get_buffer_num(&mut self, n: usize) -> PyResult<Py<PyArray2<f32>>> {
        if self.r_shape.buf.len() < (n + 1) {
            return Err(PyErr::new::<exceptions::RuntimeError, _>("array index too big"));
        }
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        Ok(self.r_shape.buf[n].array_buffer
            .clone()
            .into_pyarray(py)
            .to_owned()
        )
    }
    #[getter]
    fn get_array_buffer(&mut self) -> PyResult<Py<PyArray2<f32>>> {
        self.get_buffer_num(0)
    }

    fn set_buffer_num(&mut self, n: usize, buff: &PyArray2<f32>) -> PyResult<()> {
        if self.r_shape.buf.len() < (n + 1) {
            return Err(PyErr::new::<exceptions::RuntimeError, _>("there aren't that many buffers"));
        }
        let new_buff = buff.as_array().to_owned();
        let new_shape = new_buff.shape();
        let old_shape = self.r_shape.buf[n].array_buffer.shape();
        if new_shape[0] != old_shape[0] || new_shape[1] != old_shape[1] {
            return Err(PyErr::new::<exceptions::RuntimeError, _>("array wrong shape"));
        }
        self.r_shape.buf[n].array_buffer = new_buff;
        self.r_shape.buf[n].re_init();
        Ok(())
    }
    #[setter]
    fn set_array_buffer(&mut self, buff: &PyArray2<f32>)  -> PyResult<()> {
        self.set_buffer_num(0, buff)
    }
}

macro_rules! shape_from { // NB the : and => below are arbitrary dividers in the $(),* params
    ($sh_cap:ident, $sh_lwr:ident, $($att:ident : $typ:ty => $default:expr) , *) => {
        #[pyclass(extends=Shape)]
        pub struct $sh_cap {}
        #[pymethods]
        impl $sh_cap {
            #[new]
            #[args( $($att=$default,)*)]
            fn new(obj: &PyRawObject, camera: &mut ::core::Camera $(,$att: $typ)*) {
                obj.init({
                    Shape {
                        r_shape: pi3d::shapes::$sh_lwr::create(camera.r_camera.reference() $(,$att)*),
                        _texlist: HashMap::<String, pi3d::texture::Texture>::new(),
                    }
                });
            }
        } 
    };
}

shape_from! (Cone, cone, radius:f32=>"1.0", height:f32=>"2.0", sides:usize=>"12");
shape_from! (Cylinder, cylinder, radius:f32=>"1.0", height:f32=>"2.0", sides:usize=>"12");
shape_from! (Cuboid, cuboid, w:f32=>"1.0", h:f32=>"1.0", d:f32=>"1.0", tw:f32=>"1.0", th:f32=>"1.0", td:f32=>"1.0");
shape_from! (MergeShape, merge_shape,);
shape_from! (Plane, plane, w:f32=>"1.0", h:f32=>"1.0");
shape_from! (Sphere, sphere, radius:f32=>"1.0", slices:usize=>"16", sides:usize=>"12", hemi:f32=>"0.0", invert:bool=>"false");
shape_from! (Torus, torus, radius:f32=>"2.0", thickness:f32=>"0.5", ringrots:usize=>"6", sides:usize=>"12");
shape_from! (Tube, tube, radius:f32=>"1.0", thickness:f32=>"0.2", height:f32=>"2.0", sides:usize=>"12", use_lathe:bool=>"true");
shape_from! (TCone, tcone, radius_bot:f32=>"1.0", radius_top:f32=>"0.5", height:f32=>"2.0", sides:usize=>"12");
/* Canvas, Disk, Extrude, Helix, LodSprite,
MultiSprite, Polygon, Sprite, Tetrahedron, Triangle */

#[pyclass(extends=Shape)]
pub struct Lathe {}
#[pymethods]
impl Lathe {
    #[new]
    #[args(sides="12",rise="0.0", loops="1.0")]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera, path: Vec<Vec<f32>>, sides: usize, rise: f32, loops: f32) {
        let vec_arr: Vec<[f32; 2]> = path.iter().map(|v| [v[0], v[1]]).collect(); //TODO error if wrong dim 
        obj.init({
            Shape {
                r_shape: pi3d::shapes::lathe::create(camera.r_camera.reference(), vec_arr, sides, rise, loops),
                _texlist: HashMap::<String, pi3d::texture::Texture>::new(),
            }
        });
    }
}
#[pyclass(extends=Shape)]
pub struct Lines {}
#[pymethods]
impl Lines {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera, verts: Vec<f32>, line_width: f32, closed: bool) {
        obj.init({
            Shape {
                r_shape: pi3d::shapes::lines::create(camera.r_camera.reference(), &verts, line_width, closed),
                _texlist: HashMap::<String, pi3d::texture::Texture>::new(),
            }
        });
    }
}
#[pyclass(extends=Shape)]
pub struct Points {}
#[pymethods]
impl Points {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera, verts: Vec<f32>, point_size: f32) {
        obj.init({
            Shape {
                r_shape: pi3d::shapes::points::create(camera.r_camera.reference(), &verts, point_size),
                _texlist: HashMap::<String, pi3d::texture::Texture>::new(),
            }
        });
    }
}
#[pyclass(extends=Shape)]
pub struct Model {}
#[pymethods]
impl Model {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera, file_name: &str) {
        let (r_shape, _texlist) = pi3d::shapes::model_obj::create(camera.r_camera.reference(), file_name);
        obj.init({
            Shape {
                r_shape,
                _texlist,
            }
        });
    }
}

#[pyclass(extends=Shape)]
pub struct EnvironmentCube {}
#[pymethods]
impl EnvironmentCube {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera, size: f32, stem: &str, suffix: &str) {
        let (r_shape, _texlist) = pi3d::shapes::environment_cube::create(camera.r_camera.reference(), size, stem, suffix);
        obj.init({
            Shape {
                r_shape,
                _texlist,
            }
        });
    }
}

#[pyclass(extends=Shape)]
pub struct PyString {}
#[pymethods]
impl PyString {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera, font: &::util::Font, string: &str, justify: f32) {
        obj.init({
            Shape {
                r_shape: pi3d::shapes::string::create(camera.r_camera.reference(), &font.r_font, string, justify),
                _texlist: HashMap::<String, pi3d::texture::Texture>::new(),
            }
        });
    }
}

#[pyclass]
pub struct ElevationMap {
    r_elevation_map: pi3d::shapes::elevation_map::ElevationMap,
}
#[pymethods]
impl ElevationMap {
    #[new]
    fn new(obj: &PyRawObject, camera: &mut ::core::Camera,
               mapfile: &str, width: f32, depth: f32, height: f32, ix: usize, iz: usize,
               ntiles: f32, _texmap: &str) {
        obj.init({
            ElevationMap {
                r_elevation_map: pi3d::shapes::elevation_map::new_map(camera.r_camera.reference(),
                                mapfile, width, depth, height, ix, iz, ntiles, _texmap),
            }
        });
    }
    fn calc_height(&self, px: f32, pz: f32) -> (f32, Vec<f32>) {
        pi3d::shapes::elevation_map::calc_height(&self.r_elevation_map, px, pz)
    }
    #[args(ntiles="1.0", shiny="0.0", umult="1.0", vmult="1.0", bump_factor="1.0")]
    fn set_draw_details(&mut self, shader: &::core::Shader, textures: Vec<&::core::Texture>,
                ntiles: f32, shiny: f32, umult: f32,
                vmult:f32, bump_factor: f32) -> PyResult<()>{
        let texlist = textures.iter().map(|t| t.r_texture.id).collect();
        self.r_elevation_map.set_draw_details(&shader.r_shader, &texlist, ntiles, shiny,
                            umult, vmult, bump_factor);
        Ok(())
    }
    fn draw(&mut self) {
        self.r_elevation_map.draw();
    }
    fn set_shader(&mut self, shader: &::core::Shader) {
        self.r_elevation_map.set_shader(&shader.r_shader);
    }
    fn set_textures(&mut self, textures: Vec<&::core::Texture>) {
        let texlist = textures.iter().map(|t| t.r_texture.id).collect();
        self.r_elevation_map.set_textures(&texlist);
    }
    fn set_material(&mut self, material: Vec<f32>) {
        self.r_elevation_map.set_material(&material);
    }
    fn position(&mut self, pos: Vec<f32>) {
        self.r_elevation_map.position(&pos);
    }
}

/// RefShape stuff
#[pyclass]
pub struct RefShape {
    r_shape_ref: Rc<RefCell<pi3d::shape::Shape>>,
}
#[pymethods]
impl RefShape {
    #[new]
    fn new(obj: &PyRawObject, shape: &mut Shape) {
        obj.init({
            RefShape {
                r_shape_ref: shape.r_shape.clone().reference(),
            }
        });
    }

    fn rotate_inc_x(&mut self, da: f32) {
        self.r_shape_ref.borrow_mut().rotate_inc_x(da);
    }
    fn rotate_inc_y(&mut self, da: f32) {
        self.r_shape_ref.borrow_mut().rotate_inc_y(da);
    }
    fn rotate_inc_z(&mut self, da: f32) {
        self.r_shape_ref.borrow_mut().rotate_inc_z(da);
    }
    fn rotate_to_x(&mut self, a: f32) {
        self.r_shape_ref.borrow_mut().rotate_to_x(a);
    }
    fn rotate_to_y(&mut self, a: f32) {
        self.r_shape_ref.borrow_mut().rotate_to_y(a);
    }
    fn rotate_to_z(&mut self, a: f32) {
        self.r_shape_ref.borrow_mut().rotate_to_z(a);
    }
    fn position_x(&mut self, pos: f32) {
        self.r_shape_ref.borrow_mut().position_x(pos);
    }
    fn position_y(&mut self, pos: f32) {
        self.r_shape_ref.borrow_mut().position_y(pos);
    }
    fn position_z(&mut self, pos: f32) {
        self.r_shape_ref.borrow_mut().position_z(pos);
    }
    fn position(&mut self, pos: Vec<f32>) {
        self.r_shape_ref.borrow_mut().position(&pos);
    }
    fn offset(&mut self, offs: Vec<f32>) {
        self.r_shape_ref.borrow_mut().offset(&offs);
    }
    fn scale(&mut self, scale: Vec<f32>) {
        self.r_shape_ref.borrow_mut().scale(&scale);
    }
    fn add_child(&mut self, child: &RefShape) {
        self.r_shape_ref.borrow_mut().add_child(child.r_shape_ref.clone());
    }
}