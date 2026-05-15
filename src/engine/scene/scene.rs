use hecs::World;

/// Owns [`World`](hecs::World) and all entities of the scene.
pub struct Scene {
    pub world: World,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
