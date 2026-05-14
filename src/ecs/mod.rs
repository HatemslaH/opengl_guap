//! Минимальный ECS на базе [`hecs`]: сущности = набор компонентов, системы = проходы по `World`.

pub mod components;
pub mod systems;

pub use components::{Camera, CameraLookTarget, Position, RenderMesh, SpinAnimation};
pub use systems::{camera_look_at_system, render_mesh_system, spin_animation_system};
