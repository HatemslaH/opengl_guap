use cgmath::Vector3;

/// Position of an entity in the world.
#[derive(Clone, Debug)]
pub struct Position {
    pub position: Vector3<f32>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::new(x, y, z),
        }
    }
}
