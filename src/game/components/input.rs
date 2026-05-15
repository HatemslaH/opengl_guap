/// Holding **[** / **]** — rotation of the «root of the scene» around the world axis Y (see [`crate::ecs::systems::render_mesh_system`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyboardSceneRootKeys {
    pub bracket_left: bool,
    pub bracket_right: bool,
}

/// State of the orbit keys (A/D/W/S), which the application updates from keyboard events and passes to [`crate::ecs::systems::camera_keyboard_orbit_system`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyboardOrbitKeys {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}
