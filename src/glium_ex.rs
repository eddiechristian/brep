#[derive(Copy, Clone)]
pub struct CADVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

// This macro registers the memory alignment and layout offsets for the GPU hardware
glium::implement_vertex!(CADVertex, position, normal);


impl BRepKernel {
    pub fn build_gpu_buffers(&self, display: &glium::Display) -> (glium::VertexBuffer<CADVertex>, glium::IndexBuffer<u32>) {
        let mut vertex_data: Vec<CADVertex> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();

        // [Tessellation Logic Happens Here]
        // You loop through your faces, compute points/normals, and push them to vectors.
        // For example, adding a single point:
        vertex_data.push(CADVertex {
            position: [0.0, 10.0, 0.0],
            normal: [0.0, 0.0, 1.0], // Facing straight up
        });
        index_data.push(0);

        // 3. Upload the Arrays to GPU VRAM
        let vertex_buffer = glium::VertexBuffer::new(display, &vertex_data).unwrap();
        
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList, // For solid faces
            &index_data,
        ).unwrap();

        (vertex_buffer, index_buffer)
    }
}


// --- DRAW PASS 1: The Solid Faces ---
let face_params = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::DepthTest::IfLess, // Standard depth sorting
        write: true,
        ..Default::default()
    },
    ..Default::default()
};
// target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &face_params).unwrap();


// --- DRAW PASS 2: The Wireframe Outlines ---
let edge_params = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::DepthTest::IfLessOrEqual,
        write: true,
        ..Default::default()
    },
    // --- THIS IS Rule 4: Depth Bias Applied via Glium ---
    polygon_offset: glium::PolygonOffset {
        factor: -1.5, // Shifts depth values dynamically based on slope
        units: -2.0,  // Flat uniform shift closer to the camera lens
        fill: false,
        line: true,   // Apply specifically to line primitives
        point: false,
    },
    ..Default::default()
};
// target.draw(&vertex_buffer, &edge_index_buffer, &edge_program, &uniforms, &edge_params).unwrap();


let program = glium::Program::from_source(
    &display, 
    include_str!("face_shader.vert"), 
    include_str!("face_shader.frag"), 
    None
).unwrap();
