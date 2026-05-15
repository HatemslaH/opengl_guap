//! Scene on ECS ([`hecs::World`]): grid, cubes and other entities as sets of components.
//!
//! Rendering and animation — in [`crate::engine::ecs::systems`], not here.
//!
//! Demo scenes live under [`crate::game::scenes`] (e.g. many spheres in [`crate::game::scenes::demo2`]).
//! Rotation of a group of meshes around the Y axis in demos — keys **[** / **]**
//! (processing in [`GlutinApp`](crate::app::glutin_app::GlutinApp)).

pub mod primitives;
pub mod spawn;
pub mod world;

pub use crate::engine::ecs::{
    Camera, Light, LightKind, Material, Position, RenderMesh, Rotation, Scale, SurfaceLighting,
};
pub use spawn::{
    spawn_camera, spawn_camera_with_look, spawn_camera_with_look_and_keyboard_orbit, spawn_capsule,
    spawn_coordinate_grid, spawn_cube, spawn_cylinder, spawn_directional_light, spawn_point_light,
    spawn_sphere,
};
pub use world::Scene;
