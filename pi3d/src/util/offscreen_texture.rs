extern crate gl;
extern crate ndarray;
extern crate image;

use gl::types::*;
use ndarray as nd;

pub struct OffscreenTexture {
    pub tex: ::texture::Texture,
    pub framebuffer: GLuint,
    pub depthbuffer: GLuint,
}

impl OffscreenTexture {
    ///
    pub fn start(&mut self, clear: bool) {
        //println!("{:?}", self.tex.id);
        unsafe {
            //gl::BindTexture(gl::TEXTURE_2D, self.tex.id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                                     gl::TEXTURE_2D, self.tex.id, 0);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.depthbuffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT16,
                self.tex.width as GLsizei, self.tex.height as GLsizei);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT,
                gl::RENDERBUFFER, self.depthbuffer);
            if clear { // TODO allow just depth or just color clearing?
                gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
            }
        }
        //TODO global offscreen queue - check why needed.
    }
    ///
    pub fn end(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        //TODO offscreen queue?
    }
    ///
    pub fn delete_buffers(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.framebuffer);
            gl::DeleteRenderbuffers(1, &mut self.depthbuffer);
        }
    }
}

pub fn create(display: &::display::Display) -> OffscreenTexture {
    let height = display.height as usize;
    let width = display.width as usize;
    let image: nd::Array3<u8> = nd::Array3::<u8>::zeros((height, width, 4)); //TODO RGB or RBGA?
    let tex = ::texture::create_from_array(image);
    /*let tex = ::texture::Texture {
        id: 0,
        image,
        width,
        height,
        repeat: gl::REPEAT as GLint,
    };*/
    let mut framebuffer: GLuint = 0;
    let mut depthbuffer: GLuint = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::GenRenderbuffers(1, &mut depthbuffer);
    }
    OffscreenTexture {
        tex,
        framebuffer,
        depthbuffer,
    }
}