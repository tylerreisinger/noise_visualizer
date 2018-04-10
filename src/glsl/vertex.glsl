#version 330

uniform mat4 view_projection = mat4(1.0);

in vec3 position;
in vec4 color;
out vec3 frag_position;
out vec4 frag_color;

void main() {
    gl_Position = view_projection * vec4(position, 1.0);
    frag_position = position;
    frag_color = color;
}
