# rust_pi3d
translation of pi3d from python to rust

Initially I want to break the different parts of the tutorial from
http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
into a module called pi3d so I can do something like::

    extern crate pi3d;

    fn main {
        let mut display = pi3d::display::create(&"experimental window");
        let shader = pi3d::shader::create("shaders/triangle");
        let mut cube = pi3d::cubeoid(0.5, 0.5, 0.5);
        cube.set_shader(&shader);
        while display.loop_running() {
            cube.rotateIncX(0.02);
            //etc
            cube.draw();
        }
    }
