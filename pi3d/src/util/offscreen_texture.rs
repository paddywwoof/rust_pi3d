extern crate gl;
extern crate ndarray;
extern crate image;

use gl::types::*;
use ndarray as nd;

pub struct OffscreenTexture {
    pub color_tex_id: GLuint,
    pub depth_tex_id: GLuint,
    pub width: usize,
    pub height: usize,
    pub framebuffer: GLuint,
    pub depthbuffer: GLuint,
}

impl OffscreenTexture {
    ///
    pub fn start(&mut self, clear: bool) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.depthbuffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT16,
                self.width as GLsizei, self.height as GLsizei);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT,
                gl::RENDERBUFFER, self.depthbuffer);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D, self.color_tex_id, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0); // this seems to be needed here
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D, self.depth_tex_id, 0);
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
            gl::BindTexture(gl::TEXTURE, 0);
            gl::DeleteTextures(1, &self.color_tex_id);
            gl::DeleteTextures(1, &self.depth_tex_id);
        }
    }
}
///
impl Drop for OffscreenTexture {
    fn drop(&mut self) {
        print!("-ost{:?}.{:?} ", self.color_tex_id, self.depth_tex_id);
        self.delete_buffers();
    }
}

///
pub fn create(display: &::display::Display) -> OffscreenTexture {
    let height = display.height as usize;
    let width = display.width as usize;
    let mut color_tex_id: GLuint = 0;
    let mut depth_tex_id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut color_tex_id);
        gl::BindTexture(gl::TEXTURE_2D, color_tex_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, width as GLint,
                        height as GLint, 0, gl::RGBA, gl::UNSIGNED_BYTE,
                        std::ptr::null() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::GenTextures(1, &mut depth_tex_id);
        gl::BindTexture(gl::TEXTURE_2D, depth_tex_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT16 as GLint, width as GLint,
                        height as GLint, 0, gl::DEPTH_COMPONENT, gl::UNSIGNED_SHORT,
                        std::ptr::null() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl::Enable(gl::TEXTURE_2D);
    }
    let mut framebuffer: GLuint = 0;
    let mut depthbuffer: GLuint = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::GenRenderbuffers(1, &mut depthbuffer);
    }
    OffscreenTexture {
        color_tex_id,
        depth_tex_id,
        width,
        height,
        framebuffer,
        depthbuffer,
    }
}