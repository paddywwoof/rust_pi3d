import rpi3d
import os 
import time

display = rpi3d.Display.create("pyo3 minimal", 1200, 700, "GLES", 2, 0)
shader = rpi3d.Shader("uv_light")
shader_flat = rpi3d.Shader("uv_flat")
dir_path = os.path.dirname(os.path.realpath(__file__))
tex = rpi3d.Texture(os.path.join(dir_path, "pattern.png"))

camera = rpi3d.Camera(display)
camera2d = rpi3d.Camera(display)
camera2d.set_3d(False)

plane = rpi3d.Plane(camera, 300.0, 300.0) # NB camera has to be passed to Shape constructor
plane.set_draw_details(shader_flat, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
plane.position_z(300.0)

cube = rpi3d.Cuboid(camera, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0)
cube.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
cube.position([-2.0, -2.0, 5.0])

sphere = rpi3d.Sphere(camera, 1.0,  16, 16, 2.0, False)
sphere.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
sphere.position([0.0, 0.0, 4.0])

verts = [[0.0, 2.0],[0.5, 1.9],[0.2, 1.8],[1.0, 0.5],[1.0,0.4],[0.0,0.0]]
lathe = rpi3d.Lathe(camera, verts, 32, 2.1, 8.0)
lathe.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
lathe.position([2.0, -2.0, 4.0])

verts = [-1.0, 2.0, 1.0, -1.2, -0.5, 1.0, -0.2, -0.5, 1.0, 0.0, -1.0, 1.0, 1.5, 0.5, 1.0]
lines = rpi3d.Lines(camera, verts, 5.0, True)
lines.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
lines.position([2.0, 2.0, 5.0])

points = rpi3d.Points(camera, verts, 40.0)
points.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
points.position([-2.0, 2.0, 5.0])

font = rpi3d.Font(os.path.join(dir_path, "NotoSerif-Regular.ttf"), "", "", 64)
string = rpi3d.PyString(camera2d, font, "Hello from rust pi3d", 0.0)
string.set_shader(shader_flat)
string.position([100.0, 100.0, 4.0])

sphere.scale([0.5, 1.5, 0.5])
sphere.position([1.0, 1.0, 1.0])
cube.add_child(sphere)
sphere.position([0.0, 0.0, 4.0])
sphere.scale([0.7, 0.7, 0.7])

n=0
tm = time.time()
while display.loop_running():
    plane.draw()
    plane.rotate_inc_z(0.001)

    cube.draw()
    cube.rotate_child_y(0, 0.01)
    cube.rotate_inc_z(0.001)
    cube.rotate_inc_x(0.0021)
    cube.rotate_inc_y(0.0011)

    sphere.draw()
    sphere.rotate_inc_z(0.001)
    sphere.rotate_inc_x(0.0021)
    sphere.rotate_inc_y(0.007)

    lathe.draw()
    lathe.rotate_inc_z(0.001)
    lathe.rotate_inc_x(0.0021)
    lathe.rotate_inc_y(0.0011)

    lines.draw()
    lines.rotate_inc_z(0.001)
    lines.rotate_inc_x(0.0021)
    lines.rotate_inc_y(0.0011)

    points.draw()
    points.rotate_inc_z(0.001)
    points.rotate_inc_x(0.0021)
    points.rotate_inc_y(0.0011)

    string.draw()
    string.rotate_inc_z(0.001)
    n += 1

print("{:.1f} FPS".format(n / (time.time() - tm)))

