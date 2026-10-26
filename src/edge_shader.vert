#version 330 core

in vec3 position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    // Transform coordinates identically to the faces
    // The hardware Depth Bias (glPolygonOffset) handles pushing these lines forward automatically!
    gl_Position = projection * view * model * vec4(position, 1.0);
}
