#version 300 es
precision mediump float;

in vec3 vertex;
in vec3 normal;
in vec2 texcoord;

uniform mat4 modelviewmatrix[3]; // [0] model movement in real coords, [1] in camera coords, [2] camera at light
uniform vec3 unib[5];
//uniform float ntiles => unib[0][0]
//uniform vec2 umult, vmult => unib[2]
//uniform vec2 u_off, v_off => unib[3]
uniform vec3 unif[20];
//uniform vec3 eye > unif[6]
//uniform vec3 lightpos > unif[8]

out float dist;
out float fog_start;
out vec2 texcoordout;

void main(void) {
  texcoordout = texcoord * unib[2].xy + unib[3].xy;
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  dist = gl_Position.z;
//std_fog_start.inc
//NB previousl (in std_head_vs.inc) define fog_start and unif[20]
  fog_start = fract(unif[5][0]);
  if (fog_start == 0.0) {
    fog_start = unif[5][0] * 0.333;
  } else {
    fog_start *= unif[5][0];
  }  gl_PointSize = unib[2][2] / dist;
}
