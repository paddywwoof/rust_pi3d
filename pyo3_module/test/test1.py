import rpi3d
import os 

display = rpi3d.Display.create()
shader = rpi3d.Shader("uv_flat")
dir_path = os.path.join(os.path.dirname(os.path.realpath(__file__)),
                        "pattern.png")
tex = rpi3d.Texture(dir_path)
camera = rpi3d.Camera(display)
camera.set_3d(False)
plane = rpi3d.Plane(camera, 300.0, 300.0) # NB camera has to be passed to Shape constructor
plane.set_draw_details(shader, [tex])

while display.loop_running():
    plane.draw()