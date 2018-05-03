extern crate pi3d;

fn main() {
    let mut display = pi3d::display::create("experimental window");
    let shader_program = pi3d::shader::Program::from_res(
          &display, "shaders/triangle").unwrap();
    let mut cube = pi3d::shape::cuboid(0.2, 0.7, 0.4);
    cube.set_shader(&shader_program);
    cube.position_z(0.5);

    let mut cube2 = pi3d::shape::cuboid(0.8, 0.6, 0.5);
    cube2.set_shader(&shader_program);
    cube2.position_z(0.6);

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

        cube.draw();
        cube2.draw();
    }
}
