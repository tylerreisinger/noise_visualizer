#version 330

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat3 normal_model;

in vec3 position;
in vec3 normal;

out Data {
    vec3 position;
    vec3 normal;
    vec3 eye;
} DataOut;

void main() {
    mat4 mv = view * model;
    mat4 mvp = perspective * mv;

    vec3 world_normal = normalize(normal_model * normal);

    gl_Position = mvp * vec4(position, 1.0);
    DataOut.position = position;
    DataOut.normal = world_normal;
    DataOut.eye = -(mv * vec4(position, 1.0)).xyz;
}
