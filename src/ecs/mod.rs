//! Минимальный ECS на базе [`hecs`]: сущности = набор компонентов, системы = проходы по `World`.

pub mod components;
pub mod systems;

pub use components::Camera;
pub use components::Light;
pub use components::LightKind;
pub use components::Material;
pub use components::Position;
pub use components::RenderMesh;
pub use components::Rotation;
pub use components::Scale;
pub use components::SurfaceLighting;
