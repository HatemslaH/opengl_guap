//! Низкоуровневая графика: шейдеры, меши (VAO/VBO), математика для GPU, тест глубины.
//!
//! Сюда не кладут игровую логику и ECS — только примитивы рендера.

pub mod depth;
pub mod math;
pub mod mesh;
pub mod shader;

pub use depth::enable_depth_test;
pub use math::{
    camera_view_matrix, camera_view_projection_matrix, camera_yaw_pitch_deg_from_look_direction,
    matrix4_column_major, model_matrix, view_projection_matrix,
};
pub use mesh::{Mesh, MeshTopology};
pub use shader::ShaderProgram;
