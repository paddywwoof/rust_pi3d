extern crate pi3d;
extern crate sdl2;
extern crate rand;
use sdl2::keyboard::Keycode;

const W:f32 = 960.0;
const H:f32 = 720.0;

fn main() {
    let mut display = pi3d::display::create("experimental game window", W, H, "GL", 2, 1).unwrap();
            display.set_background(&[0.1, 0.1, 0.2, 1.0]);
            display.set_mouse_relative(true);
            display.set_target_fps(30.0);
    let shader_program = pi3d::shader::Program::from_res("uv_reflect").unwrap();
    let flatsh = pi3d::shader::Program::from_res("uv_flat").unwrap();
    let textsh = pi3d::shader::Program::from_res("uv_pointsprite").unwrap();
    let mut camera = pi3d::camera::create(&display);
    let mut camera2d = pi3d::camera::create(&display);
            camera2d.set_3d(false);

    let tex = pi3d::texture::create_from_file("textures/pattern.png");
    let maptex = pi3d::texture::create_from_file("textures/mountains3_512.jpg");
    let mapnorm = pi3d::texture::create_from_file("textures/grasstile_n.jpg");
    let stars = pi3d::texture::create_from_file("textures/stars.jpg");
    let font = pi3d::util::font::create("fonts/NotoSans-Regular.ttf", "", "ęĻ", 64.0);
    let mut mystring = pi3d::shapes::string::create(camera2d.reference(), &font, "\"The quick brown
fox `jumps`
over thę Ļazy
dog\"", 0.0);
            mystring.set_shader(&flatsh);
            mystring.position_z(2.0);
    
    let mut candlestick = pi3d::shapes::lathe::create(camera.reference(), vec![[0.0, 2.0], [0.1, 1.8], [0.1, 1.2],
            [0.5, 1.0], [0.6, 0.6], [0.2, 0.5], [0.2, 0.2], [1.0, 0.1], [1.2, -0.3], [0.0, -2.0]],
            144, 0.0, 1.0);
            candlestick.set_draw_details(&shader_program, &vec![tex.id, mapnorm.id, stars.id], 1.0, 0.1, 1.0, 1.0, 1.0);
            candlestick.position(&[-2.0, 30.0, 15.0]);
            candlestick.set_material(&[1.0, 0.0, 0.0]);

    let sphere = pi3d::shapes::sphere::create(camera.reference(), 1.5, 16, 32, 0.3, false).reference();
        sphere.borrow_mut().set_textures(&vec![tex.id, mapnorm.id]);
        sphere.borrow_mut().set_shader(&shader_program);

    let mut cube2 = pi3d::shapes::cuboid::create(camera.reference(), 3.0, 2.0, 1.0, 1.0, 1.0, 1.0);
            cube2.set_draw_details(&shader_program, &vec![tex.id, mapnorm.id, stars.id], 2.0, 0.1, 2.0, 3.0, 1.0);
            cube2.set_light(0, &[1.5, 1.5, 4.0], &[10.0, 10.0, 10.0], &[0.05, 0.1, 0.05], true);
            cube2.add_child(sphere.clone());

    let mut junk = pi3d::shapes::merge_shape::create(camera.reference());
    pi3d::shapes::merge_shape::add_shapes(&mut junk,
     vec![&cube2, &cube2, &candlestick],
     vec![[2.0, 1.0, 1.0], [-1.0, -1.0, 1.0], [0.0, 0.0, -0.5]],
     vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [0.0, 0.0, 0.0]],
     vec![[1.0, 1.0, 1.0], [0.5, 2.0, 2.0], [2.2, 2.2, 2.2]],
     vec![0, 0, 1]);
    junk.buf[1].set_material(&[1.0, 1.0, 0.0, 1.0]);
    junk.position(&[1.0, 30.0, 7.5]);

    let mut map = pi3d::shapes::elevation_map::new(camera.reference(), "textures/mountainsHgt.png", 400.0, 400.0, 50.0, 64, 64, 1.0, "nothing");
            map.shape.set_draw_details(&shader_program, &vec![maptex.id, mapnorm.id, stars.id], 128.0, 0.0, 1.0, 1.0, 2.0);

    let (mut iss, _texlist) = pi3d::shapes::model_obj::create(camera.reference(), "models/iss.obj");
            iss.set_shader(&shader_program);
            iss.set_normal_shine(&vec![mapnorm.id, stars.id], 16.0, 0.1, 1.0, 1.0, 0.1, true);
            iss.position(&[20.0, 50.0, 10.0]);
            iss.scale(&[40.0, 40.0, 40.0]);

    let mut clust = pi3d::shapes::merge_shape::create(camera.reference(), );
    pi3d::shapes::merge_shape::cluster(&mut clust, &cube2, &map, -20.0, -100.0,
            200.0, 150.0, 0.5, 2.5, 200);

    let (mut ecube, _tex_list) = pi3d::shapes::environment_cube::create(camera.reference(),
                 500.0, "ecubes/miramar_256", "png");
    ecube.set_shader(&flatsh);

    // fps counter
    let mut fps_text = pi3d::shapes::point_text::create(camera2d.reference(), &font, 20, 24.0);
    fps_text.set_shader(&textsh);
    let fps_blk = fps_text.add_text_block(&font, &[-W * 0.5 + 20.0, -H * 0.5 + 20.0, 0.1], 19, "00.0 FPS");

    let mut t: f32 = 0.0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = -0.1;
    let mut ds:f32 = 0.01;
    let mut rot: f32 = 0.0;
    let mut tilt: f32 = 0.0;

    while display.loop_running() {
        t += 0.02;

        sphere.borrow_mut().rotate_inc_y(0.01);
        sphere.borrow_mut().rotate_inc_z(0.031);
        sphere.borrow_mut().position(&[(t * 0.087 % 2.2 - 1.1).abs(), (t * 0.12 % 5.98 -2.99).abs() + 4.0, 2.5]);

        cube2.rotate_inc_z(0.009);
        cube2.position_x(((t + 0.7) * 0.047 % 5.2 - 2.6).abs() - 1.01);
        cube2.position_y((t * 0.092 % 3.48 - 1.74).abs() + 30.7);

        candlestick.rotate_inc_x(0.05);
    
        junk.rotate_inc_y(0.1);

        iss.rotate_inc_y(0.01);
        iss.rotate_inc_x(0.007);

        for i in 0..clust.buf[0].array_buffer.shape()[0] {
            clust.buf[0].array_buffer[[i, 1]] *= 0.9995 + 0.001 * rand::random::<f32>();
        }
        clust.buf[0].re_init();

        ecube.draw();
        cube2.draw();
        candlestick.draw();
        junk.draw();
        map.shape.draw();
        iss.draw();
        clust.draw();
        mystring.draw();
        fps_text.set_text(&font, fps_blk, &format!("{:5.1} FPS", display.fps()));
        fps_text.draw();

        if display.keys_pressed.contains(&Keycode::Escape) {break;}
        if display.keys_down.contains(&Keycode::A) {cube2.offset(&[t % 3.0, 0.0, 0.0]);}
        if display.mouse_moved {
            tilt = (display.mouse_y as f32 - 300.0) * -0.004;
            rot = (display.mouse_x as f32 - 400.0) * -0.004;
        }
        if display.keys_pressed.contains(&Keycode::L) {candlestick.buf[0].set_line_width(2.0, true, false);}
        if display.keys_pressed.contains(&Keycode::F) {candlestick.buf[0].set_line_width(0.0, true, false);}
        if display.keys_pressed.contains(&Keycode::P) {candlestick.buf[0].set_point_size(3.0);}
        if display.keys_pressed.contains(&Keycode::W) {ds = 1.25;}
        if display.keys_pressed.contains(&Keycode::S) {ds = -0.25;}
        let cd = camera.get_direction();
        x += cd[0] * ds; y += cd[1] * ds; z += cd[2] * ds;
        camera.reset();
        camera.rotate(&[tilt, rot, 0.0]);
        if ds != 0.0 {
            let (newy, _mapnorm) = map.calc_height(x, z);
            y = newy + 5.0;
            camera.position(&[x, y, z]);
        }
        ds = 0.0;
        if display.was_resized() {
            camera.set_lens_from_display(&display);
            camera2d.set_lens_from_display(&display);
            fps_text.set_position(&font, fps_blk,
                     &[-display.width * 0.5 + 20.0, -display.height * 0.5 + 20.0, 0.1]);
        }
    }
}
