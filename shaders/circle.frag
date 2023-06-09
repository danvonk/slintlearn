#version 300 es
precision mediump float;

// in vec2 out_tex;

out vec4 out_color;

void main() {
    //float radius = length(out_tex) - 0.2;
    //float signedDistance = radius - 1.0;

    //vec2 grad = vec2(dFdx(signedDistance), dFdy(signedDistance));
    //float rangeFromLine = abs(signedDistance / length(grad));

    //float lineWeight = clamp(2.0 - rangeFromLine, 0.0, 1.0);

    //out_color = vec4(0.0, 0.1, lineWeight, 1.0);
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
}
