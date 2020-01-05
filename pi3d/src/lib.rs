#[macro_use(s)]
extern crate ndarray;
extern crate gl;
extern crate image;
extern crate rusttype;
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;

lazy_static! {
    static ref EXE_PATH: PathBuf = std::env::current_exe().unwrap()
                        .parent().unwrap()
                        .into();
    static ref CURRENT_DIR: PathBuf = std::env::current_dir().unwrap()
                        .into();
    static ref GL_ID: String = {
        // NB must be run after GL initialized
        let mut gl_str = String::from("");
        unsafe {
            let version = gl::GetString(gl::VERSION);
            for i in 0..12 {
                if version.add(i).is_null() || *version.add(1) == 0 {
                    break;
                }
                gl_str.push(*version.add(i) as char);
            }
        }
        let mut gl_id = String::from("GL");
        if gl_str.contains("ES") {
            gl_id.push_str("ES");
        }
        for s in gl_str.split(" ") {
            if s.contains(".") {
                for n in s.split(".").take(2) {
                    gl_id.push_str(n);
                }
                break;
            }
        }
        gl_id
    };
}

pub mod buffer;
pub mod camera;
pub mod display;
pub mod shader;
#[macro_use]
pub mod shape;
pub mod texture;

pub mod util;
pub mod shaders;
pub mod shapes;

