//! Cube: only vertex geometry (model in local coordinates `±0.5`).

/// Vertices: interleaving `xyz` + `rgb` + `nxnynz`, 12 triangles (6 faces × 2).
pub fn build_cube_vertex_data() -> Vec<f32> {
    let mut v = Vec::with_capacity(36 * 9);
    let white = [1.0, 1.0, 1.0];
    let mut face = |corners: [[f32; 3]; 4], normal: [f32; 3]| {
        let [bl, br, tr, tl] = corners;
        let c = white;
        let n = normal;
        for p in [bl, br, tr, bl, tr, tl] {
            v.extend_from_slice(&[
                p[0], p[1], p[2], //
                c[0], c[1], c[2], //
                n[0], n[1], n[2],
            ]);
        }
    };

    // +Z
    face(
        [
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ],
        [0.0, 0.0, 1.0],
    );
    // -Z
    face(
        [
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
        ],
        [0.0, 0.0, -1.0],
    );
    // +X
    face(
        [
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
        ],
        [1.0, 0.0, 0.0],
    );
    // -X
    face(
        [
            [-0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
        ],
        [-1.0, 0.0, 0.0],
    );
    // +Y
    face(
        [
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
        ],
        [0.0, 1.0, 0.0],
    );
    // -Y
    face(
        [
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
        ],
        [0.0, -1.0, 0.0],
    );

    debug_assert_eq!(v.len(), 36 * 9);
    v
}
