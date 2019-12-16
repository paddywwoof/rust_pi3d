extern crate pi3d;

const W:f32 = 900.0; // these are overwritten by fullscreen option
const H:f32 = 700.0;

fn main() {
    // initially set up display, shader, camera, texture and shapes
    let mut display = pi3d::display::create("minimal example ESC to quit", W, H, "GL", 2, 1).unwrap();
            display.set_target_fps(5.0); // nothing happens so no point running faster
    let shader = pi3d::shader::Program::from_res("uv_flat").unwrap();
    let mut camera = pi3d::camera::create(&display);
            camera.set_3d(false); // make it a 2D shader
    let tex = pi3d::texture::create_from_file("textures/pattern.png");
    let mut plane = pi3d::shapes::plane::create(camera.reference(), display.height * 0.7, display.height * 0.7);
            plane.set_draw_details(&shader, &vec![tex.id], 1.0, 0.0, 1.0, 1.0, 0.0);

    // draw in a loop
    while display.loop_running() { // default sdl2 check for ESC or click cross
        plane.draw();
    }
}
