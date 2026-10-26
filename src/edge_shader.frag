#version 330 core

out vec4 color_output;

// Pick a distinct silhouette color (e.g., pure black [0, 0, 0] or selection orange)
uniform vec3 edge_color = vec3(0.1, 0.1, 0.1); 

void main() {
    color_output = vec4(edge_color, 1.0);
}
