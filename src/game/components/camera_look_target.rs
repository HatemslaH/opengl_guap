use cgmath::Vector3;
use hecs::Entity;

/// Look target for the camera: put on the same entity as [`Camera`] + [`Position`] + [`Rotation`].
/// System [`crate::ecs::systems::camera_look_at_system`] each frame overwrites [`Rotation`],
/// чтобы смотреть на точку или на другую сущность с [`Position`].
///
/// If the target is [`CameraLookTarget::Entity`] and the entity is not in the world, the angles do not change.
#[derive(Clone, Debug)]
pub enum CameraLookTarget {
    /// Fixed world point (center of the scene, marker, etc.).
    World(Vector3<f32>),
    /// Center [`Position::position`] of another entity (for example a cube).
    Entity(Entity),
}

impl CameraLookTarget {
    pub fn world(point: Vector3<f32>) -> Self {
        Self::World(point)
    }

    pub fn entity(target: Entity) -> Self {
        Self::Entity(target)
    }
}
