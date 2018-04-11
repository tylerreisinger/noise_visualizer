#version 330

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat3 normal_model;

in vec3 position;
in vec3 normal;

uniform Lights {
    vec3 dir;
};

uniform Materials {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float shine;
};

out Data {
    vec4 color;
    vec3 normal;
} DataOut;

vec4 gradient(float value) {
    if(value < 0.5) {
        return vec4(0.0, position.z * 2.0, 1.0 - position.z * 2.0, 1.0);
    } else {
        return vec4((position.z - 0.5) * 2.0, 1.0 - (position.z - 0.5) * 2.0, 0.0, 1.0);
    }
}



void main() {
    mat4 mv = view * model;
    mat4 mvp = perspective * mv;
    gl_Position = mvp * vec4(position, 1.0);

    vec4 mat_color = vec4(1.0);
    //vec4 mat_color = gradient(position.z);
    vec3 model_normal = normalize(normal_model * normal);
    float intensity = max(dot(model_normal, dir), 0.0);
    vec4 specular_color = vec4(0.0);

    if(intensity > 0.0) {
        vec3 local_pos = vec3(mv * vec4(position, 1.0));
        vec3 local_norm = vec3(view * vec4(normal_model * normal, 0.0));
        vec3 local_light_dir = vec3(view * vec4(dir, 0.0));

        vec3 eye = normalize(-local_pos);
        vec3 h = normalize(local_light_dir + eye);

        float specular_intensity = max(dot(h,local_norm), 0.0);
        specular_color = specular * pow(specular_intensity, shine);
    }

    vec4 diffuse_color = mat_color * vec4(intensity);
    vec4 ambient_color = mat_color * ambient;

    vec4 linear_color = max(ambient, min(diffuse_color + specular_color, 1.0));

    

    DataOut.color = linear_color;
    DataOut.normal = normal;
}
