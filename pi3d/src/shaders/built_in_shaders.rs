pub const NAMES: [&str; 47] = ["clashtest.fs",
            "clashtest.vs",
            "conway.fs",
            "conway.vs",
            "defocus.fs",
            "defocus.vs",
            "mat_bump.fs",
            "mat_bump.vs",
            "mat_flat.fs",
            "mat_flat.vs",
            "mat_light.fs",
            "mat_light.vs",
            "mat_pointsprite.fs",
            "mat_pointsprite.vs",
            "mat_reflect.fs",
            "mat_reflect.vs",
            "post_base.fs",
            "post_base.vs",
            "shadowcast.fs",
            "shadowcast.vs",
            "shadow_uv_bump.fs",
            "shadow_uv_bump.vs",
            "star.fs",
            "star.vs",
            "std_bump.inc",
            "std_fog_start.inc",
            "std_head_fs.inc",
            "std_head_vs.inc",
            "std_light.inc",
            "std_main_mat.inc",
            "std_main_uv.inc",
            "std_main_vs.inc",
            "std_shine.inc",
            "uv_bump.fs",
            "uv_bump.vs",
            "uv_elev_map.fs",
            "uv_elev_map.vs",
            "uv_flat.fs",
            "uv_flat.vs",
            "uv_light.fs",
            "uv_light.vs",
            "uv_pointsprite.fs",
            "uv_pointsprite.vs",
            "uv_reflect.fs",
            "uv_reflect.vs",
            "uv_toon.fs",
            "uv_toon.vs"];

