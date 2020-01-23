extern crate gl;
extern crate ndarray;
extern crate image;

use gl::types::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct PostProcess {
    pub offscreen_texture: ::util::offscreen_texture::OffscreenTexture,
    pub sprite: ::shape::Shape,
    pub scale: f32,
}

impl PostProcess {
    ///
    pub fn start_capture(&mut self, clear: bool) {
        self.offscreen_texture.start(clear);
        let width = self.offscreen_texture.width as f32;
        let height = self.offscreen_texture.height as f32;
        //if self.scale != 1.0 {
            let xx = (width / 2.0 * (1.0 - self.scale)) as GLint;
            let yy = (height / 2.0 * (1.0 - self.scale)) as GLint;
            let ww = (width * self.scale) as GLint;
            let hh = (height * self.scale) as GLint;
            unsafe {
                gl::Enable(gl::SCISSOR_TEST);
                gl::Scissor(xx, yy, ww, hh);
            }
        //}
    }
    ///
    pub fn end_capture(&mut self) {
        self.offscreen_texture.end();
        //if self.scale != 1.0 {
            unsafe {
                gl::Disable(gl::SCISSOR_TEST);
            }
        //}
    }
    ///
    pub fn draw(&mut self, unif_vals: Vec<(usize, usize, f32)>) {
        for (i, j, val) in unif_vals {
            self.sprite.unif[[i, j]] = val;
        }
        self.sprite.draw();
    }
}

pub fn create(
        cam: Rc<RefCell<::camera::CameraInternals>>,
        display: &::display::Display,
        shader: &::shader::Program,
        add_tex: &Vec<GLuint>,
        scale: f32) -> PostProcess {
    let offscreen_texture = ::util::offscreen_texture::create(display);
    let mut sprite = ::shapes::plane::create(cam, display.width, display.height);
    sprite.buf[0].unib[[2, 0]] = scale;
    sprite.buf[0].unib[[2, 1]] = scale;
    sprite.buf[0].unib[[3, 0]] = (1.0 - scale) * 0.5;
    sprite.buf[0].unib[[3, 1]] = (1.0 - scale) * 0.5;
    sprite.buf[0].textures = vec![offscreen_texture.color_tex_id, offscreen_texture.depth_tex_id];
    sprite.buf[0].textures.extend(add_tex.clone());
    sprite.buf[0].set_shader(&shader);
    sprite.position_z(20.0);
    PostProcess {
        offscreen_texture,
        sprite,
        scale,
    }
}