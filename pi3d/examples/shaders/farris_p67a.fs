#version 120
//precision mediump float;

varying vec2 texcoordout;

uniform sampler2D tex0;
uniform vec3 unif[20];

//fragcolor

float TWO_PI = radians(360.0);
float ROOT_3 = sqrt(3.0);
float F3 = 0.5;
float Q_PI = radians(45.0);

vec2 eul(float angle) {
  return vec2(cos(angle), sin(angle));
}

void main(void) {
  /* This shader is the same as farris_p67 apart from the pattern scaling
  z is fixed at 2.0 times the texcoord (which varies 0,0 top left to 1,1
  bottom right.) F now varies the amount of color rotation by multiplying with
  the x component of z. i.e. none at left and F at right.
  
  If the rgb vector is rotated about the origin the values will become
  negative for much of the time (and default is to clamp to range 0-1).
  So the vector from (0.5, 0.5, 0.5) to the rgp point is rotated about an
  orthogonal axis to that (i.e. (0, -b', g').dot(r', g', b') == 0) and the result
  has (0.5, 0.5, 0.5) added back to it. The result will still fall outside
  the 0-1 range occasionally but won't be too bad.
  
  The 3x3 matrix rot is standard for rotating about an axis
  */
  float F =  unif[16][1]; // scales the color rotation
  float F1 = unif[11][0]; // borrow L-n1
  float F2 = unif[11][1]; // borrow L-m1
  vec2 z = texcoordout * 2.0;

  vec2 uv_coord = (eul(TWO_PI * z.y) + 
                   eul(TWO_PI * (ROOT_3 * z.x - z.y) * F1) + 
                   eul(TWO_PI * (-ROOT_3 * z.x - z.y) * F2)) / 6.0 + 0.5;
  gl_FragColor = texture2D(tex0, uv_coord) - vec4(0.5, 0.5, 0.5, 0.0); //keep alpha 1.0
  vec3 ax = normalize(vec3(0.0, gl_FragColor.z, -gl_FragColor.y)); // unit vector along axis
  float sa = sin(F * texcoordout.x * Q_PI);
  float ca = 1.0 - cos(F * texcoordout.x * Q_PI);
  mat3 rot = mat3(1.0 - ca,   ax.z * sa,                      -ax.y * sa,
              -ax.y * sa,       1.0 + ca * (ax.y * ax.y - 1.0), ca  * ax.y * ax.z,
              ax.y * sa,      ca * ax.z * ax.y,               1.0 + ca * (ax.z * ax.z - 1.0));
  
  gl_FragColor.xyz = rot * gl_FragColor.xyz + vec3(0.5);
}

