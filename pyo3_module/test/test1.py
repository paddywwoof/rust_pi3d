import rpi3d
import os 
import time

display = rpi3d.Display.create("pyo3 minimal", 600, 550, "GLES", 2, 0)
shader = rpi3d.Shader("uv_light")
shader_flat = rpi3d.Shader("uv_flat")
shader_mat = rpi3d.Shader("mat_light")

dir_path = os.path.dirname(os.path.realpath(__file__))
keybd = rpi3d.Keyboard(display)
mouse = rpi3d.Mouse(display)
tex = rpi3d.Texture(os.path.join(dir_path, "pattern.png"))
tex2 = rpi3d.Texture(os.path.join(dir_path, "mountains3_512.jpg"))

camera = rpi3d.Camera(display)
camera2d = rpi3d.Camera(display)
camera2d.set_3d(False)

plane = rpi3d.Plane(camera, 300.0, 300.0) # NB camera has to be passed to Shape constructor
plane.set_draw_details(shader_flat, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
plane.position_z(300.0)

cube = rpi3d.Tube(camera, 2.0, 0.5, 2.0, 32, True)
cube.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
cube.position([-2.0, 8.0, 5.0])

sphere = rpi3d.Sphere(camera, 1.0,  16, 16, 0.0, False)
sphere.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
sphere.position([0.0, 10.0, 4.0])

verts = [[0.0, 2.0], [0.5, 1.9], [0.2, 1.8], [1.0, 0.5], [1.0, 0.4], [0.0, 0.0]]
lathe = rpi3d.Lathe(camera, verts, 16, 0.0, 1.0)
lathe.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
lathe.position([2.0, 8.0, 4.0])

verts = [-1.0, 2.0, 1.0, -1.2, -0.5, 1.0, -0.2, -0.5, 1.0, 0.0, -1.0, 1.0, 1.5, 0.5, 1.0]
lines = rpi3d.Lines(camera, verts, 5.0, True)
lines.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
lines.position([2.0, 12.0, 5.0])

points = rpi3d.Points(camera, verts, 40.0)
points.set_draw_details(shader, [tex], 1.0, 0.0, 1.0, 1.0, 0.0)
points.position([-2.0, 12.0, 5.0])

font = rpi3d.Font(os.path.join(dir_path, "NotoSerif-Regular.ttf"), "", "", 64)
string = rpi3d.PyString(camera2d, font, "Hello from rust pi3d", 0.0)
string.set_shader(shader_flat)
string.position([100.0, 100.0, 4.0])

sphere.scale([0.5, 1.5, 0.5])
sphere.position([1.0, 1.0, 1.0])
cube.add_child(sphere)
sphere.position([0.0, 10.0, 4.0])
sphere.scale([0.7, 0.7, 0.7])

terrain = rpi3d.ElevationMap(camera, os.path.join(dir_path, "mountainsHgt.png"),
                         500.0, 500.0, 20.0, 32, 32, 1.0, "nothing") 
terrain.set_draw_details(shader, [tex2], 1.0, 0.0, 1.0, 1.0, 0.0)
terrain.position([0.0, -2.0, 0.0])

tree_tex = rpi3d.Texture(os.path.join(dir_path, "hornbeam2.png"))
treeplane = rpi3d.Plane(camera, 2.0, 2.0)
treemodel = rpi3d.MergeShape(camera)
treemodel.add_shapes([treeplane, treeplane], [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.0, 1.5, 0.0]],
                [[1.0, 2.0, 1.0], [1.0, 2.0, 1.0]], [0, 0])
trees = rpi3d.MergeShape(camera)
trees.cluster(treemodel, terrain, 50.0, 50.0, 100.0, 50.0, 0.5, 7.5, 100)
trees.set_draw_details(shader_flat, [tree_tex], 1.0, 0.0, 1.0, 1.0, 0.0)

model = rpi3d.Model(camera, os.path.join(dir_path, "rust_pi3d.obj"))
model.set_shader(shader_mat)
model.position([-30, 10, 30])
model.scale([20, 20, 20])
model.rotate_to_y(-2.5)

ecube = rpi3d.EnvironmentCube(camera, 900, os.path.join(dir_path, "sbox"), "jpg")
ecube.set_shader(shader_flat)

n=0
tm = time.time()
(mx, my) = (0, 0)
(rot, tilt) = (0, 0)
(x, y, z) = (0, 0, 0)
ds = 0.01
while display.loop_running():
    plane.draw()
    plane.rotate_inc_z(0.001)

    cube.draw()
    cube.rotate_child_y(0, 0.01)
    cube.rotate_inc_z(0.0017)
    cube.rotate_inc_x(0.0021)
    cube.rotate_inc_y(0.0001)

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

    model.draw()

    trees.draw()

    terrain.draw()

    ecube.draw()

    n += 1
    k = keybd.read_code()
    if len(k) > 0:
        if k == 'W':
            ds = 0.25
        elif k == 'S':
            ds = -0.07

    (new_mx, new_my) = (mouse.position())
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

print("{:.1f} FPS".format(n / (time.time() - tm)))