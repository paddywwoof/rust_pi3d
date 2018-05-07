extern crate gl;
extern crate ndarray;
extern crate image;

use std;
use std::f32;
use gl::types::*;
use ndarray as nda;
use texture::image::GenericImage;

pub struct Texture {
    pub id: GLuint,
    pub image: nda::Array3<u8>,
}

pub fn create_from_array(image: nda::Array3<u8>) -> Texture {
    let (h, w, d) = image.dim();
    let c_type = match d {
        3 => gl::RGB,
        _ => gl::RGBA, //TODO better catching
    };
    let mut new_id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut new_id);
        gl::BindTexture(gl::TEXTURE_2D, new_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexImage2D(gl::TEXTURE_2D, 0, c_type as GLint, w as GLint,
                        h as GLint, 0, c_type, gl::UNSIGNED_BYTE,
                        image.as_ptr() as *const GLvoid);
        gl::Enable(gl::TEXTURE_2D);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Texture {
        id: new_id,
        image: image,
    }
}

pub fn create_from_file(name: &str) -> Texture {
    let im = image::open(name).unwrap(); //TODO use resources to load from path relative to exe file
    let (w, h) = im.dimensions();
    let c_type: usize = match im.color() {
        image::ColorType::RGB(u8) => 3,
        _ => 4, // TODO catch unrecognised types
    };
    let image = nda::Array::from_shape_vec((h as usize, w as usize, c_type), im.raw_pixels()).unwrap();
    create_from_array(image)
}
    

pub fn create() -> Texture {
    let mut image: nda::Array3<u8> = nda::Array::zeros((128, 128, 4));
    image.fill(255);
    image.slice_mut(s![..64, ..64, ..]).assign(&nda::arr1(&[255, 0, 0, 255]));
    image.slice_mut(s![64.., ..64, ..]).assign(&nda::arr1(&[255, 255, 0, 255]));
    image.slice_mut(s![..64, 64.., ..]).assign(&nda::arr1(&[0, 255, 0, 255]));
    image.slice_mut(s![64.., 64.., ..]).assign(&nda::arr1(&[255, 0, 255, 255]));
    create_from_array(image)
}
