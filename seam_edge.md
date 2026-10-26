# The Seam Edge Trick: Handling Circles in a B-Rep CAD Kernel

In a Boundary Representation (B-Rep) CAD kernel, topological structures rely on **Vertices** (points) connected by **Edges** (lines) to form **Faces** (closed loops). This presents a classic paradox: 

> **If a face is a mathematically perfect circle, it shouldn't have any corners (vertices) or discrete straight edges at all.**

If you attempt to feed a mathematically pure, vertex-less circle into a topological loop algorithm, the system will break because the vertex count is zero. To resolve this, CAD kernels implement a concept known as the **Full-Circle Seam Edge**.

---

## The Core Concept

To represent a circle topologically while preserving perfect mathematical formulas, the kernel drops a single **invisible vertex** anywhere along the circle's circumference.

```
          Perfect Mathematical Circle
                 .-''''-.
               .'        '.
              /            \
             |      X       • <─── Vertex 0 (The Seam)
              \            /
               '.        .'
                 '-....-'
      [ HalfEdge 0 Loops All The Way Around ]
```

By adding this single vertex, the shape conforms to standard topological rules:
1. **Vertex Count:** Exactly `1` vertex (`Vertex 0`).
2. **Half-Edge Count:** Exactly `1` half-edge (`HalfEdge 0`). It starts at `Vertex 0`, wraps 360 degrees around following the exact mathematical equation of the circle, and plugs directly back into `Vertex 0` as its own `.next` edge.

---

## How Extrusion Adapts to a Seam Edge

When you pass a 1-vertex, 1-edge circular face into a standard topological extrusion loop, the algorithm executes flawlessly without needing custom logic for curves:

1. **Identify Base Vertices:** The kernel queries the face and gets a list containing just one element: `[Vertex 0]`.
2. **Duplicate for Top Cap:** The engine creates exactly one corresponding top vertex directly above it: `[Vertex 1]`.
3. **Build the Side Walls:** The side wall loop runs exactly once. 
   * It takes the bottom start (`Vertex 0`), bottom end (`Vertex 0`), top start (`Vertex 1`), and top end (`Vertex 1`).
   * Instead of generating four separate flat walls, it generates a single, self-intersecting cylindrical sheet.

```
               [ Vertex 1 (Top Seam) ]
                   .-------•-------.
                  /        |        \

                 |         |         |  <─── 1 Cylindrical Face
                  \        |        /
                   '-------•-------'
              [ Vertex 0 (Bottom Seam) ]
                           ▲
                           └─ The Seam Edge where the sheet wraps
```

---

## Why This Matters for Performance

By utilizing a Seam Edge combined with an exact geometry layer, your CAD kernel achieves two massive advantages:

* **Infinite Precision:** The `math_surface` property attached to that face reads `SurfaceGeometry::Cylinder { radius: 25.0 }`. When calculating boolean subtractions or intersections, the engine evaluates the precise algebraic formula rather than approximating with flat polygon facets.
* **Low Memory Footprint:** Instead of generating hundreds of segmented rows in your master lists to make a smooth-looking cylinder, the entire 3D shape is represented using just **2 Vertices, 2 Caps, and 1 continuous Side Face**.