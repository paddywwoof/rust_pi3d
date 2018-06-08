attribute vec3 vertex;
attribute vec3 normal;
attribute vec2 texcoord;

uniform mat4 modelviewmatrix[3]; // [0] model movement in real coords, [1] in camera coords, [2] camera at light
uniform vec3 unib[4];
//uniform float ntiles => unib[0][0]
//uniform vec2 umult, vmult => unib[2]
//uniform vec2 u_off, v_off => unib[3]
uniform vec3 unif[20];
//uniform vec3 eye > unif[6]
//uniform vec3 lightpos > unif[8]

varying float dist;
varying float fog_start;

varying vec2 texcoordout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
  vec3 normout;
  vec4 relPosn = modelviewmatrix[0] * vec4(vertex, 1.0);
  
  if (unif[7][0] == 1.0) {                  // this is a point light and unif[8] is location
    lightVector = vec3(relPosn) - unif[8];
    lightFactor = pow(length(lightVector), -2.0);
    lightVector = normalize(lightVector);
    lightVector.z *= -1.0;
  } else {                                  // this is directional light
    lightVector = normalize(unif[8]);
    lightFactor = 1.0;
  }
  lightVector.z *= -1.0;
  // uvec, vvec are tangent and bitangent vectors at the vertex approx
  // lining up with the uv texture mapping. Because (0, 1, 0) is such a
  // common normal direction uvec is generated using just off vertical
  vec3 uvec = normalize(cross(normal, vec3(0.0003, -1.0, 0.0003)));
  vec3 vvec = normalize(cross(uvec, normal));
  normout = normalize(vec3(modelviewmatrix[0] * vec4(normal, 0.0)));   
  uvec = vec3(modelviewmatrix[0] * vec4(uvec, 0.0));
  vvec = vec3(modelviewmatrix[0] * vec4(vvec, 0.0));

  lightVector = vec3(mat4(uvec.x, vvec.x, -normout.x, 0.0,
                          uvec.y, vvec.y, -normout.y, 0.0,
                          uvec.z, vvec.z, -normout.z, 0.0,
                          0.0,    0.0,    0.0,        1.0) * vec4(lightVector, 0.0));

  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
  fog_start = fract(unif[5][0]);
  if (fog_start == 0.0) {
    fog_start = unif[5][0] * 0.333;
  } else {
    fog_start *= unif[5][0];
  }

  texcoordout = texcoord * unib[2].xy + unib[3].xy;

  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist;
}
