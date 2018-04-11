#version 330

in Data {
    vec4 color;
    vec3 normal;
} DataIn;

out vec4 out_color;

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

void main() {
    out_color = apply_srgb(DataIn.color);
}
