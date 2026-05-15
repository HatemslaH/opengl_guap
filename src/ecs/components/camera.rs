/// Camera on the scene: position is in [`Position`], the look direction is in [`Rotation`], here only the projection.
///
/// The look direction at [`Rotation::default`] (`xyz = 0`) coincides with the previous camera «with +Z to the center».
///
/// If [`CameraLookTarget`] is added to the same entity, [`crate::ecs::systems::camera_look_at_system`]
/// each frame overwrites [`Rotation`] under the target.
///
/// If there are several entities with `Camera`, [`crate::ecs::systems::render_mesh_system`] takes the **first**
/// from the request traversal — keep one active camera or explicitly order the spawn.
#[derive(Clone, Debug)]
pub struct Camera {
    /// Vertical field of view in degrees.
    pub fovy_deg: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            fovy_deg: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
}

impl Camera {
    pub fn new(fovy_deg: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            fovy_deg,
            z_near,
            z_far,
        }
    }
}
