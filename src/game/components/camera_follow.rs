use cgmath::Vector3;
use hecs::Entity;

pub struct CameraFollow {
    pub target: Entity,
    pub offset: Vector3<f32>,
}

impl CameraFollow {
    pub fn new(target: Entity, offset: Vector3<f32>) -> Self {
        Self { target, offset }
    }
}
