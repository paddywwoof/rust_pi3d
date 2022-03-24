extern crate pi3d;
extern crate rand;
extern crate sdl2;
use sdl2::keyboard::Keycode;

const W: f32 = 960.0;
const H: f32 = 960.0;

fn main() {
    let mut display =
        pi3d::display::create("ship_demo (C Tim Skillman) window", W, H, "GL", 2, 1).unwrap();
    display.set_background(&[0.1, 0.1, 0.2, 1.0]);
    display.set_mouse_relative(true);
    display.set_target_fps(1000.0);
    let flatsh = pi3d::shader::Program::from_res("uv_flat").unwrap();
    let textsh = pi3d::shader::Program::from_res("uv_pointsprite").unwrap();
    let mut camera = pi3d::camera::create(&display);
    let mut camera2d = pi3d::camera::create(&display);
    camera2d.set_3d(false);

    let font = pi3d::util::font::create("fonts/NotoSans-Regular.ttf", "", "ęĻ", 64.0);

    let (mut cargo_hold, _texlist) =
        pi3d::shapes::model_obj::create(camera.reference(), "models/CargoHoldBaked2.obj");
    cargo_hold.set_shader(&flatsh);
    cargo_hold.set_material(&[0.4, 0.4, 0.4]);
    cargo_hold.set_fog(&[0.3, 0.3, 0.3], 200.2, 1.0);
    let mut radar_num: usize = 0;
    for k in 0..cargo_hold.buf.len() {
        if cargo_hold.buf[k].array_buffer[[0, 0]] == 97.226921 {
            radar_num = k;
            break;
        }
    }

    let (mut ecube, _tex_list) = pi3d::shapes::environment_cube::create(
        camera.reference(),
        500.0,
        "models/maps/sbox_512",
        "png",
    );
    ecube.set_shader(&flatsh);

    // fps counter
    let mut fps_text = pi3d::shapes::point_text::create(camera2d.reference(), &font, 20, 24.0);
    fps_text.set_shader(&textsh);
    let fps_blk = fps_text.add_text_block(
        &font,
        &[-W * 0.5 + 20.0, -H * 0.5 + 20.0, 0.1],
        19,
        "00.0 FPS",
    );

    let mut n: usize = 0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = -0.1;
    let mut df: f32 = 0.01;
    let mut ds: f32 = 0.0;
    let mut rot: f32 = 0.0;
    let mut tilt: f32 = 0.0;

    while display.loop_running() {
        ecube.draw();
        cargo_hold.draw();
        fps_text.set_text(&font, fps_blk, &format!("{:5.1} FPS", display.fps()));
        fps_text.draw();

        n += 1;
        if n > 5 {
            cargo_hold.buf[radar_num].unib[[3, 0]] += 0.083333;
            if cargo_hold.buf[radar_num].unib[[3, 0]] > 1.0 {
                cargo_hold.buf[radar_num].unib[[3, 0]] = 0.0;
            }
            n = 0;
        }

        if display.keys_pressed.contains(&Keycode::Escape) {
            break;
        }
        if display.mouse_moved {
            tilt = (display.mouse_y as f32 - 300.0) * 0.005;
            rot = (display.mouse_x as f32 - 400.0) * 0.005;
        }
        if display.keys_pressed.contains(&Keycode::W) {
            df = 0.5
        }
        if display.keys_pressed.contains(&Keycode::S) {
            df = -0.25;
        }
        if display.keys_pressed.contains(&Keycode::A) {
            ds = 0.25;
        }
        if display.keys_pressed.contains(&Keycode::D) {
            ds = -0.25;
        }
        let cd = camera.get_direction();
        x += cd[0] * df - cd[2] * ds;
        y += cd[1] * df;
        z += cd[2] * df + cd[0] * ds;
        camera.reset();
        camera.rotate(&[tilt, rot, 0.0]);
        if df != 0.0 || ds != 0.0 {
            //let (newy, _mapnorm) = pi3d::shapes::elevation_map::calc_height(&map, x, z);
            //y = newy + 5.0;
            camera.position(&[x, y, z]);
        }
        ds = 0.0;
        df = 0.0;
    }
}

// ==========================================================================
// ship_demo models and image files used by this code made by Tim Skillman
// ==========================================================================
//
// The MIT License
//
// Copyright (c) 2019 Tim Skillman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
// Dedicated to the One who has blessed me beyond my dreams ...
// Jesus Christ, King of Kings and Lord of Lords :-)
// =======================================================================
