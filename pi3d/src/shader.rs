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
    id: GLuint,
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
            id: 0, // glCreateProgram returns non-zero number so this identifies uninitiated buffer
            attribute_names: vec![],
            attribute_values: vec![],
            uniform_names: vec![],
            uniform_values: vec![],
        }
    }

    pub fn from_res(display: &::display::Display, name: &str) -> Result<Program, Error> {
        let shaders = [".vs", ".fs"].iter()
            .map(|extn| {
                  Shader::from_res(&display.res, &format!("{}{}", name, extn))
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
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        let p_attrib_names: Vec<String> = ["vertex\0", "normal\0", "texcoord\0"]
            .iter().map(|&s| {s.to_string()}).collect();
        let p_unif_names: Vec<String> = ["modelviewmatrix\0", "unib\0", "unif\0",
              "tex0\0", "tex1\0", "tex2\0", "tex3\0", "tex4\0", "tex5\0", "tex6\0", "tex7\0"]
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

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn attribute_names(&self) -> Vec<String> {
        self.attribute_names.clone()
    }

    pub fn attribute_values(&self) -> Vec<GLint> {
        self.attribute_values.clone()
    }

    pub fn uniform_names(&self) -> Vec<String> {
        self.uniform_names.clone()
    }

    pub fn uniform_values(&self) -> Vec<GLint> {
        self.uniform_values.clone()
    }


    pub fn set_used(&self) {
        unsafe {
            //println!("-->>{:?}", self.id);
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            //println!("NOT deleting program {:?}", self.id);
            //gl::DeleteProgram(self.id); //TODO use lifetimes to do this properly!
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
        let mut source = res.load_string(name)
            .map_err(|e| Error::ResourceLoad { name: name.into(), inner: e })?;
        if res.gl_id == "GLES20" {
            source = source.replace("version 120", "version 100")
                           .replace("//precision", "precision");
        }
        if res.gl_id == "GLES30" {
            source = source.replace("version 120", "version 300 es")
                           .replace("//precision", "precision")
                           .replace("attribute", "in")
                           .replace("Texture2D", "Texture")
                           .replace("//fragcolor", "out vec4 fragColor;")
                           .replace("gl_FragColor", "fragColor");
            if shader_kind == gl::VERTEX_SHADER {
                source = source.replace("varying", "out");
            }
            else {
                source = source.replace("varying", "in");
            }
        } // the default settings are for GL21
        let c_source = CString::new(source).unwrap();
        Shader::from_source(&c_source, shader_kind)
            .map_err(|message| Error::CompileError { name: name.into(), message })
    }

    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    /*pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }*/

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            //println!("NOT deleting shader {:?}", self.id);
            //gl::DeleteShader(self.id); //TODO use lifetimes to do this properly!
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
