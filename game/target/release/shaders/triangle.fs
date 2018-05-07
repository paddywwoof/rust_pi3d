uniform sampler2D tex0;
uniform sampler2D tex1;
uniform sampler2D tex2;
uniform vec3 unib[4];
// see docstring Buffer
uniform vec3 unif[20];
// see docstring Shape

varying float dist;
varying float fog_start;
varying vec3 normout;
varying vec2 texcoordout;
varying vec3 lightVector;
varying float lightFactor;

void main(void) {
  vec4 texc = texture2D(tex0, texcoordout); // ------ material or basic colour from texture
  if (texc.a < unib[0][2]) discard; // ------ to allow rendering behind the transparent parts of this object
  texc.rgb += unib[1] - vec3(0.5);
  float ffact = smoothstep(fog_start, unif[5][0], dist); // ------ smoothly increase fog between 1/3 and full fogdist
  float intensity = clamp(dot(lightVector, vec3(0.0, 0.0, 1.0)) * lightFactor, 0.0, 1.0); // ------ adjustment of colour according to combined normal
  texc.rgb = (texc.rgb * unif[9]) * intensity + (texc.rgb * unif[10]); // ------ directional lightcol * intensity + ambient lightcol
  gl_FragColor =  (1.0 - ffact) * texc + ffact * vec4(unif[4], unif[5][1]); // ------ combine using factors
  gl_FragColor.a *= unif[5][2];
}
