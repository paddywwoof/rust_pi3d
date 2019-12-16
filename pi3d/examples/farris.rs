extern crate pi3d;
extern crate sdl2;
use sdl2::keyboard::Keycode;

const W:f32 = 1200.0;
const H:f32 = 720.0;
const UNIF_OFFSET: usize = 11; // first unused row of unif
const PARAMS: &[[f32;3];8] = &[[1.0, 4.0, -0.2], [0.2, -5.0, -2.0], [0.1, 0.2,   //L-n1,m1,ar1,ai1,n2,m2,ar2,ai2
                                  1.0], [4.0, -0.2, 0.2], [-5.0, -2.0, 0.1], [0.2, //L-n3,m3,ar3,ai3 R-n1,m1,ar1,ai1
                                  1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];     //R-n2,n2,m2,ar2,ai2,n3,m3,ar3,ai3

fn reset_params(obj: &mut pi3d::shape::Shape) {
    for i in 0..PARAMS.len() {
        for j in 0..3 {
            obj.unif[[UNIF_OFFSET + i, j]] = PARAMS[i][j];
        }
    }
}

fn main() {
    // setup display
    let mut display = pi3d::display::create("Farris page 67", W, H, "GL", 2, 1).unwrap();
            display.set_background(&[0.1, 0.1, 0.2, 1.0]);
            display.set_mouse_relative(true);
            display.set_target_fps(1000.0);

    // shaders
    let shader = pi3d::shader::Program::from_res("shaders/farris_p67b").unwrap();
    let textsh = pi3d::shader::Program::from_res("uv_pointsprite").unwrap();
    //let shader = pi3d::shader::Program::from_res("shaders/farris_p67b_ES30").unwrap();
    //let textsh = pi3d::shader::Program::from_res("shaders/uv_pointsprite_ES30").unwrap();

    // cameras
    let mut camera = pi3d::camera::create(&display);
    let mut camera2d = pi3d::camera::create(&display);
            camera2d.set_3d(false);

    // textures
    let tex = pi3d::texture::create_from_file("textures/poppy1.jpg");

    // cube
    let mut cube = pi3d::shapes::cuboid::create(camera.reference(), 10.0, 10.0, 10.0, 1.0, 1.0, 1.0);
            cube.set_draw_details(&shader, &vec![tex.id], 1.0, 0.0, 0.1, 0.1, 0.0);
            cube.position_z(50.0);
            cube.buf[0].unib[[3, 0]] = 0.05;

    // plane
    let mut plane = pi3d::shapes::plane::create(camera2d.reference(), W, H);
            plane.set_draw_details(&shader, &vec![tex.id], 1.0, 0.0, 0.1, 0.1, 0.0);
            plane.position_z(9900.0);
            plane.buf[0].unib[[3, 0]] = 0.05;

    reset_params(&mut cube);
    reset_params(&mut plane);

    // fps counter
    let font = pi3d::util::font::create("fonts/NotoSans-Regular.ttf", "", "", 64.0);
    let mut fps_text = pi3d::shapes::point_text::create(camera2d.reference(), &font, 20, 24.0);
            fps_text.set_shader(&textsh);
    let fps_blk = fps_text.add_text_block(&font, &[-W * 0.5 + 20.0, -H * 0.5 + 20.0, 0.1], 19, "00.0 FPS");

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = -0.1;
    let mut df:f32 = 0.01; // for/aft
    let mut ds:f32 = 0.0;  // side/side
    let mut rot: f32 = 0.0;
    let mut tilt: f32 = 0.0;
    let mut fr_num: u32 = 0;
    let mut param_point: usize = 0;

    while display.loop_running() {
        fr_num += 1;
        if fr_num > 2000 { // this changes which parameter to alter
            fr_num = 0;
            param_point += 1;
            if param_point >= (17) {
                param_point = 0;
            }
        }
        cube.draw();
        plane.draw();
        if x.abs() > 300.0 { // draw tiled maps
            //TODO some kind of navigation
        }
        if z.abs() > 300.0 {
            //TODO
        }
        fps_text.set_text(&font, fps_blk, &format!("{:5.1} FPS", display.fps()));
        fps_text.draw();
        
        if display.keys_pressed.contains(&Keycode::Escape) {break;}
        if display.mouse_moved {
            tilt = display.mouse_y as f32 * -0.004;
            rot = display.mouse_x as f32 * -0.004;
        }

        if display.keys_pressed.contains(&Keycode::W) {df = 0.2}
        if display.keys_pressed.contains(&Keycode::S) {df = -0.15;}
        if display.keys_pressed.contains(&Keycode::A) {ds = 0.15;}
        if display.keys_pressed.contains(&Keycode::D) {ds = -0.15;}
        if display.keys_pressed.contains(&Keycode::R) {
            reset_params(&mut cube);
            reset_params(&mut plane);
        }
        let cd = camera.get_direction();
        let dx = cd[0] * df - cd[2] * ds;
        x += dx;
        y += cd[1] * df;
        let dz = cd[2] * df + cd[0] * ds;
        z += dz;
        camera.reset();
        camera.rotate(&[tilt, rot, 0.0]);
        if ds != 0.0 || df != 0.0 {
            camera.position(&[x, y, z]);
            let param_r = param_point / 3;
            let param_c = param_point - (param_r * 3);
            cube.unif[[param_r + UNIF_OFFSET, param_c]] += dx * 0.1;
            plane.unif[[param_r + UNIF_OFFSET, param_c]] += dx * 0.1;
            let o_param_point = (param_point + 8) % 16; // modulus keep in range
            let param_r = o_param_point / 3;
            let param_c = o_param_point - (param_r * 3);
            cube.unif[[param_r + UNIF_OFFSET, param_c]] += dz * 0.1;
            plane.unif[[param_r + UNIF_OFFSET, param_c]] += dz * 0.1;
            //println!("{},{},{}", param_r, param_c, plane.unif[[param_r + UNIF_OFFSET, param_c]])
        }
        //cube.unif[[16, 1]] = fr_num as f32 * 0.001571;
        //plane.unif[[16, 1]] = fr_num as f32 * 0.001571;
        df = 0.0;
        ds = 0.0;

        /* // TODO world wrapping
        if x.abs() >= halfsize {
            x -= x.signum() * mapsize;
        }
        if z.abs() >= halfsize {
            z -= z.signum() * mapsize;
        }*/
    }
}
