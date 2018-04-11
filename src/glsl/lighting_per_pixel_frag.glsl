#version 330

uniform mat4 view;

layout(std140) uniform Lights {
    vec4 light_color;
    vec3 light_pos;
};

uniform Materials {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float shine;
};

in Data {
    vec3 position;
    vec3 normal;
    vec3 eye;
    vec3 light_dir;
} DataIn;

out vec4 color;

float apply_srgb_component(float component) {
    const float srgb_coeff = 12.92;
    const float exponent = 2.4;
    const float cutoff = 0.04045;
    const float a = 0.055;

    if(component < cutoff) {
        return component/srgb_coeff;
    } else {
        return pow((component + a)/(1.0 + a), exponent);
    }
}

vec4 apply_srgb(vec4 color) {
    return vec4(
        apply_srgb_component(color.r), 
        apply_srgb_component(color.g), 
        apply_srgb_component(color.b), 
        color.a
    );
}

vec4 gradient(float value) {
    if(value < 0.5) {
        return vec4(0.0, DataIn.position.z * 2.0, 1.0 - DataIn.position.z * 2.0, 1.0);
    } else {
        return vec4((DataIn.position.z - 0.5) * 2.0, 1.0 - (DataIn.position.z - 0.5) * 2.0, 0.0, 1.0);
    }
}

void main() {
    float intensity = max(dot(DataIn.normal, DataIn.light_dir), 0.0);
    vec4 mat_color = gradient(DataIn.position.z);
    vec4 light_color = light_color * intensity;
    vec4 diffuse_color = mat_color * diffuse * light_color;
    vec4 specular_color = vec4(0.0);

    if(intensity > 0.0) {
        vec3 local_light_dir = vec3(view * vec4(DataIn.light_dir, 0.0));
        vec3 h = normalize(local_light_dir + DataIn.eye);

        float specular_intensity = max(dot(h, DataIn.normal), 0.0);
        specular_color = specular * pow(specular_intensity, shine);
    }

    color = apply_srgb(max(diffuse_color + specular_color, ambient * mat_color));
}
