extern crate pi3d;

fn main() {
    let mut display = pi3d::display::create("experimental window", 800.0, 600.0);
    let shader_program = pi3d::shader::Program::from_res(
          &display, "shaders/triangle").unwrap();
    let mut camera = pi3d::camera::create(&display);
    let texture = pi3d::texture::create_from_file("target/release/textures/pattern.png");
    let mut cube = pi3d::shapes::cuboid::create(0.2, 0.7, 0.4, 1.0, 1.0, 1.0);
    cube.buf[0].set_textures(&vec![texture.id]);
    cube.set_shader(&shader_program);
    cube.position_z(2.5);

    let mut cube2 = pi3d::shapes::cuboid::create(0.8, 0.6, 1.5, 1.0, 1.0, 1.0);
    cube2.set_shader(&shader_program);
    cube2.position_z(3.6);

    cube2.set_light(0, &[1.5, 1.5, 4.0], &[10.0, 10.0, 10.0], &[0.05, 0.1, 0.05], true);

    let mut t: f32 = 0.0;

    while display.loop_running() {
        t += 0.02;
        cube.rotate_inc_x(0.01);
        cube.rotate_inc_y(0.0173);
        cube.rotate_inc_z(0.031);
        cube.position_x(t * 0.087 % 2.2 - 1.1);
        cube.position_y(t * 0.12 % 1.98 - 0.9);
        cube2.rotate_inc_x(0.005);
        cube2.rotate_inc_y(0.007);
        cube2.rotate_inc_z(0.009);
        cube2.position_x((t + 0.7) * 0.047 % 3.2 - 1.01);
        cube2.position_y(t * 0.092 % 1.48 - 0.8);

        cube.draw(&mut camera);
        cube2.draw(&mut camera);
    }
}
