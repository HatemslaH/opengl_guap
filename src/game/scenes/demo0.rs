use crate::engine::ecs::components::{Camera, Material, Position, Scale};
use crate::engine::graphics::Color;
use crate::engine::scene::{Scene, spawn_camera_with_look_and_keyboard_orbit, spawn_capsule};
use crate::game::components::{CameraKeyboardOrbit, CameraLookTarget};

pub fn build_demo0() -> Scene {
    let mut s = Scene::new();

    let cube = spawn_capsule(
        &mut s.world,
        Position::default(),
        None,
        Some(Scale::new(1.0, 1.0, 1.0)),
        Some(Material::opaque(Color::from_rgb8(22, 1, 244))),
    );

    spawn_camera_with_look_and_keyboard_orbit(
        &mut s.world,
        Position::new(-2.2, 2.4, 3.0),
        Camera::new(88.0, 0.1, 100.0),
        CameraLookTarget::Entity(cube),
        CameraKeyboardOrbit::default(),
    );

    s
}
