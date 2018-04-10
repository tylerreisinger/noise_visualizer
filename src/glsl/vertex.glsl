#version 330

uniform mat4 view_projection = mat4(1.0);

in vec3 position;
out vec3 frag_position;
out vec4 frag_color;

vec4 gradient(float value) {
    if(value < 0.5) {
        return vec4(0.0, position.z * 2.0, 1.0 - position.z * 2.0, 1.0);
    } else {
        return vec4((position.z - 0.5) * 2.0, 1.0 - (position.z - 0.5) * 2.0, 0.0, 1.0);
    }
}

void main() {
    gl_Position = view_projection * vec4(position.xy, position.z, 1.0);
    frag_position = position;
    //frag_color = vec4(1.0 - position.z, 0.0, position.z, 1.0);
    frag_color = gradient(position.z);
}
