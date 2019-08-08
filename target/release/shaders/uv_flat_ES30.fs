#version 300 es
precision mediump float;

uniform sampler2D tex0;
uniform sampler2D tex1;
uniform sampler2D tex2;
uniform vec3 unib[5];
// see docstring Buffer
uniform vec3 unif[20];
// see docstring Shape

in float dist;
in float fog_start;
in vec2 texcoordout;

out vec4 fragColor;

void main(void) {
//std_main_uv.inc
// ----- boiler-plate code for fragment shaders with uv textures

// NB previous define: tex0, texcoordout, unib, unif, dist

  vec4 texc = texture(tex0, texcoordout); // ------ material or basic colour from texture
  if (texc.a < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  texc.rgb += unib[1] - vec3(0.5);
  float ffact = smoothstep(fog_start, unif[5][0], dist); // ------ smoothly increase fog between 1/3 and full fogdist
  
  fragColor = mix(texc, vec4(unif[4], unif[5][1]), ffact); // ------ combine using factors
  fragColor.a *= unif[5][2];
}
