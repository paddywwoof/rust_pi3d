#version 300 es
precision mediump float;

uniform sampler2D tex0;
uniform vec3 unib[5];

in float dist;
in mat2 rotn;
in vec2 corner;
in float subsize;
in float alpha;
in vec4 colour;

out vec4 fragColor;

const vec2 p_centre = vec2(0.5);
const vec2 limit = vec2(0.6);

void main(void) {
  vec2 rot_coord = rotn * (gl_PointCoord - p_centre);
  if (any(greaterThan(abs(rot_coord), limit))) discard;
  rot_coord += p_centre;
  vec4 texc = texture(tex0, (rot_coord * subsize + corner));
  if (texc.a < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  fragColor = colour * texc;
  //fragColor.a *= texc.a;
}
