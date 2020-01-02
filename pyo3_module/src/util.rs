extern crate pyo3;
extern crate pi3d;
//extern crate gl;

use pyo3::prelude::*;
use pyo3::{PyObject, PyRawObject};

/// Font stuff
///
#[pyclass(module="rpi3d")]
pub struct Font {
    pub r_font: pi3d::util::font::TextureFont,
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