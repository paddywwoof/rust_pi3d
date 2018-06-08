extern crate pi3d;
extern crate sdl2;
use sdl2::keyboard::Keycode;
use std::time;

fn main() {
    let mut display = pi3d::display::create("experimental window", 960.0, 960.0);
    display.set_background(&[0.1, 0.1, 0.2, 1.0]);
    display.set_mouse_relative(true);
    let shader_program = pi3d::shader::Program::from_res(
          &display, "uv_reflect").unwrap();
    let flatsh = pi3d::shader::Program::from_res(
          &display, "uv_flat").unwrap();
    let mut camera = pi3d::camera::create(&display);

    let tex = pi3d::texture::create_from_file(&display, "textures/pattern.png");
    let maptex = pi3d::texture::create_from_file(&display, "textures/mountains3_512.jpg");
    let mapnorm = pi3d::texture::create_from_file(&display, "textures/grasstile_n.jpg");
    let stars = pi3d::texture::create_from_file(&display, "textures/stars.jpg");
    let myfont = pi3d::util::font::create(&display, "fonts/NotoSans-Regular.ttf", "", "", 48.0);
    let mut mystring = pi3d::shapes::string::create(&myfont, "\"The quick brown
fox `jumps`
over the !azy
dog\"", 0.0);
    mystring.set_shader(&flatsh);
    mystring.position_z(2.0);
    let mut camera2d = pi3d::camera::create(&display);
    camera2d.set_3d(false);
    
    let mut candlestick = pi3d::shapes::lathe::create(vec![[0.0, 2.0], [0.1, 1.8], [0.1, 1.2],
            [0.5, 1.0], [0.6, 0.6], [0.2, 0.5], [0.2, 0.2], [1.0, 0.1], [1.2, -0.3], [0.0, -2.0]],
            144, 0.0, 1.0);
    candlestick.set_draw_details(&shader_program, &vec![tex.id, mapnorm.id, stars.id], 1.0, 0.1, 1.0, 1.0, 1.0);
    candlestick.position(&[-2.0, 30.0, 15.0]);
    candlestick.set_material(&[1.0, 0.0, 0.0]);

    let mut sphere = pi3d::shapes::sphere::create(1.5, 16, 32, 0.3, false);
    sphere.buf[0].set_textures(&vec![tex.id, mapnorm.id]);
    sphere.set_shader(&shader_program);

    let mut cube2 = pi3d::shapes::cuboid::create(3.0, 2.0, 1.0, 1.0, 1.0, 1.0);
    cube2.set_draw_details(&shader_program, &vec![tex.id, mapnorm.id, stars.id], 2.0, 0.1, 2.0, 3.0, 1.0);
    cube2.set_light(0, &[1.5, 1.5, 4.0], &[10.0, 10.0, 10.0], &[0.05, 0.1, 0.05], true);
    cube2.add_child(sphere);

    let mut junk = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::add_shapes(&mut junk,
     vec![&cube2, &cube2, &candlestick],
     vec![&[2.0, 1.0, 1.0], &[-1.0, -1.0, 1.0], &[0.0, 0.0, -0.5]],
     vec![&[0.0, 1.0, 0.0], &[1.0, 1.0, 0.0], &[0.0, 0.0, 0.0]],
     vec![&[1.0, 1.0, 1.0], &[0.5, 2.0, 2.0], &[2.2, 2.2, 2.2]],
     vec![0, 0, 1]);
    junk.buf[1].set_material(&[1.0, 1.0, 0.0, 1.0]);
    junk.position(&[1.0, 30.0, 7.5]);

    let mut map = pi3d::shapes::elevation_map::create(&display, "textures/mountainsHgt.png", 400.0, 400.0, 50.0, 64, 64, 1.0, "nothing");
    map.set_draw_details(&shader_program, &vec![maptex.id, mapnorm.id, stars.id], 128.0, 0.0, 1.0, 1.0, 2.0);

    let (mut iss, _texlist) = pi3d::shapes::model_obj::create(&display, "models/iss.obj");
    iss.set_shader(&shader_program);
    iss.set_normal_shine(&vec![mapnorm.id, stars.id], 16.0, 0.1, 1.0, 1.0, 0.1, true);
    iss.position(&[20.0, 50.0, 10.0]);
    iss.scale(&[40.0, 40.0, 40.0]);

    let mut clust = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::cluster(&mut clust, &cube2, &map, -20.0, -100.0,
            200.0, 150.0, 0.5, 2.5, 200);

    let (mut ecube, _tex_list) = pi3d::shapes::environment_cube::create(&display, 500.0,
                "ecubes/miramar_256", "png");
    ecube.set_shader(&flatsh);

    let mut t: f32 = 0.0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = -0.1;
    let mut ds:f32 = 0.01;
    let mut rot: f32 = 0.0;
    let mut tilt: f32 = 0.0;
    let mut frames: f32 = 0.0;
    let start = time::Instant::now();
    while display.loop_running() {
        frames += 1.0;
        t += 0.02;

        cube2.children[0].rotate_inc_y(0.01);
        cube2.children[0].rotate_inc_z(0.031);
        cube2.children[0].position(&[t * 0.087 % 2.2 - 1.1, t * 0.12 % 5.98 + 4.0, 2.5]);

        cube2.rotate_inc_z(0.009);
        cube2.position_x((t + 0.7) * 0.047 % 3.2 - 1.01);
        cube2.position_y(t * 0.092 % 1.48 + 30.7);

        candlestick.rotate_inc_x(0.05);

        junk.rotate_inc_y(0.1);

        iss.rotate_inc_y(0.01);
        iss.rotate_inc_x(0.007);

        ecube.draw(&mut camera);
        cube2.draw(&mut camera);
        candlestick.draw(&mut camera);
        junk.draw(&mut camera);
        map.draw(&mut camera);
        iss.draw(&mut camera);
        clust.draw(&mut camera);
        mystring.draw(&mut camera2d);

        if display.keys_pressed.contains(&Keycode::Escape) {break;}
        if display.keys_down.contains(&Keycode::A) {cube2.offset(&[t % 3.0, 0.0, 0.0]);}
        if display.mouse_moved {
            tilt = (display.mouse_y as f32 - 300.0) * 0.01;
            rot = (display.mouse_x as f32 - 400.0) * 0.01;
        }
        if display.keys_pressed.contains(&Keycode::L) {candlestick.buf[0].set_line_width(2.0, true, false);}
        if display.keys_pressed.contains(&Keycode::F) {candlestick.buf[0].set_line_width(0.0, true, false);}
        if display.keys_pressed.contains(&Keycode::P) {candlestick.buf[0].set_point_size(30.0);}
        if display.keys_pressed.contains(&Keycode::W) {ds = 1.25}
        if display.keys_pressed.contains(&Keycode::S) {ds = -0.25;}
        let cd = camera.get_direction();
        x += cd[0] * ds; y += cd[1] * ds; z += cd[2] * ds;
        camera.reset();
        camera.rotate(&[tilt, rot, 0.0]);
        if ds != 0.0 {
            let (newy, _mapnorm) = pi3d::shapes::elevation_map::calc_height(&map, x, z);
            y = newy + 5.0;
            camera.position(&[x, y, z]);
        }
        ds = 0.0;
    }
    println!("{:?} FPS", frames / start.elapsed().as_secs() as f32);
}
