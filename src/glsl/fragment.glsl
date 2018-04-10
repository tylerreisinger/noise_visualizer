#version 330

in vec3 frag_position;
in vec4 frag_color;
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

void main() {
    color = apply_srgb(frag_color);
}
