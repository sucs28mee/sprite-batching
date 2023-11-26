#version 140

in uint index;
in vec3 position;
in vec2 uv;
in mat4 matrix;
in vec4 color;

out vec2 out_uv;
out vec4 out_color;
flat out uint out_index;

void main() {
    out_uv = uv;
    out_index = index;
    out_color = color;

    gl_Position = vec4(position, 1.0) * matrix;
}