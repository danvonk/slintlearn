#version 100

attribute vec2 position;
varying vec2 frag_position;

void main() {
    frag_position = position;
    gl_Position = vec4(position, 0.0, 1.0);
}
