#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;

layout (location = 0) out vec3 frag_color;

void main() {
    gl_Position = vec4(position,0.0,1.0);
    frag_color = color;
}