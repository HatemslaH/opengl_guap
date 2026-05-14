//! Низкоуровневая графика: шейдеры, меши (VAO/VBO), математика для GPU, тест глубины.
//!
//! Сюда не кладут игровую логику и ECS — только примитивы рендера.

pub mod depth;
pub mod math;
pub mod mesh;
pub mod render_state;
pub mod shader;

pub use depth::enable_depth_test;
pub use math::{
    camera_eye_for_look_at_target, camera_view_matrix, camera_view_projection_matrix,
    camera_yaw_pitch_deg_from_look_direction, matrix3_column_major, matrix4_column_major,
    model_matrix, normal_matrix3_from_model, view_projection_matrix,
};
pub use mesh::{Mesh, MeshTopology};
pub use render_state::{set_opaque_depth_blend, set_transparent_depth_blend};
pub use shader::{MAX_DIRECTIONAL_LIGHTS, MAX_POINT_LIGHTS, ShaderProgram};
