extern crate pi3d;
extern crate sdl2;
use sdl2::keyboard::Keycode;

const W:f32 = 800.0;
const H:f32 = 480.0;

fn main() {
    // setup display
    let mut display = pi3d::display::create("experimental window", W, H, "GL", 2, 1).unwrap();
            display.set_background(&[0.1, 0.1, 0.2, 1.0]);
            display.set_mouse_relative(true);
            display.set_target_fps(1000.0);

    // shaders
    let shader = pi3d::shader::Program::from_res(&display, "uv_bump").unwrap();
    let shinesh = pi3d::shader::Program::from_res(&display, "mat_reflect").unwrap();
    let flatsh = pi3d::shader::Program::from_res(&display, "uv_flat").unwrap();
    let textsh = pi3d::shader::Program::from_res(&display, "uv_pointsprite").unwrap();

    // cameras
    let mut camera = pi3d::camera::create(&display);
            camera.set_absolute(false);
    let mut camera2d = pi3d::camera::create(&display);
            camera2d.set_3d(false);

    // textures
    let tree2img = pi3d::texture::create_from_file(&display, "textures/tree2.png");
    let tree1img = pi3d::texture::create_from_file(&display, "textures/tree1.png");
    let hb2img = pi3d::texture::create_from_file(&display, "textures/hornbeam2.png");
    let bumpimg = pi3d::texture::create_from_file(&display, "textures/grasstile_n.jpg");
    let reflimg = pi3d::texture::create_from_file(&display, "textures/stars.jpg");
    //let floorimg = pi3d::texture::create_from_file(&display, "textures/floor_nm.jpg");

    // fog constants
    let (fog_shade, fog_alpha, fog_dist) = ([0.3, 0.3, 0.4], 0.8, 650.0);
    let (t_fog_shade, t_fog_alpha, t_fog_dist) = ([0.2, 0.24, 0.22], 1.0, 150.0);

    // environment cube
    let (mut ecube, _tex_list) = pi3d::shapes::environment_cube::create(&display, 900.0,
                "ecubes/sbox", "jpg");
             ecube.set_shader(&flatsh);

    // elevation map
    let mapsize = 1000.0;
    let halfsize = mapsize * 0.5;
    let mountimg1 = pi3d::texture::create_from_file(&display, "textures/mountains3_512.jpg");
    let mut mymap = pi3d::shapes::elevation_map::new_map(&display, "textures/mountainsHgt.png",
                                mapsize, mapsize, 60.0, 32, 32, 1.0, "nothing");
            mymap.set_draw_details(&shader, &vec![mountimg1.id, bumpimg.id, reflimg.id], 128.0, 0.0, 1.0, 1.0, 2.0);
            mymap.set_fog(&fog_shade, fog_dist, fog_alpha);

    // create trees v.1 2 planes, v.2 3 planes
    let treeplane = pi3d::shapes::plane::create(4.0, 5.0);
    let mut treemodel1 = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::add_shapes(&mut treemodel1,
        vec![&treeplane, &treeplane],
        vec![&[0.0, 2.0, 0.0], &[0.0, 2.0, 0.0]],
        vec![&[0.0, 0.0, 0.0], &[0.0, 1.571, 0.0]],
        vec![&[1.0, 1.0, 1.0], &[1.0, 1.0, 1.0]],
        vec![0, 0]);
    let mut treemodel2 = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::add_shapes(&mut treemodel2,
        vec![&treeplane, &treeplane, &treeplane],
        vec![&[0.0, 2.0, 0.0], &[0.0, 2.0, 0.0], &[0.0, 2.0, 0.0]],
        vec![&[0.0, 0.0, 0.0], &[0.0, 1.047, 0.0], &[0.0, 2.094, 0.0]],
        vec![&[1.0, 1.0, 1.0], &[1.0, 1.0, 1.0], &[1.0, 1.0, 1.0]],
        vec![0, 0, 0]);
    // scatter
    let mut mytrees1 = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::cluster(&mut mytrees1, &treemodel1, &mymap, 0.0, 0.0,
            200.0, 200.0, 3.0, 8.0, 20);
    mytrees1.set_draw_details(&flatsh, &vec![tree2img.id], 0.0, 0.0, 1.0, 1.0, 1.0);
    mytrees1.set_fog(&t_fog_shade, t_fog_dist, t_fog_alpha);

    let mut mytrees2 = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::cluster(&mut mytrees2, &treemodel2, &mymap, 0.0, 0.0,
            200.0, 200.0, 3.0, 6.0, 20);
    mytrees2.set_draw_details(&flatsh, &vec![tree1img.id], 0.0, 0.0, 1.0, 1.0, 1.0);
    mytrees2.set_fog(&t_fog_shade, t_fog_dist, t_fog_alpha);

    let mut mytrees3 = pi3d::shapes::merge_shape::create();
    pi3d::shapes::merge_shape::cluster(&mut mytrees3, &treemodel2, &mymap, 0.0, 0.0,
            300.0, 300.0, 6.0, 10.0, 20);
    mytrees3.set_draw_details(&flatsh, &vec![hb2img.id], 0.0, 0.0, 1.0, 1.0, 1.0);
    mytrees3.set_fog(&t_fog_shade, t_fog_dist, t_fog_alpha);

    // create and position monument
    let (ht, _norm) = pi3d::shapes::elevation_map::calc_height(&mymap, 100.0, 245.0);
    let (mut monument, _tex_list) = pi3d::shapes::model_obj::create(&display, "models/pi3d.obj");
             monument.set_shader(&shinesh);
             monument.set_normal_shine(&vec![bumpimg.id, reflimg.id], 16.0, 0.02, 1.0, 1.0, 0.02, false);
             monument.set_fog(&fog_shade, fog_dist, fog_alpha);
             monument.set_specular(&[0.8, 0.8, 2.0]);
             monument.position(&[100.0, ht + 1.0, 245.0]);
             monument.scale(&[20.0, 20.0, 20.0]);
             monument.rotate_to_y(1.1);

    // fps counter
    let font = pi3d::util::font::create(&display, "fonts/NotoSans-Regular.ttf", "", "", 64.0);
    let mut fps_text = pi3d::shapes::point_text::create(&font, 20, 24.0);
            fps_text.set_shader(&textsh);
    let fps_blk = fps_text.add_text_block(&font, &[-W * 0.5 + 20.0, -H * 0.5 + 20.0, 0.1], 19, "00.0 FPS");

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = -0.1;
    let mut df:f32 = 0.01;
    let mut ds:f32 = 0.0;
    let mut rot: f32;
    let mut tilt: f32;
    let mut mouse_x: f32 = 0.0;
    let mut mouse_y: f32 = 0.0;
    while display.loop_running() {
        monument.draw(&mut camera);
        mymap.draw(&mut camera);
        if x.abs() > 300.0 { // draw tiled maps
            mymap.position(&[mapsize * x.signum(), 0.0, 0.0]);
            mymap.draw(&mut camera);
        }
        if z.abs() > 300.0 {
            mymap.position(&[ 0.0, 0.0, mapsize * z.signum()]);
            mymap.draw(&mut camera);
            if x.abs() > 300.0 {
                mymap.position(&[mapsize * x.signum(), 0.0, mapsize * z.signum()]);
                mymap.draw(&mut camera);
            }
        }
        mymap.position(&[0.0, 0.0, 0.0]);
        ecube.position(&[x, 0.0, z]);
        ecube.draw(&mut camera);
        mytrees1.draw(&mut camera);
        mytrees2.draw(&mut camera);
        mytrees3.draw(&mut camera);
        fps_text.set_text(&font, fps_blk, &format!("{:5.1} FPS", display.fps()));
        fps_text.draw(&mut camera2d);
        
        if display.keys_pressed.contains(&Keycode::Escape) {break;}
        if display.mouse_moved {
            tilt = (mouse_y - display.mouse_y as f32) * 0.002;
            rot = (mouse_x - display.mouse_x as f32) * 0.002;
            mouse_x = display.mouse_x as f32;
            mouse_y = display.mouse_y as f32;
        } else {
            tilt = 0.0;
            rot = 0.0;
        }
        if display.keys_pressed.contains(&Keycode::W) {df = 1.25}
        if display.keys_pressed.contains(&Keycode::S) {df = -0.25;}
        if display.keys_pressed.contains(&Keycode::A) {ds = 0.25;}
        if display.keys_pressed.contains(&Keycode::D) {ds = -0.25;}
        let cd = camera.get_direction();
        x += cd[0] * df - cd[2] * ds; y += cd[1] * df; z += cd[2] * df + cd[0] * ds;
        camera.reset();
        camera.rotate(&[tilt, rot, 0.0]);
        if ds != 0.0 || df != 0.0 {
            let (newy, _mapnorm) = pi3d::shapes::elevation_map::calc_height(&mymap, x, z);
            y = newy + 5.0;
            camera.position(&[x, y, z]);
        }
        df = 0.0;
        ds = 0.0;
        
        if x.abs() >= halfsize { //TODO what causes momentary limbo crossing border?
            x -= x.signum() * mapsize;
        }
        if z.abs() >= halfsize {
            z -= z.signum() * mapsize;
        }
    }
}
