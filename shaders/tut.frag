#version 100
precision mediump float;

varying vec2 frag_position;
uniform float effect_time;
uniform float rotation_time;

uniform vec2 resolution;

float roundRectDistance(vec2 pos, vec2 rect_size, float radius)
{
    vec2 q = abs(pos) - rect_size + radius;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radius;
}

float effect(vec2 pos_x, float time, vec2 pos_p) {
    return sin(20.0 * length(pos_x - pos_p) - time);
}

//void main() {
//    vec2 size = vec2(0.4, 0.5) + 0.2 * cos(effect_time / 500. + vec2(0.3, 0.2));
//    float radius = 0.5 * sin(effect_time / 300.);
//    float a = rotation_time / 800.0;
//    float d = roundRectDistance(mat2(cos(a), -sin(a), sin(a), cos(a)) * frag_position, size, radius);
//    vec3 col = (d > 0.0) ? vec3(sin(d * 0.2), 0.4 * cos(effect_time / 1000.0 + d * 0.8), sin(d * 1.2)) : vec3(0.2 * cos(d * 0.1), 0.17 * sin(d * 0.4), 0.96 * abs(sin(effect_time / 500. - d * 0.9)));
//    col *= 0.8 + 0.5 * sin(50.0 * d);
//    col = mix(col, vec3(0.9), 1.0 - smoothstep(0.0, 0.03, abs(d) ));
//    gl_FragColor = vec4(col, 1.0);
//}

vec3 grey(float f) {
    f = clamp (f, 0.0, 1.0);
    return vec3(f);
}

vec3 hsv2rgb(vec3 c) //TODO check whether same as in Operators.js
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 hue(float a) {
  return hsv2rgb(vec3(a,1.,1.));
}

void main() {
    vec2 coord_normd = gl_FragCoord.xy / resolution;
    coord_normd *= 2.0;
    coord_normd -= 1.0;

    vec2 p0 = vec2(0.0, 0.0);
    vec2 p1 = vec2(-0.5, -0.5);

    float u = effect(coord_normd, effect_time, p0) + effect(coord_normd, effect_time, p1);
    gl_FragColor = vec4(hue(0.5 + u / 4.0), 1.0);
}
