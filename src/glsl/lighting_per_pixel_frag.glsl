#version 330

uniform mat4 view;

uniform sampler2D grass_texture;
uniform sampler2D dirt_texture;
uniform sampler2D snow_texture;
uniform sampler2D water_texture;

layout(std140) uniform Lights {
    vec4 light_color;
    vec3 light_pos;
};

struct Material {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float shine;
};

uniform Materials {
    Material mat[5];
};

in Data {
    vec3 position;
    vec3 normal;
    vec3 eye;
    vec3 light_dir;
    vec2 tex_coord;
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
    vec4 water = vec4(0.1, 0.1, 0.6, 1.0);
    vec4 grass = vec4(0.1, 0.6, 0.1, 1.0);
    vec4 dirt = vec4(0.910, 0.435, 0.220, 1.0);
    vec4 stone = vec4(0.5, 0.5, 0.5, 1.0);

    if(value < 0.30) {
        float level = smoothstep(0.15, 0.30, value);
        return water * (1.0 - level) + grass * level;
    } else if(value < 0.60) {
        float level = smoothstep(0.45, 0.55, value);
        return grass * (1.0 - level) + dirt * level;
    } else {
        float level = smoothstep(0.70, 0.90, value);
        return dirt * (1.0 - level) + stone * level; 
    }
}

vec4 texture_gradient(float value) {
    vec4 tex1_color = texture(grass_texture, DataIn.tex_coord * 4.0);
    vec4 tex2_color = texture(dirt_texture, DataIn.tex_coord * 4.0);
    vec4 tex3_color = texture(snow_texture, DataIn.tex_coord * 4.0);
    vec4 tex4_color = texture(water_texture, DataIn.tex_coord * 4.0);

    if(value < 0.30) {
        float level = smoothstep(0.20, 0.30, value);
        return tex4_color * (1.0 - level) + tex1_color * level;
    } else if(value < 0.70) {
        float level = smoothstep(0.50, 0.65, value);
        return tex1_color * (1.0 - level) + tex2_color * level;
    } else if(value >= 0.70) {
        float level = smoothstep(0.70, 0.85, value);
        return tex2_color * (1.0 - level) + tex3_color * level;
    }
}
int texture_mat(float value) {
    if(value < 0.25) {
        return 1;
    }
    return 0;
}

void main() {
    Material frag_mat = mat[texture_mat(1.0 - DataIn.position.z)];
    float intensity = max(dot(DataIn.normal, DataIn.light_dir), 0.0);
    vec4 mat_color = texture_gradient(1.0 - DataIn.position.z);
    vec4 light_color = light_color * intensity;
    vec4 diffuse_color = mat_color * frag_mat.diffuse * light_color;
    vec4 specular_color = vec4(0.0);

    if(intensity > 0.0) {
        vec3 local_light_dir = vec3(view * vec4(DataIn.light_dir, 0.0));
        vec3 h = normalize(local_light_dir + DataIn.eye);

        float specular_intensity = max(dot(h, DataIn.normal), 0.0);
        specular_color = frag_mat.specular * pow(specular_intensity, frag_mat.shine);
    }

    color = apply_srgb(max(diffuse_color + specular_color, mat[0].ambient * mat_color));
}
