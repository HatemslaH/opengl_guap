use cgmath::Vector3;

/// Scale along the local X, Y, Z axes. `0` — zero size along this axis; negative values reflect the geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scale {
    pub xyz: Vector3<f32>,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            xyz: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Scale {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            xyz: Vector3::new(x, y, z),
        }
    }
}
