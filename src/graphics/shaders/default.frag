#version 330 core

uniform sampler2D u_texture;

in vec2 v_uv;
in vec4 v_color;

out vec4 frag_color;

void main() {
    frag_color = texture(u_texture, v_uv) * v_color;
}
