//! Куб как пример меша: 36 вершин (цвет по граням), реализация [`super::Drawable`].

use super::drawable::{DrawContext, Drawable};
use crate::graphics::Mesh;

/// Единичный куб в диапазоне координат примерно `[-0.5, 0.5]` по осям.
pub struct Cube {
    mesh: Mesh,
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

impl Cube {
    pub fn new() -> Self {
        let vertex_data = build_cube_vertex_data();
        let mesh = Mesh::new_interleaved_pos3_color3(&vertex_data, 36);
        Self { mesh }
    }
}

impl Drawable for Cube {
    fn draw(&self, _ctx: &DrawContext<'_>) {
        // Общие uniform'ы и шейдер выставляет владелец сцены; здесь только геометрия.
        self.mesh.draw_triangles();
    }
}

/// Вершины: интерливинг `xyz` + `rgb`, 12 треугольников (6 граней × 2).
fn build_cube_vertex_data() -> Vec<f32> {
    let mut v = Vec::with_capacity(36 * 6);
    let mut face = |corners: [[f32; 3]; 4], color: [f32; 3]| {
        let [bl, br, tr, tl] = corners;
        let c = color;
        for p in [bl, br, tr, bl, tr, tl] {
            v.extend_from_slice(&[p[0], p[1], p[2], c[0], c[1], c[2]]);
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
        [0.2, 0.8, 1.0],
    );
    // -Z
    face(
        [
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
        ],
        [1.0, 0.3, 0.6],
    );
    // +X
    face(
        [
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
        ],
        [0.3, 1.0, 0.4],
    );
    // -X
    face(
        [
            [-0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
        ],
        [1.0, 0.85, 0.2],
    );
    // +Y
    face(
        [
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
        ],
        [0.95, 0.95, 1.0],
    );
    // -Y
    face(
        [
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
        ],
        [0.45, 0.45, 0.55],
    );

    debug_assert_eq!(v.len(), 36 * 6);
    v
}
