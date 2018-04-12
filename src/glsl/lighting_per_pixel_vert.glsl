#version 330

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat3 normal_model;

in vec3 position;
in vec3 normal;
in vec2 tex_coord;

layout(std140) uniform Lights {
    vec4 light_color;
    vec3 light_pos;
};

out Data {
    vec3 position;
    vec3 normal;
    vec3 eye;
    vec3 light_dir;
    vec2 tex_coord;
} DataOut;

void main() {
    mat4 mv = view * model;
    mat4 mvp = perspective * mv;

    vec3 world_normal = normalize(normal_model * normal);

    gl_Position = mvp * vec4(position, 1.0);
    DataOut.position = position;
    DataOut.normal = world_normal;
    DataOut.eye = normalize(-(mv * vec4(position, 1.0)).xyz);
    DataOut.light_dir = normalize(position - light_pos);
    DataOut.tex_coord = tex_coord;
}
