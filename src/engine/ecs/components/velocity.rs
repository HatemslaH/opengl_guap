use cgmath::Vector3;

#[derive(Clone, Debug)]
pub struct Velocity {
    pub linear: Vector3<f32>,
    pub angular: Vector3<f32>,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vector3::new(0.0, 0.0, 0.0),
            angular: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Velocity {
    pub fn new(linear: Vector3<f32>, angular: Vector3<f32>) -> Self {
        Self { linear, angular }
    }
}
