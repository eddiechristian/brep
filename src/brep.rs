#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// 1. The Vertex Layer
pub struct Vertex {
    pub point: Point3D,
    // Index of one outgoing half-edge leaving this vertex
    pub half_edge: Option<usize>, 
}

// 2. The Half-Edge Layer (The Core Connective Tissue)
pub struct HalfEdge {
    pub origin_vertex: usize,   // Where the edge starts
    pub twin: usize,            // The opposing half-edge index
    pub next: usize,            // The next half-edge in the face loop
    pub face: Option<usize>,    // The face this loop belongs to
}

pub struct Loop {
    pub half_edge: usize, // The starting edge of this specific loop
}

pub struct Face {
    pub math_surface: SurfaceGeometry,
    pub outer_loop: Loop,       // The outer boundary of the face
    pub inner_loops: Vec<Loop>,  // A list of any holes inside the face!
}



pub struct BRepKernel {
    pub vertices: Vec<Vertex>,
    pub half_edges: Vec<HalfEdge>,
    pub faces: Vec<Face>,
}

impl BRepKernel {
    pub fn new() -> Self {
        BRepKernel {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(Vertex {
            point: Point3D { x, y, z },
            half_edge: None,
        });
        idx
    }
}

impl BRepKernel {
    pub fn add_edge(&mut self, v_start: usize, v_end: usize) -> (usize, usize) {
        let he1_idx = self.half_edges.len();
        let he2_idx = he1_idx + 1;

        // Create forward edge
        let he1 = HalfEdge {
            origin_vertex: v_start,
            twin: he2_idx,
            next: he1_idx, // Point to self temporarily
            face: None,
        };

        // Create reverse twin edge
        let he2 = HalfEdge {
            origin_vertex: v_end,
            twin: he1_idx,
            next: he2_idx, // Point to self temporarily
            face: None,
        };

        self.half_edges.push(he1);
        self.half_edges.push(he2);

        // Hook vertices to their outgoing edges if they don't have one
        if self.vertices[v_start].half_edge.is_none() {
            self.vertices[v_start].half_edge = Some(he1_idx);
        }
        if self.vertices[v_end].half_edge.is_none() {
            self.vertices[v_end].half_edge = Some(he2_idx);
        }

        (he1_idx, he2_idx)
    }
}

impl BRepKernel {
    pub fn get_vertices_around_face(&self, face_idx: usize) -> Vec<usize> {
        let mut face_vertices = Vec::new();
        
        // Grab the starting edge of the face
        let start_edge_idx = self.faces[face_idx].half_edge;
        let mut current_edge_idx = start_edge_idx;

        loop {
            let edge = &self.half_edges[current_edge_idx];
            face_vertices.push(edge.origin_vertex);

            // Move to the next edge in the loop
            current_edge_idx = edge.next;

            // Stop once we loop all the way back to the start
            if current_edge_idx == start_edge_idx {
                break;
            }
        }

        face_vertices
    }
}


impl BRepKernel {
    pub fn extrude_face(&mut self, face_idx: usize, height: f64) {
        // 1. Find all the vertices belonging to the bottom face
        let bottom_vertex_indices = self.get_vertices_around_face(face_idx);
        let num_vertices = bottom_vertex_indices.len();
        
        // 2. Create the top vertices directly above the bottom ones
        let mut top_vertex_indices = Vec::new();
        for &v_idx in &bottom_vertex_indices {
            let old_pt = self.vertices[v_idx].point;
            let new_v_idx = self.add_vertex(old_pt.x, old_pt.y, old_pt.z + height);
            top_vertex_indices.push(new_v_idx);
        }

        // 3. Create the vertical side walls (quadrilaterals)
        // We loop through each edge of the base triangle to build a wall up to the top
        for i in 0..num_vertices {
            let next_i = (i + 1) % num_vertices;

            // Get the 4 corners of our new vertical wall face
            let b1 = bottom_vertex_indices[i];      // Bottom Start
            let b2 = bottom_vertex_indices[next_i]; // Bottom End
            let t1 = top_vertex_indices[i];         // Top Start
            let t2 = top_vertex_indices[next_i];    // Top End

            // Stitch a new 4-sided face into the master lists
            self.create_quad_face(b1, b2, t2, t1);
        }

        // 4. Create the Top Cap Face
        // The top face loops in the opposite direction to keep the solid oriented outwards
        top_vertex_indices.reverse();
        self.create_polygon_face(top_vertex_indices);
    }
}

// An enum to hold exact surface math
pub enum SurfaceGeometry {
    Disk {origin: Point3D,normal: Vector3D,radius: f64,},
    Rectangle {origin: Point3D,u: Vector3D, v: Vector3D,},
    Cylinder { center: Point3D, axis: Vector3D, radius: f64 },
    Cone { apex: Point3D, axis: Vector3D, angle: f64 },
    Sphere { center: Point3D, radius: f64 },
    // New Variant
    Triangle { p0: Point3D, p1: Point3D, p2: Point3D },
}
s



