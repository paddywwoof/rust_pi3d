extern crate pi3d;
extern crate sdl2;
use sdl2::keyboard::Keycode;

fn main() {
    let mut display = pi3d::display::create("experimental window", 800.0, 600.0);
    display.set_background(&[0.1, 0.1, 0.2, 1.0]);
    let shader_program = pi3d::shader::Program::from_res(
          &display, "uv_light").unwrap();
    let mut camera = pi3d::camera::create(&display);
    let texture = pi3d::texture::create_from_file(&display, "textures/pattern.png");

    let mut candlestick = pi3d::shapes::lathe::create(vec![[0.0, 2.0], [0.1, 1.8], [0.1, 1.2],
            [0.5, 1.0], [0.6, 0.6], [0.2, 0.5], [0.2, 0.2], [1.0, 0.1], [1.2, -0.3], [0.0, -2.0]],
            36, 0.0, 1.0);
    candlestick.set_draw_details(&shader_program, &vec![texture.id, texture.id], 1.0, 0.0, 1.0, 1.0, 1.0);
    candlestick.position(&[-2.0, 1.0, 5.0]);

    let mut cube = pi3d::shapes::cuboid::create(1.5, 1.0, 0.5, 1.0, 1.0, 1.0);
    cube.buf[0].set_textures(&vec![texture.id, texture.id]);
    cube.set_shader(&shader_program);
    cube.position_z(2.5);

    let mut cube2 = pi3d::shapes::cuboid::create(3.0, 2.0, 1.0, 1.0, 1.0, 1.0);
    cube2.set_draw_details(&shader_program, &vec![texture.id, texture.id], 2.0, 1.0, 2.0, 3.0, 1.0);
    cube2.position_z(3.6);

    cube2.set_light(0, &[1.5, 1.5, 4.0], &[10.0, 10.0, 10.0], &[0.05, 0.1, 0.05], true);

    let mut t: f32 = 0.0;
    while display.loop_running() {
        t += 0.02;
        cube.rotate_inc_x(0.01);
        cube.rotate_inc_z(0.031);
        cube.position(&[t * 0.087 % 2.2 - 1.1, t * 0.12 % 1.98 - 0.9, 2.5]);

        cube2.rotate_inc_z(0.009);
        cube2.position_x((t + 0.7) * 0.047 % 3.2 - 1.01);
        cube2.position_y(t * 0.092 % 1.48 - 0.8);

        candlestick.rotate_inc_x(0.05);

        cube.draw(&mut camera);
        cube2.draw(&mut camera);
        candlestick.draw(&mut camera);

        if display.keys_pressed.contains(&Keycode::Escape) {break;}
        if display.keys_down.contains(&Keycode::A) {cube.rotate_inc_y(0.07);}
        if display.mouse_moved {
            cube2.rotate_to_x(display.mouse_y as f32 * 0.01);
            cube2.rotate_to_y(display.mouse_x as f32 * 0.01);
        }
        if display.keys_pressed.contains(&Keycode::L) {candlestick.buf[0].set_line_width(5.0, true, false);}
        if display.keys_pressed.contains(&Keycode::F) {candlestick.buf[0].set_line_width(0.0, true, false);}
        if display.keys_pressed.contains(&Keycode::P) {candlestick.buf[0].set_point_size(30.0);}
    }
}
