#version 330 core

in vec3 frag_normal;
in vec3 frag_pos;

out vec4 color_output;

// Simple light direction (simulating a headlamp attached to the camera)
uniform vec3 light_dir = vec3(0.2, 0.5, 0.8);
uniform vec3 base_color = vec3(0.75, 0.75, 0.75); // Standard CAD metallic grey

void main() {
    // Ensure our normal vectors are mathematically unit-length
    vec3 norm = normalize(frag_normal);
    vec3 light = normalize(light_dir);
    
    // Ambient Light (so shadowed sides don't turn completely black)
    float ambient = 0.25;
    
    // Diffuse Component (Lambertian shading)
    float diffuse = max(dot(norm, light), 0.0) * 0.75;
    
    // Combine light attributes and set pixel output
    vec3 final_rgb = base_color * (ambient + diffuse);
    color_output = vec4(final_rgb, 1.0);
}
