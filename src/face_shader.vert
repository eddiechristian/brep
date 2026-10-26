#version 330 core

// Input attributes from our Rust glium struct layout
in vec3 position;
in vec3 normal;

// Output variables passed to the fragment shader
out vec3 frag_normal;
out vec3 frag_pos;

// Uniform matrices sent from our CPU camera tracking loop
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    // Calculate the physical position of the point in 3D space
    frag_pos = vec3(model * vec4(position, 1.0));
    
    // Transform the directional normal vector so it matches rotation
    frag_normal = mat3(transpose(inverse(model))) * normal;
    
    // Project the final position onto the screen space
    gl_Position = projection * view * model * vec4(position, 1.0);
}
