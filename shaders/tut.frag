#version 100
precision mediump float;

varying vec2 frag_position;

uniform float effect_time;
uniform vec2 resolution;
uniform bool cool_colours;
uniform vec4 points;
uniform bool has_points;

float effect(vec2 pos_x, float time, vec2 pos_p) {
    return sin(20.0 * length(pos_x - pos_p) - time);
}

vec3 grey(float f) {
    f = clamp (f, 0.0, 1.0);
    return vec3(f);
}

vec3 hsv2rgb(vec3 c)
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

    float u = 0.0;

    vec2 p0 = vec2(points.x, points.y);
    vec2 p1 = vec2(points.z, points.w);
    // vec2 p1 = vec2(points.z, max(0.0, (resolution.y - points.w)));

    p0 = (2.0 * p0 / resolution) - 1.0;
    p1 = (2.0 * p1 / resolution) - 1.0;

    if (length(p0) > 0.1) {
        u += effect(coord_normd, effect_time, p0);
    }

    if (length(p1) > 0.1) {
        u += effect(coord_normd, effect_time, p1);
    }

    if (!has_points) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        if (cool_colours) {
            gl_FragColor = vec4(hue(0.5 + u / 4.0), 1.0);
        } else {
            gl_FragColor = vec4(grey(0.5 + u / 4.0), 1.0);
        }
    }
}
