//! Координатная сетка: генерация вершин для линий на плоскости XZ и осей.

const AXIS_RED: [f32; 3] = [0.92, 0.25, 0.22];
const AXIS_GREEN: [f32; 3] = [0.28, 0.9, 0.32];
const AXIS_BLUE: [f32; 3] = [0.28, 0.45, 0.95];
const GRID_GRAY: [f32; 3] = [0.38, 0.4, 0.48];

/// Вершины для `GL_LINES`: оси + сетка на плоскости `y = 0`.
pub fn build_grid_vertices(half_extent: f32, step: f32) -> Vec<f32> {
    let mut v = Vec::new();

    push_line(
        &mut v,
        [-half_extent, 0.0, 0.0],
        [half_extent, 0.0, 0.0],
        AXIS_RED,
    );
    push_line(
        &mut v,
        [0.0, -half_extent, 0.0],
        [0.0, half_extent, 0.0],
        AXIS_GREEN,
    );
    push_line(
        &mut v,
        [0.0, 0.0, -half_extent],
        [0.0, 0.0, half_extent],
        AXIS_BLUE,
    );

    let mut z = -half_extent;
    while z <= half_extent + 1e-5 {
        if z.abs() > 1e-4 {
            push_line(
                &mut v,
                [-half_extent, 0.0, z],
                [half_extent, 0.0, z],
                GRID_GRAY,
            );
        }
        z += step;
    }

    let mut x = -half_extent;
    while x <= half_extent + 1e-5 {
        if x.abs() > 1e-4 {
            push_line(
                &mut v,
                [x, 0.0, -half_extent],
                [x, 0.0, half_extent],
                GRID_GRAY,
            );
        }
        x += step;
    }

    v
}

fn push_vertex(buf: &mut Vec<f32>, pos: [f32; 3], color: [f32; 3]) {
    buf.extend_from_slice(&[pos[0], pos[1], pos[2], color[0], color[1], color[2]]);
}

fn push_line(buf: &mut Vec<f32>, a: [f32; 3], b: [f32; 3], color: [f32; 3]) {
    push_vertex(buf, a, color);
    push_vertex(buf, b, color);
}
