#version 100
precision mediump float;
varying vec2 frag_position;
uniform float effect_time;
uniform float rotation_time;

float roundRectDistance(vec2 pos, vec2 rect_size, float radius)
{
    vec2 q = abs(pos) - rect_size + radius;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radius;
}

void main() {
    vec2 size = vec2(0.4, 0.5) + 0.2 * cos(effect_time / 500. + vec2(0.3, 0.2));
    float radius = 0.5 * sin(effect_time / 300.);
    float a = rotation_time / 800.0;
    float d = roundRectDistance(mat2(cos(a), -sin(a), sin(a), cos(a)) * frag_position, size, radius);
    vec3 col = (d > 0.0) ? vec3(sin(d * 0.2), 0.4 * cos(effect_time / 1000.0 + d * 0.8), sin(d * 1.2)) : vec3(0.2 * cos(d * 0.1), 0.17 * sin(d * 0.4), 0.96 * abs(sin(effect_time / 500. - d * 0.9)));
    col *= 0.8 + 0.5 * sin(50.0 * d);
    col = mix(col, vec3(0.9), 1.0 - smoothstep(0.0, 0.03, abs(d) ));
    gl_FragColor = vec4(col, 1.0);
}
