attribute vec3 vertex;
attribute vec3 normal;
//attribute vec2 texcoord;

uniform mat4 modelviewmatrix; // [0] model movement in real coords, [1] in camera coords, [2] camera at light

varying vec3 color;

void main(void) {
  gl_Position = modelviewmatrix * vec4(vertex, 1.0);
  color = normal;
  //gl_Position = vec4(vertex, 1.0);
}
