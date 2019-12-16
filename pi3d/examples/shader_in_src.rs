extern crate pi3d;
extern crate sdl2;
extern crate gl;

use std::ffi::CString;

fn main() {
    // initially set up display, shader, camera, texture and shapes
    let mut display = pi3d::display::create("shader source in code ESC to quit", 800.0, 500.0, "GLES", 3, 0).unwrap();
            display.set_background(&[0.4, 0.5, 0.4, 1.0]);
            display.set_opacity(0.9);
    let v_shader = pi3d::shader::Shader::from_source(&CString::new(
"#version 300 es
precision mediump float;

layout(location = 0) in vec3 vertex; /// called vertex in pi3d

void main()
{
    gl_Position = vec4(vertex, 1.0);
}").unwrap(), gl::VERTEX_SHADER).unwrap();
/*
    // simple setup from github.com/Blakkis/GLSL_Python
    let f_shader = pi3d::shader::Shader::from_source(&CString::new(
"#version 300 es
precision mediump float;
#define fragCoord gl_FragCoord.xy
//uniform vec2  iMouse;
//uniform float iTime; /// unif[19].z
//uniform vec2  iResolution; /// unif[19].xy
uniform vec3 unif[20];
out vec4 fragColor;
void main()
{
    vec2 iResolution = unif[19].xy;
    float iTime = unif[19][2];
    // Set origin to center of the screen
    vec2 uv = fragCoord/iResolution.xy * 2.0 - 1.0;
    // Fix aspect ratio
    uv.x *= iResolution.x / iResolution.y;
    // Time varying pixel color (Copied from ShaderToy default scene)
    vec3 color = 0.5 + 0.5 * cos(iTime + uv.xyx + vec3(0.0, 2.0, 4.0));
    fragColor = vec4(color, 1.0);
    //fragColor = vec4(1.0, 1.0, 0.5, 1.0);
}").unwrap(), gl::FRAGMENT_SHADER).unwrap();
*/
    // raymarch_mod setup from github.com/Blakkis/GLSL_Python
    let f_shader = pi3d::shader::Shader::from_source(&CString::new(
"#version 300 es
precision mediump float;
#define fragCoord gl_FragCoord.xy
//uniform vec2  iMouse;      /// unif[18].xy
//uniform float iTime;       /// unif[19].z
//uniform vec2  iResolution; /// unif[19].xy
uniform vec3 unif[20];
out vec4 fragColor;

float sdSphere(vec3 p, float r) {
  return length(p) - r;
}

float map_the_world(in vec3 pos) {
    float iTime = unif[19].z;
    float displacement = sin(abs(4.0 * cos(iTime)) * pos.x) *
                         sin(abs(4.0 * sin(iTime)) * pos.y) *
                         sin(4.0                   * pos.z) *
                        (0.1 + abs(0.1 * sin(iTime * 2.0)));
    float sphere_0 = sdSphere(pos, 2.5) + displacement;
    return sphere_0;
}

vec3 calculate_normal(in vec3 pos) {
    const vec3 small_step = vec3(0.001, 0.0, 0.0);
    float gradient_x = map_the_world(pos + small_step.xyy) - map_the_world(pos - small_step.xyy);
    float gradient_y = map_the_world(pos + small_step.yxy) - map_the_world(pos - small_step.yxy);
    float gradient_z = map_the_world(pos + small_step.yyx) - map_the_world(pos - small_step.yyx);
    vec3 normal = vec3(gradient_x, gradient_y, gradient_z);
    return normalize(normal);
}

vec3 ray_march(in vec3 ro, in vec3 rd) {
    vec2 iMouse = unif[18].xy;
    float total_distance_traveled = 0.0;
    const int NUMBER_OF_STEPS = 128;
    const float MINIMUM_HIT_DISTANCE = 0.001;
    const float MAXIMUM_TRACE_DISTANCE = 512.0;
    const float AMBIENT = 0.2;
    for (int i = 0; i < NUMBER_OF_STEPS; ++i)
    {
        vec3 current_position = ro + total_distance_traveled * rd;
        float distance_to_closest = map_the_world(current_position);
        if (distance_to_closest < MINIMUM_HIT_DISTANCE) 
        {
            vec3 normal = calculate_normal(current_position);
            vec3 light_position = vec3(-iMouse.x, iMouse.y, 4.0);
            vec3 direction_to_light = normalize(current_position - light_position);
            float diffuse_intensity = max(AMBIENT, pow(dot(normal, direction_to_light), 16.0));
            return vec3(1.0, 0.0, 0.0) * diffuse_intensity;
        }
        if (total_distance_traveled > MAXIMUM_TRACE_DISTANCE){
            break;
        }
        total_distance_traveled += distance_to_closest;
    }
    return vec3(0.0);
}

void main() {
    vec2 iResolution = unif[19].xy;
    vec2 uv = fragCoord / iResolution.xy * 2.0 - 1.0;
    uv.x *= iResolution.x / iResolution.y;
    vec3 camera_position = vec3(0.0, 0.0, -5.0);
    vec3 ray_origin = camera_position;
    vec3 ray_direction = vec3(uv, 1.0);
    vec3 result = ray_march(ray_origin, ray_direction);
    fragColor = vec4(result, 1.0);
}
").unwrap(), gl::FRAGMENT_SHADER).unwrap();

    let shader = pi3d::shader::Program::from_shaders(&[v_shader, f_shader]).unwrap();
    let mut camera = pi3d::camera::create(&display);
            camera.set_3d(false); // make it a 2D shader
    let (w, h) = display.get_size();
    let mut slide = pi3d::shapes::plane::create(camera.reference(), 0.5 * w as f32, 0.5 * h as f32); // fullscreen
            slide.set_draw_details(&shader, &vec![], 1.0, 1.0, 1.0, 1.0, 1.0);
            slide.unif[[19, 0]] = w as f32;
            slide.unif[[19, 1]] = h as f32;
    let mut t: f32 = 0.0;
    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;
    // draw in a loop
    while display.loop_running() { // default sdl2 check for ESC or click cross
        t += 0.0001; // v. approx time
        slide.unif[[19, 2]] = t;
        if display.mouse_moved {
            my = display.mouse_y as f32;
            mx = display.mouse_x as f32;
        }
        slide.unif[[18, 0]] = 2.0 * mx / w - 1.0;
        slide.unif[[18, 1]] = 2.0 * my / h - 1.0;
        slide.draw();
    }
}