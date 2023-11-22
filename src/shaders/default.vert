#version 140

in uint index;
in vec3 position;
in mat4 matrix;

out vec2 out_uv;
flat out uint out_index;

void main() {
    out_uv = position.xy;
    out_index = index;

    gl_Position = vec4(position, 1.0) * matrix;
}