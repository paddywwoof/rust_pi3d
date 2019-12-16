extern crate gl;
extern crate ndarray;
extern crate image;

use gl::types::*;
use ndarray as nd;
use texture::image::GenericImage; // confusing name texture

pub struct Texture {
    pub id: GLuint,
    pub image: nd::Array3<u8>,
    pub width: usize,
    pub height: usize,
    repeat: GLint,
}

impl Texture {
    pub fn update_ndarray(&mut self) {
        let (h, w, d) = self.image.dim();
        let c_type = match d {
            3 => gl::RGB,
            _ => gl::RGBA, //TODO better catching, Gray or GrayA also valid
        };
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.repeat);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.repeat);
            gl::TexImage2D(gl::TEXTURE_2D, 0, c_type as GLint, w as GLint,
                            h as GLint, 0, c_type, gl::UNSIGNED_BYTE,
                            self.image.as_ptr() as *const GLvoid);
            gl::Enable(gl::TEXTURE_2D);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn flip_image(&mut self, vert: bool, horizontal: bool) {
        let v = if vert {-1} else {1};
        let h = if horizontal {-1} else {1};
        self.image = self.image.slice(s![..;v, ..;h, ..]).to_owned();
        self.update_ndarray();
    }

    pub fn size(&mut self) -> (usize, usize) {
        let (h, w, _d) = self.image.dim();
        (w, h)
    }

    pub fn set_mirrored_repeat(&mut self, on: bool) {
        self.repeat = if on {gl::MIRRORED_REPEAT} else {gl::REPEAT} as GLint;
        self.update_ndarray();
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        print!("-tex{:?} ", self.id);
        unsafe {
            gl::BindTexture(gl::TEXTURE, 0);
            gl::DeleteTextures(1, &self.id);
        }
    }
}

pub fn create_from_array(image: nd::Array3<u8>) -> Texture {
    let mut new_id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut new_id);
    }
    let (height, width, _d) = image.dim();
    let mut tex = Texture {
        id: new_id,
        image: image,
        width,
        height,
        repeat: gl::REPEAT as GLint,
    };
    tex.update_ndarray();
    tex
}

pub fn create_from_file(name: &str) -> Texture {
    let res = ::util::resources::from_exe_path().unwrap();
    let pb = res.resource_name_to_path(name);
    let im = image::open(pb).unwrap();
    let (w, h) = im.dimensions();
    let c_type: usize = match im.color() {
        image::ColorType::Gray(_u8) => 1,
        image::ColorType::GrayA(_u8) => 2,
        image::ColorType::RGB(_u8) => 3,
        image::ColorType::RGBA(_u8) => 4,
        _ => 4, // TODO catch unrecognised types, need to cope with indexed
    };
    let image = nd::Array::from_shape_vec((h as usize, w as usize, c_type), im.raw_pixels()).unwrap();
    create_from_array(image)
}
