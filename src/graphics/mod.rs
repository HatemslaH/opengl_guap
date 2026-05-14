//! Низкоуровневая графика: шейдеры, меши (VAO/VBO), математика для GPU, тест глубины.
//!
//! Сюда не кладут «игровые» объекты (куб, персонаж) — только переиспользуемые примитивы рендера.
//! Новые фигуры добавляются в модуль [`crate::scene`], а не сюда.

pub mod depth;
pub mod math;
pub mod mesh;
pub mod shader;

pub use depth::enable_depth_test;
pub use math::{matrix4_column_major, mvp_matrix};
pub use mesh::Mesh;
pub use shader::ShaderProgram;
