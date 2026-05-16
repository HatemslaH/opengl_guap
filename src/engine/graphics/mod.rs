//! Low-level graphics: shaders, meshes (VAO/VBO), math for GPU, depth test.
//!
//! Here we don't put game logic and ECS — only render primitives.

pub mod color;
pub mod depth;
pub mod fps_overlay;
pub mod frame_stats;
pub mod math;
pub mod mesh;
pub mod render_state;
pub mod shader;

pub use color::Color;
pub use depth::enable_depth_test;
pub use fps_overlay::{FpsOverlay, FrameHudMetrics};
pub use frame_stats::SceneRenderStats;
pub use math::{
    camera_eye_for_look_at_target, camera_view_matrix, camera_view_projection_matrix,
    camera_yaw_pitch_deg_from_look_direction, matrix3_column_major, matrix4_column_major,
    model_matrix, normal_matrix3_from_model, view_projection_matrix,
};
pub use mesh::{Mesh, MeshTopology};
pub use render_state::{set_opaque_depth_blend, set_transparent_depth_blend};
pub use shader::{MAX_DIRECTIONAL_LIGHTS, MAX_POINT_LIGHTS, ShaderProgram};
