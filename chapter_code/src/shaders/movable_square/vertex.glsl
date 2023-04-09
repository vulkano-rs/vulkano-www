#version 460

layout(location = 0) in vec2 position;

layout(set = 0, binding = 0) uniform Data {
    vec3 color;
    vec2 position;
} uniforms;

layout(location = 0) out vec3 outColor;

void main() {
    outColor = uniforms.color;
    gl_Position = vec4(
        position.x + uniforms.position.x, 
        position.y + uniforms.position.y, 
        0.0, 
        1.0
    );
}
