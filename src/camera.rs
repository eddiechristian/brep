use nalgebra::{Matrix4, Point3, Vector3, Perspective3};

pub struct CADCamera {
    pub target: Point3<f32>,     // The center point the camera orbits around
    pub distance: f32,           // How far zoomed back the camera is
    pub pitch: f32,              // Vertical rotation angle (radians)
    pub yaw: f32,                // Horizontal rotation angle (radians)
    pub aspect_ratio: f32,       // Window width / window height
}

impl CADCamera {
    // 1. Calculate the VIEW Matrix (Camera Position)
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        // Calculate the camera's spatial coordinate position using spherical coordinates
        let eye_x = self.target.x + self.distance * self.pitch.cos() * self.yaw.sin();
        let eye_y = self.target.y + self.distance * self.pitch.sin();
        let eye_z = self.target.z + self.distance * self.pitch.cos() * self.yaw.cos();
        
        let eye = Point3::new(eye_x, eye_y, eye_z);
        let up = Vector3::new(0.0, 1.0, 0.0); // Y-Axis is straight up

        // Generate the standard look-at mapping matrix
        Matrix4::look_at_rh(&eye, &self.target, &up)
    }

    // 2. Calculate the PROJECTION Matrix (Field of View and Clipping Planes)
    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        let fov = 45.0f32.to_radians();
        let znear = 0.1f32;   // Minimum visibility depth threshold
        let zfar = 1000.0f32; // Maximum visibility distance limit

        let perspective = Perspective3::new(self.aspect_ratio, fov, znear, zfar);
        perspective.to_matrix()
    }
}


// Compute the updated frame transformations
let model_matrix = Matrix4::<f32>::identity(); // Object is resting at center origin
let view_matrix = camera.get_view_matrix();
let proj_matrix = camera.get_projection_matrix();

// Convert the nalgebra multi-dimensional structures into raw 4x4 flat arrays
let raw_model: [[f32; 4]; 4] = model_matrix.into();
let raw_view: [[f32; 4]; 4] = view_matrix.into();
let raw_proj: [[f32; 4]; 4] = proj_matrix.into();

// Construct the uniform bundle map for the drawing call
let uniforms = glium::uniform! {
    model: raw_model,
    view: raw_view,
    projection: raw_proj,
    base_color: [0.75f32, 0.75f32, 0.75f32], // Feeds our diffuse fragment shader
    light_dir: [0.2f32, 0.5f32, 0.8f32],
};

// Execute render pass natively on GPU
// target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &draw_params).unwrap();
