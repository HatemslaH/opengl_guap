//! Минимальный ECS на базе [`hecs`]: сущности = набор компонентов, системы = проходы по `World`.

pub mod components;
pub mod systems;

pub use components::{
    Camera, CameraKeyboardOrbit, CameraLookTarget, Color, KeyboardOrbitKeys, KeyboardSceneRootKeys,
    Light, LightKind, Material, Position, RenderMesh, Rotation, Scale, SpinAnimation,
    SurfaceLighting,
};
pub use systems::{
    camera_keyboard_orbit_system, camera_look_at_system, render_mesh_system, spin_animation_system,
};
