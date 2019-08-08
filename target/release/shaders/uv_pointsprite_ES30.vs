#version 300 es
precision mediump float;

in vec3 vertex;
in vec3 normal;
in vec2 texcoord;

uniform mat4 modelviewmatrix[2]; // [0] model movement in real coords, [1] in camera coords
uniform vec3 unib[5];
//uniform float ntiles => unib[0][0]
//uniform vec2 umult, vmult => unib[2]
//uniform vec2 u_off, v_off => unib[3]
uniform vec3 unif[20];

out float dist;
out mat2 rotn;
out vec2 corner;
out float subsize;
out vec4 colour;

void main(void) {
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  dist = vertex[2];
  rotn = mat2(cos(normal[0]), sin(normal[0]),
             -sin(normal[0]), cos(normal[0])); 
  gl_PointSize = unib[2][2] * fract(dist);
  corner = texcoord;
  subsize = unif[16][0];
  colour = vec4(normal[1]/1000.0, fract(normal[1]), normal[2]/1000.0, fract(normal[2]) );
}
