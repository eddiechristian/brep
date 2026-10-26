pub struct TriangleMesh {
    pub vertices: Vec<Point3D>,
    pub indices: Vec<u32>, // GPU index buffer
}


// more segments if zoomed in.
// 
impl BRepKernel {
    // Generates triangles for a perfectly round cylinder face
    pub fn tessellate_cylinder(&self, radius: f64, height: f64, tolerance: f64) -> TriangleMesh {
        let mut mesh = TriangleMesh { vertices: Vec::new(), indices: Vec::new() };

        // 1. DYNAMIC STEP CALCULATION
        // Calculate exactly how many segments we need so the flat facets 
        // don't deviate from the true radius by more than our tolerance.
        let angle_step = 2.0 * (1.0 - (tolerance / radius)).acos();
        let segments = ( (2.0 * std::f64::consts::PI) / angle_step ).ceil() as usize;
        
        // Ensure a reasonable minimum number of segments
        let segments = segments.max(12); 

        // 2. GENERATE POINTS
        for i in 0..=segments {
            let pct = (i as f64) / (segments as f64);
            let angle = pct * 2.0 * std::f64::consts::PI;
            
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            // Bottom ring point
            mesh.vertices.push(Point3D { x, y, z: 0.0 });
            // Top ring point
            mesh.vertices.push(Point3D { x, y, z: height });
        }

        // 3. BUILD TRIANGLE INDEX PAIRS (Stitching the triangles for the GPU)
        for i in 0..segments {
            let b0 = (i * 2) as u32;       // Bottom current
            let t0 = (i * 2 + 1) as u32;   // Top current
            let b1 = (i * 2 + 2) as u32;   // Bottom next
            let t1 = (i * 2 + 3) as u32;   // Top next

            // Triangle 1 of the wall quad
            mesh.indices.push(b0); mesh.indices.push(t0); mesh.indices.push(b1);
            // Triangle 2 of the wall quad
            mesh.indices.push(b1); mesh.indices.push(t0); mesh.indices.push(t1);
        }

        mesh
    }
}
