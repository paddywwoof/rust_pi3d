#version 120
precision mediump float;

varying vec2 texcoordout;

uniform sampler2D tex0;
uniform vec3 unif[20];

//fragcolor

float TWO_PI = radians(360.0);
float ROOT_3 = sqrt(3.0);


vec2 eul(float angle) {
  return vec2(cos(angle), sin(angle));
}

void main(void) {
  float F =  unif[16][1]; // borrow R-rot
  float F1 = unif[11][0]; // borrow L-n1
  float F2 = unif[11][1]; // borrow L-m1
  vec2 z = texcoordout * F;

  vec2 uv_coord = (eul(TWO_PI * z.y) + 
                   eul(TWO_PI * (ROOT_3 * z.x - z.y) * F1) + 
                   eul(TWO_PI * (-ROOT_3 * z.x - z.y) * F2)) / 3.0 + 0.5;
  /* When F1 = F2 = 0.5 this is approximately as per Farris page 67 with
  the modification that rather then everything being divided by 3.0 it is
  divided by 6.0 and has 0.5 added this is to scale the uv coordinates to
  the mapping used by GLSL (0,0) in the top right corner and (1,1) bottom
  left
  */
  gl_FragColor = texture2D(tex0, uv_coord);

}

