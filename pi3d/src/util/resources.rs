use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};
use std::ffi;


use ::shaders::built_in_shaders::NAMES;
use ::shaders::built_in_shaders::CODES;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
    MissingResource,
    RecursionDepth,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut listing = Vec::<String>::new();
        self.load_includes(&resource_name, &mut listing, 0)?;
        let buffer: Vec<u8> = listing.join("\n").as_bytes().to_vec();
        ffi::CString::new(buffer).map_err(|_e| Error::FileContainsNil)
    }

    pub fn resource_name_to_path(&self, location: &str) -> PathBuf {
        //TODO if location starts with '/' use fs root rather than exe root
        let mut path: PathBuf = self.root_path.clone();
        for part in location.split("/") {
            path = path.join(part);
        }
        path
    }

    fn load_includes(&self, resource_name: &str, mut listing: &mut Vec<String>, depth: u32) -> Result<(), Error> {
        if depth > 16 {return Err(Error::RecursionDepth);}
        let mut text_chunk = String::new();
        for (i, name) in NAMES.iter().enumerate() { // first try built_in_shaders
            if *name == resource_name.trim() {
                text_chunk = CODES[i].to_string();
                break;
            }
        }
        if text_chunk == "" { // now check file path
            let path_buf = self.resource_name_to_path(resource_name);
            if !path_buf.is_file() {return Err(Error::MissingResource);} // nope
            let mut file = fs::File::open(path_buf).unwrap();
            file.read_to_string(&mut text_chunk)?;
        }
        if text_chunk == "" {return Err(Error::MissingResource);} // still not got anything
        for s in (&text_chunk).lines() {
            match s.find("#include") { 
                Some(ix) => {
                    let (_, new_key) = s.split_at(ix + 9);
                    self.load_includes(&new_key, &mut listing, depth + 1)?;
                },
                None => {
                    listing.push(s.to_string());
                }
            }
        }
        Ok(())
    }
}

pub fn from_exe_path() -> Result<Resources, Error> {
    //! creates a Resource object containing the root path to the exe
    //! that's running
    let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
    let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
    Ok(Resources { root_path: exe_path.into() })
}

