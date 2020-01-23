import rpi3d
import os 
import time

display = rpi3d.Display.create("pyo3 minimal", w=800, h=600, profile="GLES", major=2, minor=0)
shader = rpi3d.Shader("uv_light")
shader_flat = rpi3d.Shader("uv_flat")
shader_blur = rpi3d.Shader("defocus")

keybd = rpi3d.Keyboard(display)
mouse = rpi3d.Mouse(display)
tex = rpi3d.Texture("pattern.png")
tex2 = rpi3d.Texture("mountains3_512.jpg")

camera = rpi3d.Camera(display)
camera2d = rpi3d.Camera(display)
camera2d.set_3d(False)

cube = rpi3d.Tube(camera)
cube.position([-4.0, 7.0, 2.0])
cube.set_draw_details(shader, [tex])

sphere = rpi3d.Sphere(camera)
sphere.position([-3.0, 8.0, 3.0])
sphere.set_draw_details(shader, [tex])

font = rpi3d.Font("NotoSerif-Regular.ttf", "", "", 64)
string = rpi3d.PyString(camera2d, font, "Hello from rust pi3d", 0.0)
string.set_shader(shader_flat)
string.position([100.0, 100.0, 4.0])

terrain = rpi3d.ElevationMap(camera, "mountainsHgt.png", 500.0, 500.0, 20.0, 32, 32, 1.0, "nothing") 
terrain.set_draw_details(shader, [tex2])
terrain.position([0.0, -2.0, 0.0])

defocus = rpi3d.PostProcess(camera2d, display, shader_blur, [], 1.0)

n=0
tm = time.time()
(mx, my) = (0, 0)
(rot, tilt) = (0, 0)
(x, y, z) = (0, 0, 0)
ds = 0.01
while display.loop_running():

    #string.draw()
    #string.rotate_inc_z(0.001)

    defocus.start_capture(True)

    terrain.draw()

    cube.draw()
    cube.rotate_inc_z(0.0017)
    cube.rotate_inc_x(0.0021)
    cube.rotate_inc_y(0.0001)

    sphere.draw()
    sphere.rotate_inc_z(0.001)
    sphere.rotate_inc_x(0.0021)
    sphere.rotate_inc_y(0.007)

    defocus.end_capture()

    n += 1
    k = keybd.read_code()
    if len(k) > 0:
        if k == 'W':
            ds = 0.25
        elif k == 'S':
            ds = -0.07

    (new_mx, new_my) = mouse.position()
    if new_mx != mx or new_my != my or ds != 0.0:
        (mx, my) = (new_mx, new_my)
        tilt = (my - 300.0) * -0.004
        rot = (mx - 400.0) * -0.004
        camera.reset()
        camera.rotate([tilt, rot, 0.0])

    if ds != 0.0:
        cd = camera.get_direction()
        x += cd[0] * ds
        y += cd[1] * ds
        z += cd[2] * ds
        (newy, _mapnorm) = terrain.calc_height(x, z)
        y = newy + 0.0
        camera.position([x, y, z])
    ds = 0.0 ## to trick camera setting to terrail

    defocus.draw([(14, 0, 0.7), (14, 1, 0.1), (14, 2, 0.003)])


print("{:.1f} FPS".format(n / (time.time() - tm)))