pub const CODES: [&str; 47] = [
            "//clashtest.fs",
            "//clashtest.vs",
            "//conway.fs",
            "//conway.vs",
            "//defocus.fs",
            "//defocus.vs",
            "//mat_bump.fs
#include std_head_fs.inc

varying vec2 bumpcoordout;
varying vec3 lightVector;
varying float lightFactor;

//fragcolor

void main(void) {
#include std_main_mat.inc
  vec3 bump = normalize(texture2D(tex0, bumpcoordout).rgb * 2.0 - 1.0);
#include std_bump.inc

  gl_FragColor =  mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//mat_bump.vs
#include std_head_vs.inc

varying vec2 bumpcoordout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
  vec3 normout;
#include std_main_vs.inc
  bumpcoordout = (texcoord * unib[2].xy + unib[3].xy) * vec2(1.0, 1.0) * unib[0][0];

  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist;
}",


"            //mat_flat.fs
#include std_head_fs.inc

//fragcolor

void main(void) {
#include std_main_mat.inc
  //if (distance(gl_PointCoord, vec2(0.5)) > 0.5) discard; //circular points
  gl_FragColor = mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}
",


"            //mat_flat.vs
#include std_head_vs.inc

void main(void) {
  vec4 relPosn = modelviewmatrix[0] * vec4(vertex,1.0);
  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = clamp(unib[2][2] / dist, 1.0, unib[2][2]);
}",


            "//mat_light.fs
#include std_head_fs.inc

varying vec3 lightVector;
varying float lightFactor;

//fragcolor

void main(void) {
#include std_main_mat.inc
#include std_light.inc

  gl_FragColor =  mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//mat_light.vs
#include std_head_vs.inc

varying vec3 lightVector;
varying float lightFactor;

void main(void) {
  vec3 normout;
#include std_main_vs.inc

  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc

  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist;
}",


"            //mat_pointsprite.fs
#version 120
//precision mediump float;
uniform vec3 unib[5];
//uniform float hardness => unib[0][0]
//uniform float discard => unib[0][2]

varying vec4 colour;

//fragcolor

void main(void) {
  float alpha = 2.0 * unib[0][0] * (0.5 - length(gl_PointCoord - vec2(0.5)));
  if (alpha < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  gl_FragColor = colour;
  gl_FragColor.a *= alpha;
}
",


"            //mat_pointsprite.vs
#version 120
//precision mediump float;
attribute vec3 vertex;
attribute vec3 normal;
attribute vec2 texcoord;

uniform mat4 modelviewmatrix[2]; // [0] model movement in real coords, [1] in camera coords
uniform vec3 unib[5];
//uniform vec2 umult, vmult => unib[2]
//uniform vec2 u_off, v_off => unib[3]
uniform vec3 unif[20];

varying vec4 colour;

void main(void) {
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2];// * fract(vertex[2]);
  colour = vec4(normal[0], normal[1], normal[2], texcoord[0]);
}
",


            "//mat_reflect.fs
#include std_head_fs.inc

varying vec2 bumpcoordout;
varying vec3 inray;
varying vec3 normout;
varying vec3 lightVector;
varying float lightFactor;

//fragcolor

void main(void) {
#include std_main_mat.inc
  vec3 bump = normalize(texture2D(tex0, bumpcoordout).rgb * 2.0 - 1.0);
#include std_bump.inc
#include std_shine.inc
  shinec = texture2D(tex1, shinecoord); // ------ get the reflection for this pixel
  shinec += vec4(max(pow(dot(refl, -inray), 4.0), 0.0) * unib[4], 1.0); // Phong specular
  float shinefact = clamp(unib[0][1]*length(shinec)/length(texc), 0.0, unib[0][1]);// ------ reduce the reflection where the ground texture is lighter than it

  gl_FragColor = mix(mix(texc, shinec, shinefact), vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//mat_reflect.vs
#include std_head_vs.inc

varying vec2 bumpcoordout;
varying vec3 inray;
varying vec3 normout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
#include std_main_vs.inc
  bumpcoordout = (texcoord * unib[2].xy + unib[3].xy) * vec2(1.0, 1.0) * unib[0][0];

  inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc
  inray = normalize(inray);

  gl_Position = modelviewmatrix[1] * vec4(vertex, 1.0);
  gl_PointSize = unib[2][2] / dist;
}",


            "//post_base.fs
#include std_head_fs.inc

varying vec2 texcoordout;

//fragcolor

void main(void) {
  ///////////////////////////////////////////////////////////////////
  // you can do processing on the texture (or utilise other textures)
  // here. This skeleton just does a fairly approximate convolution
  // with a variable offset distance
  ///////////////////////////////////////////////////////////////////
  vec4 texc = vec4(0.0, 0.0, 0.0, 1.0);

  float dx[9]; // x offsets
  float dy[9]; // y offsets
  float f[9];  // factors. NB the book says that arrays can be initialized
  // float x[3] = float[](1.0, 1.0, 1.0); but I can't get it to work
  dx[0] = -0.00125; dx[1] = 0.0; dx[2] = 0.00125;
  dx[3] = -0.00125; dx[4] = 0.0; dx[5] = 0.00125;
  dx[6] = -0.00125; dx[7] = 0.0; dx[8] = 0.00125;
  
  dy[0] =  0.00125; dy[1] =  0.00125; dy[2] =  0.00125;
  dy[3] =  0.0;     dy[4] =  0.0;     dy[5] =  0.0;
  dy[6] = -0.00125; dy[7] = -0.00125; dy[8] = -0.00125;
  
  f[0] =  0.75; f[1] = -1.0;  f[2] =  0.75;
  f[3] = -1.0;  f[4] =  1.1;  f[5] = -1.0;
  f[6] =  0.75; f[7] = -1.0;  f[8] =  0.75;
  
  vec2 fcoord = vec2(0.0, 0.0);
  
  for (int i=0; i<9; i+=1) {
    fcoord = (texcoordout + vec2(dx[i] * unif[16][0], dy[i] * unif[16][0]));
    texc += (texture2D(tex0, fcoord) * f[i]);
  }
  
  gl_FragColor = texc;
}",


            "//post_base.vs
#include std_head_vs.inc

varying vec2 texcoordout;

void main(void) {
  texcoordout = texcoord * unib[2].xy + unib[3].xy;
  texcoordout.y = 1.0 - texcoordout.y;
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
}",
            "//shadowcast.fs",
            "//shadowcast.vs",
            "//shadow_uv_bump.fs",
            "//shadow_uv_bump.vs",
            "//star.fs",
            "//star.vs",


            "//std_bump.inc
// ----- boiler-plate code for fragment shader to get lighting with additional normal mapping
//       look up normal map value as a vector where each colour goes from -100% to +100% over its range so
//       0xFF7F7F is pointing right and 0X007F7F is pointing left. This vector is then rotated relative to the rotation
//       of the normal at that vertex.

// NB previous define: bump, dist, lightVector, lightFactor, texc, unif, unib

  bump.y *= -1.0;
  // unib[3][2] ([11] in python) is used to adjust the strength of normal map textures
  float bfact = unib[3][2] * (1.0 - smoothstep(100.0, 600.0, dist)); // ------ attenuate smoothly between 100 and 600 units
  float intensity = clamp(dot(lightVector, normalize(vec3(0.0, 0.0, 1.0) + bump * bfact)) * lightFactor, 0.0, 1.0); // ------ adjustment of colour according to combined normal
  texc.rgb = (texc.rgb * unif[9]) * intensity + (texc.rgb * unif[10]); // ------ directional lightcol * intensity + ambient lightcol",



            "//std_fog_start.inc
//NB previousl (in std_head_vs.inc) define fog_start and unif[20]
  fog_start = fract(unif[5][0]);
  if (fog_start == 0.0) {
    fog_start = unif[5][0] * 0.333;
  } else {
    fog_start *= unif[5][0];
  }",

  
            "//std_head_fs.inc
// ----- boiler-plate code for fragment shader variable definition
#version 120
//precision mediump float;

uniform sampler2D tex0;
uniform sampler2D tex1;
uniform sampler2D tex2;
uniform vec3 unib[5];
// see docstring Buffer
uniform vec3 unif[20];
// see docstring Shape

varying float dist;
varying float fog_start;",


            "//std_head_vs.inc
// ----- boiler-plate code for vertex shader variable definition
#version 120
//precision mediump float;

attribute vec3 vertex;
attribute vec3 normal;
attribute vec2 texcoord;

uniform mat4 modelviewmatrix[3]; // [0] model movement in real coords, [1] in camera coords, [2] camera at light
uniform vec3 unib[5];
//uniform float ntiles => unib[0][0]
//uniform vec2 umult, vmult => unib[2]
//uniform vec2 u_off, v_off => unib[3]
uniform vec3 unif[20];
//uniform vec3 eye > unif[6]
//uniform vec3 lightpos > unif[8]

varying float dist;
varying float fog_start;",


            "//std_light.inc
// ----- boiler-plate code for fragment shader to get lighting

// NB previous define: lightVector, lightFactor, texc, unif

  float intensity = clamp(dot(lightVector, vec3(0.0, 0.0, 1.0)) * lightFactor, 0.0, 1.0); // ------ adjustment of colour according to combined normal
  texc.rgb = (texc.rgb * unif[9]) * intensity + (texc.rgb * unif[10]); // ------ directional lightcol * intensity + ambient lightcol",


            "//std_main_mat.inc
// ----- boiler-plate code for fragment shaders with material textures

// NB previous define: unib, unif, dist

  vec4 texc = vec4(unib[1], 1.0); // ------ basic colour from material vector
  if (texc.a < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  float ffact = smoothstep(unif[5][0]/3.0, unif[5][0], dist); // ------ smoothly increase fog between 1/3 and full fogdist",


            "//std_main_uv.inc
// ----- boiler-plate code for fragment shaders with uv textures

// NB previous define: tex0, texcoordout, unib, unif, dist

  vec4 texc = texture2D(tex0, texcoordout); // ------ material or basic colour from texture
  if (texc.a < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  texc.rgb += unib[1] - vec3(0.5);
  float ffact = smoothstep(fog_start, unif[5][0], dist); // ------ smoothly increase fog between 1/3 and full fogdist",


            "//std_main_vs.inc
// ----- boiler-plate code for vertex shader to calculate light direction
//       vector and light strength factor

// NB previous define: modelviewmatrix, vertex, lightVector, unif, lightFactor, normout, normal

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
                          0.0,    0.0,    0.0,        1.0) * vec4(lightVector, 0.0));",


            "//std_shine.inc
// ----- boiler-plate code for fragment shader to get mapping for use
//       with reflected image

// NB previous define: inray, normout, bfact, bump

  vec3 refl = normalize(reflect(inray, normout - 0.1 * bfact * bump)); // ----- reflection direction from this vertex
  vec2 shinecoord = vec2(atan(-refl.x, -refl.z)/ 6.2831854 + 0.5,
                          acos(refl.y) / 3.1415927); // ------ potentially need to clamp with bump included in normal
  vec4 shinec = vec4(0.0, 0.0, 0.0, 0.0);",


            "//uv_bump.fs
#include std_head_fs.inc

varying vec2 texcoordout;
varying vec2 bumpcoordout;
varying vec3 lightVector;
varying float lightFactor;

//fragcolor

void main(void) {
#include std_main_uv.inc
  vec3 bump = normalize(texture2D(tex1, bumpcoordout).rgb * 2.0 - 1.0);
#include std_bump.inc

  gl_FragColor =  mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//uv_bump.vs
#include std_head_vs.inc

varying vec2 texcoordout;
varying vec2 bumpcoordout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
  vec3 normout;
#include std_main_vs.inc
  bumpcoordout = (texcoord * unib[2].xy + unib[3].xy) * vec2(1.0, 1.0) * unib[0][0];

  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc

  texcoordout = texcoord * unib[2].xy + unib[3].xy;

  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist;
}",


            "//uv_elev_map.fs
#include std_head_fs.inc

uniform sampler2D tex3;
uniform sampler2D tex4;
uniform sampler2D tex5;
uniform sampler2D tex6;
uniform sampler2D tex7;

varying vec2 texcoordout;
varying vec2 bumpcoordout;
varying vec3 lightVector;
varying float lightFactor;
varying float texFactor;

//fragcolor

void main(void) {
  vec4 texc = mix(
                  mix(texture2D(tex0, texcoordout), texture2D(tex2, texcoordout), clamp(texFactor, 0.0, 1.0)),
                  mix(texture2D(tex4, texcoordout), texture2D(tex6, texcoordout), clamp((texFactor - 2.0), 0.0, 1.0)),
                  clamp((texFactor - 1.0), 0.0, 1.0));
  texc.rgb += unib[1] - vec3(0.5);
  float ffact = smoothstep(unif[5][0]/3.0, unif[5][0], dist); // ------ smoothly increase fog between 1/3 and full fogdist
  vec3 bump = normalize(mix(
                  mix(texture2D(tex1, bumpcoordout), texture2D(tex3, bumpcoordout), clamp(texFactor, 0.0, 1.0)),
                  mix(texture2D(tex5, bumpcoordout), texture2D(tex7, bumpcoordout), clamp((texFactor - 2.0), 0.0, 1.0)),
                  clamp((texFactor - 1.0), 0.0, 1.0)).rgb * 2.0 - 1.0);
#include std_bump.inc

  gl_FragColor =  mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//uv_elev_map.vs
#include std_head_vs.inc

varying vec2 texcoordout;
varying vec2 bumpcoordout;
varying vec3 lightVector;
varying float lightFactor;
varying float texFactor;

void main(void) {
  vec3 normout;
#include std_main_vs.inc
  texcoordout = fract(texcoord * unib[2].xy + unib[3].xy);
  bumpcoordout = texcoordout * vec2(1.0, 1.0) * unib[0][0];
  texFactor = floor(texcoord[0]); // ----- u and v expected to go up together!

  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc

  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist;
}",


            "//uv_flat.fs
#include std_head_fs.inc

varying vec2 texcoordout;

//fragcolor

void main(void) {
#include std_main_uv.inc
  gl_FragColor = mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//uv_flat.vs
#include std_head_vs.inc

varying vec2 texcoordout;

void main(void) {
  texcoordout = texcoord * unib[2].xy + unib[3].xy;
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  dist = gl_Position.z;
#include std_fog_start.inc
  gl_PointSize = unib[2][2] / dist;
}",


            "//uv_light.fs
#include std_head_fs.inc

varying vec3 normout;
varying vec2 texcoordout;
varying vec3 lightVector;
varying float lightFactor;

//fragcolor

void main(void) {
#include std_main_uv.inc
#include std_light.inc

  gl_FragColor =  mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}
",


            "//uv_light.vs
#include std_head_vs.inc

varying vec2 texcoordout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
  vec3 normout;
#include std_main_vs.inc

  vec3 inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc

  texcoordout = texcoord * unib[2].xy + unib[3].xy;

  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist;
}
",


"            ////uv_pointsprite.fs
#version 120
//precision mediump float;
uniform sampler2D tex0;
uniform vec3 unib[5];

varying float dist;
varying mat2 rotn;
varying vec2 corner;
varying float subsize;
varying float alpha;
varying vec4 colour;

//fragcolor

const vec2 p_centre = vec2(0.5);
const vec2 limit = vec2(0.6);

void main(void) {
  vec2 rot_coord = rotn * (gl_PointCoord - p_centre);
  if (any(greaterThan(abs(rot_coord), limit))) discard;
  rot_coord += p_centre;
  vec4 texc = texture2D(tex0, (rot_coord * subsize + corner));
  if (texc.a < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  gl_FragColor = colour * texc;
  //gl_FragColor.a *= texc.a;
}",


"            //uv_pointsprite.vs
#version 120
//precision mediump float;
attribute vec3 vertex;
attribute vec3 normal;
attribute vec2 texcoord;

uniform mat4 modelviewmatrix[2]; // [0] model movement in real coords, [1] in camera coords
uniform vec3 unib[5];
//uniform float ntiles => unib[0][0]
//uniform vec2 umult, vmult => unib[2]
//uniform vec2 u_off, v_off => unib[3]
uniform vec3 unif[20];

varying float dist;
varying mat2 rotn;
varying vec2 corner;
varying float subsize;
varying vec4 colour;

void main(void) {
  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  dist = vertex[2];
  rotn = mat2(cos(normal[0]), sin(normal[0]),
             -sin(normal[0]), cos(normal[0])); 
  gl_PointSize = unib[2][2] * fract(dist);
  corner = texcoord;
  subsize = unif[16][0];
  colour = vec4(normal[1]/1000.0, fract(normal[1]), normal[2]/1000.0, fract(normal[2]) );
}",


            "//uv_reflect.fs
#include std_head_fs.inc

varying vec2 texcoordout;
varying vec2 bumpcoordout;
varying vec3 inray;
varying vec3 normout;
varying vec3 lightVector;
varying float lightFactor;

//fragcolor

void main(void) {
#include std_main_uv.inc
  vec3 bump = normalize(texture2D(tex1, bumpcoordout).rgb * 2.0 - 1.0);
#include std_bump.inc
#include std_shine.inc
  shinec = texture2D(tex2, shinecoord); // ------ get the reflection for this pixel
  shinec += vec4(max(pow(dot(refl, -inray), 4.0), 0.0) * unib[4], 1.0); // Phong specular
  float shinefact = clamp(unib[0][1]*length(shinec)/length(texc), 0.0, unib[0][1]);// ------ reduce the reflection where the ground texture is lighter than it

  gl_FragColor = mix(mix(texc, shinec, shinefact), vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}",


            "//uv_reflect.vs
#include std_head_vs.inc

varying vec2 texcoordout;
varying vec2 bumpcoordout;
varying vec3 inray;
varying vec3 normout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
#include std_main_vs.inc
  bumpcoordout = (texcoord * unib[2].xy + unib[3].xy) * vec2(1.0, 1.0) * unib[0][0];

  inray = vec3(relPosn - vec4(unif[6], 0.0)); // ----- vector from the camera to this vertex
  dist = length(inray);
#include std_fog_start.inc
  inray = normalize(inray);

  texcoordout = texcoord * unib[2].xy + unib[3].xy;

  gl_Position = modelviewmatrix[1] * vec4(vertex,1.0);
  gl_PointSize = unib[2][2] / dist; // NB this line stops the shader working on windows platforms!
}
",
            "", ""];
