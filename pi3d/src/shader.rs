use gl;
use gl::types::*;
use std;
use std::ffi::{CString, CStr};
use ::util::resources::{self, Resources};

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name: String, inner: resources::Error },
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

pub struct Program {
    id: GLuint, // start value -1
    attribute_names: Vec<String>,
    attribute_values: Vec<GLint>,
    uniform_names: Vec<String>,
    uniform_values: Vec<GLint>,
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {
            id: self.id,
            attribute_names: self.attribute_names.iter().map(|s| {s.to_string()}).collect(),
            attribute_values: self.attribute_values.clone(),
            uniform_names: self.uniform_names.iter().map(|s| {s.to_string()}).collect(),
            uniform_values: self.uniform_values.clone(),
        }
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            id: 0,
            attribute_names: vec![],
            attribute_values: vec![],
            uniform_names: vec![],
            uniform_values: vec![],
        }
    }

    pub fn from_res(display: &::display::Display, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vs",
            ".fs",
        ];

        let resource_names = POSSIBLE_EXT.iter()
            .map(|file_extension| format!("{}{}", name, file_extension))
            .collect::<Vec<String>>();

        let shaders = resource_names.iter()
            .map(|resource_name| {
                Shader::from_res(&display.res, resource_name)
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..])
            .map_err(|message| Error::LinkError { name: name.into(), message })
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        println!("success={:?}", success);
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        let p_attrib_names: Vec<String> = ["vertex", "normal", "texcoord"]
            .iter().map(|&s| {s.to_string()}).collect();
        let p_unif_names: Vec<String> = ["modelviewmatrix", "unib", "unif",
              "tex0", "tex1", "tex2", "tex3", "tex4", "tex5", "tex6", "tex7"]
            .iter().map(|&s| {s.to_string()}).collect();
        let mut p_attrib_vals: Vec<GLint> = vec![-1; 3];
        let mut p_unif_vals: Vec<GLint> = vec![-1; 11];
        unsafe {
            for i in 0..p_attrib_names.len() {
                p_attrib_vals[i] = gl::GetAttribLocation(program_id,
                p_attrib_names[i].as_bytes().as_ptr() as *const GLchar);
            }
            for i in 0..p_unif_names.len() {
                p_unif_vals[i] = gl::GetUniformLocation(program_id,
                p_unif_names[i].as_bytes().as_ptr() as *const GLchar);
            }
        }

        Ok(Program { id: program_id,
                     attribute_names: p_attrib_names,
                     attribute_values: p_attrib_vals,
                     uniform_names: p_unif_names,
                     uniform_values: p_unif_vals })
    }

    pub fn id(&self) -> GLuint { //TODO allow -1 to be used for empty shaders
        self.id
    }

    pub fn get_attribute_location(&self, attrib_name: &str) -> GLuint {
        for i in 0..self.attribute_names.len() {
            if self.attribute_names[i] == attrib_name {
                return self.attribute_values[i] as GLuint; // TODO both these fn return -1 and buffer.draw converts attribute_location
            }
        }
        0
    }
  
    pub fn get_uniform_location(&self, unif_name: &str) -> GLint { // this needs to be int but attribs need uint!!
        for i in 0..self.uniform_names.len() {
            if self.uniform_names[i] == unif_name {
                return self.uniform_values[i];
            }
        }
        -1
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id); // TODO change to GLuint here if >= 0
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, GLenum); 2] = [
            (".vs", gl::VERTEX_SHADER),
            (".fs", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;

        let source = res.load_cstring(name)
            .map_err(|e| Error::ResourceLoad { name: name.into(), inner: e })?;

        Shader::from_source(&source, shader_kind)
            .map_err(|message| Error::CompileError { name: name.into(), message })
    }

    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
