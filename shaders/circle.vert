#version 300 es

in vec2 position;
//in vec2 tex;

//out vec2 out_tex;

void main() {
    //out_tex = tex;
    gl_Position = vec4(position, 0.0, 1.0);
}
