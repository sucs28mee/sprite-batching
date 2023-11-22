#version 140

uniform sampler2DArray textures;

in vec2 out_uv;
flat in uint out_index;
out vec4 fragment_color;

void main() {
    fragment_color = texture(textures, vec3(out_uv, float(out_index)));
}