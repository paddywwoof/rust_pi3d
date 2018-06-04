# rust_pi3d
translation of pi3d from python to rust

Following parts of the tutorial:
http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
I have started the process of making a rust version of the python pi3d
module.

As at commit 908064 most of the functionality is in place to get demos such
as ForestWalk working.

TODO::

    installation, requirements and compile instructions on here!

    Fonts and lettering.

    error and failure handling. Many functions need to return a Result<..>
    wrapper around whatever they are supposed to do.

    Texture blender option (lower alpha to drop pixel)

    Find out why last value of array_buffer is always set to zero (i.e.
    why a sacrificial extra one needs to be added)

    Mouse buttons

    Offscreen textures, screen capture and post processing

    Lifetimes and controlled deletion of Shaders and Programs.

    More elaborate Camera functions.

    Other Texture types i.e. different internal storage modes supported
    by OpenGL
