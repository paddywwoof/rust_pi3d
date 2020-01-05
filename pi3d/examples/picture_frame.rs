extern crate pi3d;
extern crate sdl2;
extern crate rand;

use std::path::Path;
use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::time::{Instant, Duration};
//use std::thread::sleep;

const IMG_EXT: [&str;6] = ["jpg", "JPG", "jpeg", "JPEG", "png", "PNG"];
const FILE_DIR: &str = "./";
//const FILE_DIR: &str = "/home/patrick/Pictures"; // or absolute path

// recursive function to build list of image files. Pass ref to vec to build in place
fn get_files(dir: &Path, file_list: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() { // will panic if dir is not a Path
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.is_dir() {
                    get_files(&path, file_list); // recurse into dir
                } else { // it's a file, check if image TODO exif dates and rotation
                    let path_str = path.to_str().unwrap().to_string();
                    let extension = path_str.split(".").last().unwrap();
                    if IMG_EXT.iter().any(|&x| x == extension) {
                        file_list.push(path_str);
                    }
                }
            },
            _ => {
                println!("odd entry in directory, permission?");
            },
        }
    }
}

fn main() {
    let time_delay = Duration::new(10, 0); // duration per slide in (seconds, nanoseconds)
    let fade_time = Duration::new(5, 0);

    // initially set up display, shader, camera, texture and shapes
    let mut display = pi3d::display::create("picture_frame ESC to quit", 100.0, 100.0, "GL", 2, 1).unwrap();
            display.set_fullscreen(true);
            display.set_background(&[0.2, 0.2, 0.2, 1.0]);
            display.set_target_fps(30.0);
    let shader = pi3d::shader::Program::from_res("shaders/blend_new").unwrap();
    let mut camera = pi3d::camera::create(&display);
            camera.set_3d(false); // make it a 2D shader
    let (w, h) = display.get_size();
    let mut slide = pi3d::shapes::plane::create(camera.reference(), w as f32, h as f32); // fullscreen
            slide.set_draw_details(&shader, &vec![], 1.0, 1.0, 1.0, 1.0, 1.0);
            //slide.position_z(5.0);

    // pi3d has a resources function that checks if path has root
    let file_path = pi3d::util::resources::resource_name_to_path(FILE_DIR);
    let mut file_list: Vec<String> = vec![];
    get_files(&file_path, &mut file_list);
    let mut rng = thread_rng();
    file_list.shuffle(&mut rng);

    let mut sbg:Option<pi3d::texture::Texture> = None; //Options used to cater for bad image files later
    let mut sfg:Option<pi3d::texture::Texture> = None;

    let mut last_tm = Instant::now().checked_sub(time_delay).unwrap();
    let mut pic_num = 0usize;
    // draw in a loop
    while display.loop_running() { // default sdl2 check for ESC or click cross
        let delta_tm = last_tm.elapsed();
        if delta_tm.as_secs() > time_delay.as_secs() {
            let mut loop_i = 0;
            loop {
                loop_i += 1;
                if sfg.is_some() {
                    sbg = sfg.take(); // leave None in its place
                }
                if !sfg.is_some() { // as could have been set to None
                    let mut tex = pi3d::texture::create_from_file(file_list[pic_num].as_str());
                            tex.set_mirrored_repeat(true);
                    sfg = Some(tex);
                    pic_num += 1;
                    if pic_num >= file_list.len() {
                        pic_num = 0;
                    }
                }
                if (sfg.is_some() && sbg.is_some()) || loop_i > 5 {break;}
            }
            let sfg_tex = sfg.expect("missing f_gnd image"); // unwrap them here to use contained info
            let sbg_tex = sbg.expect("missing bk_gnd image");
            slide.set_textures(&vec![sfg_tex.id, sbg_tex.id]);
            let w = sfg_tex.width;
            let h = sfg_tex.height;
            sfg = Some(sfg_tex); // wrapped up again for next loop
            sbg = Some(sbg_tex);
            slide.unif[[15, 0]] = slide.unif[[14, 0]]; // copy forgrnd to bkgrnd
            slide.unif[[15, 1]] = slide.unif[[14, 1]];
            slide.unif[[17, 0]] = slide.unif[[16, 0]];
            slide.unif[[17, 1]] = slide.unif[[16, 1]];
            let wh_rat = (display.width * h as f32) / (display.height * w as f32);
            let (ix0, ix1) = if wh_rat > 1.0 {
                (0, 1)
            } else {
                (1, 0)
            };
            slide.unif[[14, ix0]] = wh_rat;
            slide.unif[[14, ix1]] = 1.0;
            slide.unif[[16, ix0]] = (wh_rat - 1.0) * 0.5;
            slide.unif[[16, ix1]] = 0.0;

            slide.unif[[14, 2]] = 0.0;
            slide.unif[[15, 2]] = 0.5;
            last_tm = Instant::now();
        } else {
            let a: f32 = delta_tm.as_millis() as f32 / fade_time.as_millis() as f32;
            slide.unif[[14, 2]] = if a < 1.0 {a} else {1.0};
        }
        slide.draw();
        //TODO sleep while nothing's changing?
    }
}
