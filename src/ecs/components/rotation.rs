use cgmath::Vector3;

/// Orientation in **degrees** around the X, Y, Z axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub xyz: Vector3<f32>,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            xyz: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            xyz: Vector3::new(x, y, z),
        }
    }
}
