# Custom CAD/CAM & B-Rep Kernel Development Guide

Building a custom **CAD/CAM system or B-Rep (Boundary Representation) kernel from scratch** is an engineering journey that bridges advanced data structures, discrete mathematics, and precise physical manufacturing workflows.

---

## 1. Core Architecture of a B-Rep Kernel

A Boundary Representation (B-Rep) model consists of two tightly coupled layers: **Topology** (the structural relationships) and **Geometry** (the exact mathematics).

```
       [ Shell ]
           │
       [ Face ]  ──────► Surface (e.g., NURBS, Plane, Cylinder)
           │
       [ Loop ]
           │
       [ Edge ]  ──────► Curve (e.g., Line, Bezier, Circular Arc)
           │
   [ Vertex/Vertex ] ──► Point (x, y, z)
```

### The Topology Layer (The Graph)
Topology acts as the "glue" that describes how shapes connect. A standard implementation uses the **Half-Edge Data Structure** (or Winged-Edge). Instead of storing a bidirectional edge as a single entity, every edge is split into two directional "half-edges" pointing in opposite directions:
* **`origin_vertex`**: The index of the vertex where the half-edge begins.
* **`twin`**: The index of the opposing half-edge across the same structural boundary.
* **`next`**: The index of the next half-edge in a sequential loop traveling around a face.
* **`face`**: The index of the specific face this loop bounds.

### Index-Based Memory Layout (`usize`)
In modern implementations (especially in languages like Rust), kernels avoid complex pointer webs (like `Rc<RefCell<T>>`) by using flat master arrays. Nodes reference each other using simple array indices (`usize`). This design completely eliminates cyclic ownership memory leaks and allows the CPU to access elements instantaneously via direct array offsets.

---

## 2. The Full-Circle Seam Edge Paradox

A classic paradox in geometric modeling is representing a mathematically perfect circle or cylinder within a graph-based topology layer. Because a true mathematical circle has no discrete corners or flat segments, a naive graph-based loop breaks.

To resolve this seamlessly, professional CAD kernels use an architectural construct called a **Full-Circle Seam Edge**.

```
          Perfect Mathematical Circle
                 .-''''-.
               .'        '.
              /                         |      X       • <─── Vertex 0 (The Seam)
              \            /
               '.        .'
                 '-....-'
      [ HalfEdge 0 Loops All The Way Around ]
```

### The Seam Construct
Instead of approximating a circle with segmented lines, the kernel drops a single **invisible vertex** anywhere on the circumference of the perfect analytical curve:
* **Vertex Count**: Exactly 1 (`Vertex 0`).
* **Half-Edge Count**: Exactly 1 (`HalfEdge 0`). The edge starts at `Vertex 0`, wraps all the way around using the analytical radius formula, and hooks back into `Vertex 0` as its own `next` edge index.

### Symmetrical Extrusion
When a 1-vertex circle face is extruded vertically into a cylinder, the engine processes the operation smoothly without needing customized geometric edge cases:
1. **Base Vertices**: The engine reads the bottom boundary array: `[Vertex 0]`.
2. **Top Vertices**: It generates a single top vertex directly above it: `[Vertex 1]`.
3. **The Wall Loop**: The side wall stitching loop runs exactly once (`num_vertices == 1`), binding the bottom seam and top seam into **one continuous, wrapped side face** matching a flawless `SurfaceGeometry::Cylinder` equation.

---

## 3. Boolean Seam Resolution

Handling a seam edge during Boolean operations (such as a Subtraction or Union) requires a strict pipeline to prevent topological corruption. For instance, if a flat cutting plane slices directly through the invisible seam line of a cylinder, an unprepared engine will double-count the boundary intersection.

```
                 Top View of Cylinder
                      .-''''-.
      Left Cut      .'        '.     Right Cut
      ───────────► /     X      • ◄──────────── (The Seam Edge)
                    \          / 
                     '.      .'
                       '-....-'
```

### The 3-Step Resolution Pipeline

#### Step 1: Geometric Intersection (Math First)
The kernel completely detaches from the topology layer at first. It takes the raw algebraic surface equations:
$$	ext{Plane: } Ax + By + Cz + D = 0$$
$$	ext{Cylinder: } X^2 + Y^2 = R^2$$
It solves this system of equations in pure 3D space to yield the exact coordinates where the surfaces meet, entirely independent of the arbitrary topological seam line.

#### Step 2: Parameter Space Mapping (UV Checking)
Every analytical surface in a B-Rep kernel maps to a 2D coordinate system called **UV Space**. For a cylinder, the $U$ coordinate tracks the angle around the circumference ($0$ to $2\pi$ radians). The **Seam Edge** lives exactly where $U = 0$ (which is spatially identical to $U = 2\pi$).

The intersection algorithm maps the calculated 3D coordinates back into the cylinder's 2D UV coordinates. If an intersection point registers a $U$ coordinate of exactly $0$ or $2\pi$, the kernel flags it as landing directly on the seam.

#### Step 3: Edge Splitting & Structural Re-stitching
Instead of creating a new topological vertex out of nowhere, the engine surgically alters the master index lists:
* It **splits** the vertical seam edge into two pieces at the calculated intersection height.
* It inserts a new vertex right on the seam line.
* It routes the incoming cut curve *through* this new seam vertex.

```
       Before Cut                         After Cut
         [Top]                              [Top]
           │                                  │
           │  (Continuous Seam)               │
           │                                  ▼
           │                           [New Seam Vertex] ◄─── (Plane Edge Hooks In)
           │                                  │
           ▼                                  ▼
       [Bottom]                           [Bottom]
```

---

## 4. The Tessellation Bridge

While the B-Rep kernel preserves infinite mathematical accuracy for downstream processes like CAM toolpath calculations, modern Graphics Processing Units (GPUs) can only parse flat triangles. 

The **Tessellation Engine** bridges this gap using **Adaptive Subdivisions**. It dynamically computes the maximum allowable linear distance (**Chord Error**) and **Angular Deflection** between the analytical curve and the triangle facets to construct an optimal mesh.

When a user zooms out, the engine computes a lower-density grid to save computing cycles. When a user zooms in close to a curved boundary, the engine dynamically regenerates a tight, high-density mesh to ensure visual smoothness, leaving the underlying core mathematical formulas uncompromised.
