#version 120

uniform mat4 model_view_proj;

attribute vec3 a_Pos;
attribute vec3 a_Color;

varying vec3 color;

void main() {
    color = a_Color;
    gl_Position = model_view_proj * vec4(a_Pos, 1.0);
}
