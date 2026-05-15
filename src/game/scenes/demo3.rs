use cgmath::Vector3;

use crate::engine::ecs::components::{
    Camera, Light, LightKind, Material, Position, Scale, SurfaceLighting, Velocity,
};
use crate::engine::graphics::Color;
use crate::engine::scene::{
    Scene, spawn_camera, spawn_camera_with_look_and_keyboard_orbit, spawn_capsule,
    spawn_coordinate_grid, spawn_cube, spawn_directional_light,
};
use crate::game::components::{CameraFollow, CameraKeyboardOrbit, CameraLookTarget, Player};

pub fn build_demo3() -> Scene {
    let mut s = Scene::new();
    spawn_coordinate_grid(&mut s.world, 8.0, 1.0);

    spawn_directional_light(
        &mut s.world,
        Light::new(
            LightKind::point_default_attenuation(),
            Color::from_rgb8(255, 195, 150),
            1.25,
        ),
    );

    let albedo = Color::from_rgb8(208, 208, 218);

    let surf_gloss = SurfaceLighting {
        ambient: 0.1,
        diffuse: 0.92,
        specular_color: Color::new(1.0, 1.0, 1.0),
        shininess: 140.0,
    };

    let _ = spawn_cube(
        &mut s.world,
        Position::new(0.0, 0.0, 0.0),
        None,
        Some(Scale::new(10.0, 1.0, 10.0)),
        Some(Material::opaque(albedo).with_surface(surf_gloss)),
    );

    let player = spawn_capsule(
        &mut s.world,
        Position::new(0.0, 1.0, 0.0),
        None,
        Some(Scale::new(1.0, 1.0, 1.0)),
        Some(Material::opaque(Color::from_rgb8(255, 0, 0))),
    );
    s.world
        .insert(player, (Player::new(2.0), Velocity::default()))
        .unwrap_or_else(|e| {
            eprintln!("Error inserting player: {}", e);
            std::process::exit(1);
        });

    let camera = spawn_camera(
        &mut s.world,
        Position::new(0.0, 1.6, 0.0),
        Camera::new(88.0, 0.1, 100.0),
    );
    s.world
        .insert_one(
            camera,
            CameraFollow::new(player, Vector3::new(0.0, 1.6, 0.0)),
        )
        .unwrap_or_else(|e| {
            eprintln!("Error inserting camera follow: {}", e);
            std::process::exit(1);
        });
    s
}
