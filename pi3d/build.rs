extern crate fs_extra;

use std::env;

fn main() {
    let host_triple = env::var("HOST").unwrap();
    let targ_triple = env::var("TARGET").unwrap();
    let target_dir = if targ_triple == host_triple {
        "".to_string()
    } else {
        format!("{}/", targ_triple)
    };
    let profile = env::var("PROFILE").unwrap(); //ie debug or release
    let out_dir = format!("target/{}{}/examples", target_dir, profile);
    println!("this out_dir={:?}", out_dir);
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;

    let mut from_paths = vec![
        "examples/textures",
        "examples/shaders",
        "examples/models",
        "examples/fonts",
        "examples/ecubes",
        "examples/cities",
    ];
    if targ_triple.find("windows") != None {
        // will need dll for windows
        from_paths.push("../SDL2.dll");
    }
    println!("{:?}", from_paths);
    fs_extra::copy_items(&from_paths, &out_dir, &options).unwrap();
}
