use glium::{winit, Surface};
use nalgebra::{Matrix4, Point3, Vector3, Perspective3};

// --- 1. GPU VERTEX STRUCT LAYOUT ---
#[derive(Copy, Clone)]
pub struct CADVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}
glium::implement_vertex!(CADVertex, position, normal);

// --- 2. DYNAMIC CYLINDER TESSELLATION ENGINE ---
// Converts exact mathematical parameters into flat hardware triangle lists
fn tessellate_cylinder(radius: f32, height: f32, tolerance: f32) -> (Vec<CADVertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Calculate how many segments we need so the flat facets stay within chord error tolerance
    let angle_step = 2.0 * (1.0 - (tolerance / radius)).acos();
    let segments = ((2.0 * std::f32::consts::PI) / angle_step).ceil() as usize;
    let segments = segments.max(16); // Clamp to a minimum structural resolution

    // Generate vertex positions and normal attributes
    for i in 0..=segments {
        let pct = (i as f32) / (segments as f32);
        let angle = pct * 2.0 * std::f32::consts::PI;
        
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let x = radius * cos_a;
        let y = radius * sin_a;

        // Side walls share standard radial normals facing straight out
        let normal = [cos_a, sin_a, 0.0];

        // Bottom ring node
        vertices.push(CADVertex { position: [x, y, 0.0], normal });
        // Top ring node
        vertices.push(CADVertex { position: [x, y, height], normal });
    }

    // Stitch the triangle indices together
    for i in 0..segments {
        let b0 = (i * 2) as u32;
        let t0 = (i * 2 + 1) as u32;
        let b1 = (i * 2 + 2) as u32;
        let t1 = (i * 2 + 3) as u32;

        // Triangle 1
        indices.push(b0); indices.push(t0); indices.push(b1);
        // Triangle 2
        indices.push(b1); indices.push(t0); indices.push(t1);
    }

    (vertices, indices)
}

// --- 3. CORE APPLICATION ENTRY POINT ---
fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Performant Desktop CAD Engine")
        .with_inner_size(800, 600)
        .build(&event_loop);

    // Generate cylinder data arrays using a 0.01mm tolerance threshold
    let (v_data, i_data) = tessellate_cylinder(25.0, 80.0, 0.01);
    
    // Upload structural data lists straight to GPU VRAM memory arrays
    let vertex_buffer = glium::VertexBuffer::new(&display, &v_data).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        &display, 
        glium::index::PrimitiveType::TrianglesList, 
        &i_data
    ).unwrap();

    // Define foundational GLSL lighting shaders
    let vertex_shader_src = r#"
        #version 330 core
        in vec3 position;
        in vec3 normal;
        out vec3 frag_normal;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;
        void main() {
            frag_normal = mat3(transpose(inverse(model))) * normal;
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        in vec3 frag_normal;
        out vec4 color_output;
        uniform vec3 light_dir = vec3(0.3, 0.6, 0.7);
        uniform vec3 base_color = vec3(0.65, 0.65, 0.7);
        void main() {
            vec3 norm = normalize(frag_normal);
            float diffuse = max(dot(norm, normalize(light_dir)), 0.0) * 0.7;
            color_output = vec4(base_color * (0.3 + diffuse), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // 4. Set up camera positions utilizing nalgebra structures
    let target = Point3::new(0.0f32, 0.0f32, 40.0f32);
    let eye = Point3::new(120.0f32, 120.0f32, 150.0f32);
    let up = Vector3::new(0.0f32, 0.0f32, 1.0f32);

    let view_matrix = Matrix4::look_at_rh(&eye, &target, &up);
    
    // --- FIXED: Break this into two lines so the temporary value lives long enough ---
    let projection_helper = Perspective3::new(800.0 / 600.0, 45.0f32.to_radians(), 0.1, 1000.0);
    let proj_matrix = projection_helper.as_matrix();    // 4. Set up camera positions utilizing nalgebra structures
    let target = Point3::new(0.0f32, 0.0f32, 40.0f32);
    let eye = Point3::new(120.0f32, 120.0f32, 150.0f32);
    let up = Vector3::new(0.0f32, 0.0f32, 1.0f32);

    let view_matrix = Matrix4::look_at_rh(&eye, &target, &up);
    
    // --- FIXED: Break this into two lines so the temporary value lives long enough ---
    let projection_helper = Perspective3::new(800.0 / 600.0, 45.0f32.to_radians(), 0.1, 1000.0);
    let proj_matrix = projection_helper.as_matrix();

    // 5. Convert them safely into the arrays Glium uniforms expect
    let raw_model: [[f32; 4]; 4] = Matrix4::<f32>::identity().into();
    let raw_view: [[f32; 4]; 4] = *view_matrix.as_ref();
    let raw_proj: [[f32; 4]; 4] = *proj_matrix.as_ref();
();

   
    let uniforms = glium::uniform! {
        model: raw_model,
        view: raw_view,
        projection: raw_proj,
    };

    // Configure hardware depth testing flags
    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Native Window Render Pipeline Loop
    let _ = event_loop.run(move |event, window_target| {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::RedrawRequested => {
                    let mut target_frame = display.draw();
                    target_frame.clear_color_and_depth((0.15, 0.15, 0.18, 1.0), 1.0);
                    
                    // Render cylinder mesh entirely on GPU native hardware silicon
                    target_frame.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &draw_parameters).unwrap();
                    
                    target_frame.finish().unwrap();
                },
                _ => (),
            }
        }
    });
}
