//! Минимальный ECS на базе [`hecs`]: сущности = набор компонентов, системы = проходы по `World`.

pub mod components;
pub mod systems;

pub use components::{Camera, RenderMesh, SpinAnimation, Transform};
pub use systems::{render_mesh_system, spin_animation_system};
