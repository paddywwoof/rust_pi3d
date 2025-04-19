use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::shaders::built_in_shaders::{CODES, NAMES};
use crate::{EXE_PATH, CURRENT_DIR};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
    MissingResource,
    RecursionDepth,
    WindowBuildError { name: String },
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub fn load_string(resource_name: &str) -> Result<String, Error> {
    let mut listing = Vec::<String>::new();
    load_includes(resource_name, &mut listing, 0)?;
    Ok(listing.join("\n"))
}
/*
pub fn resource_name_to_path(location: &str) -> PathBuf {
    let new_path = PathBuf::from(location);
    let mut path = PathBuf::new();
    if !new_path.has_root() { // only start from exe root path if not /
        path = (*EXE_PATH).to_path_buf(); //why does it need this (clone says it's a Path)
    }
    path.join(new_path)
}
*/
pub fn resource_name_to_path(location: &str) -> PathBuf {
    //let new_path = PathBuf::from(location);
    let mut exe_path = (*EXE_PATH).to_path_buf();
    exe_path.push(location);
    let mut cur_path = (*CURRENT_DIR).to_path_buf();
    cur_path.push(location);
    if cur_path.is_file() {
        return cur_path;
    }
    if exe_path.is_file() {
        return exe_path;
    }
    if cur_path.is_dir() {
        return cur_path;
    }
    exe_path
}

fn load_includes(resource_name: &str, listing: &mut Vec<String>, depth: u32) -> Result<(), Error> {
    if depth > 16 {
        return Err(Error::RecursionDepth);
    }
    let mut text_chunk = String::new();
    for (i, name) in NAMES.iter().enumerate() {
        // first try built_in_shaders
        if *name == resource_name.trim() {
            text_chunk = CODES[i].to_string();
            break;
        }
    }
    if text_chunk.is_empty() {
        // now check file path
        let path_buf = resource_name_to_path(resource_name);
        if !path_buf.is_file() {
            return Err(Error::MissingResource);
        } // nope
        let mut file = fs::File::open(path_buf).unwrap();
        file.read_to_string(&mut text_chunk)?;
    }
    if text_chunk.is_empty() {
        return Err(Error::MissingResource); // still not got anything so stop now
    }
    for s in (&text_chunk).lines() {
        match s.find("#include") {
            Some(ix) => {
                let (_, new_key) = s.split_at(ix + 9);
                load_includes(new_key, listing, depth + 1)?;
            }
            None => {
                listing.push(s.to_string());
            }
        }
    }
    Ok(())
}
