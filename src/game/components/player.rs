#[derive(Clone, Debug)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 10.0 }
    }
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}
