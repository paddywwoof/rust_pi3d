extern crate pi3d;
extern crate pyo3;
//extern crate gl;

use pyo3::prelude::*;
use pyo3::PyObject;

//use numpy::{IntoPyArray, PyArray3};

/// Font stuff
///
#[pyclass]
pub struct Font {
    pub r_font: pi3d::util::font::TextureFont,
}

#[pymethods]
impl Font {
    #[new]
    fn new(file_name: &str, glyphs: &str, add_glyphs: &str, size: f32) -> Self {
        Font {
            r_font: pi3d::util::font::create(file_name, glyphs, add_glyphs, size),
        }
    }
}
/// PostProcess stuff
///
#[pyclass(unsendable)]
pub struct PostProcess {
    pub r_postprocess: pi3d::util::post_process::PostProcess,
}

#[pymethods]
impl PostProcess {
    #[new]
    fn new(
        camera: &mut ::core::Camera,
        display: &::core::Display,
        shader: &::core::Shader,
        add_tex: Vec<PyRef<::core::Texture>>,
        scale: f32,
    ) -> Self {
        let texlist = add_tex.iter().map(|t| t.r_texture.id).collect();
        PostProcess {
            r_postprocess: pi3d::util::post_process::create(
                camera.r_camera.reference(),
                &display.r_display.borrow(),
                &shader.r_shader,
                &texlist,
                scale,
            ),
        }
    }
    pub fn start_capture(&mut self, clear: bool) {
        self.r_postprocess.start_capture(clear);
    }
    pub fn end_capture(&mut self) {
        self.r_postprocess.end_capture();
    }
    pub fn draw(&mut self, unif_vals: Vec<(usize, usize, f32)>) {
        self.r_postprocess.draw(unif_vals);
    }
    /*#[getter]//don't think this will work! Probably need gl::ReadPixels()
    fn get_image(&mut self) -> PyResult<Py<PyArray3<u8>>> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        Ok(self.r_postprocess.offscreen_texture.tex.image
            .clone()
            .into_pyarray(py)
            .to_owned()
        )
    }*/
}
