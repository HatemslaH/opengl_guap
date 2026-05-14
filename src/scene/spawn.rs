//! Создание сущностей в [`super::Scene`] (hecs `World`).

use super::cube::build_cube_vertex_data;
use super::grid::build_grid_vertices;
use crate::ecs::{Camera, CameraLookTarget, Position, RenderMesh, SpinAnimation};
use crate::graphics::{Mesh, MeshTopology};
use cgmath::Vector3;
use hecs::{Entity, World};

/// Камера: только [`Transform`] (глаз) и [`Camera`] (yaw/pitch в градусах, FOV, near/far). Без меша — не рисуется.
pub fn spawn_camera(world: &mut World, translation: Vector3<f32>, camera: Camera) {
    world.spawn((
        Position {
            position: translation,
        },
        camera,
    ));
}

/// Камера с автоповоротом на цель ([`CameraLookTarget`]) — см. [`crate::ecs::systems::camera_look_at_system`].
pub fn spawn_camera_with_look(
    world: &mut World,
    translation: Vector3<f32>,
    camera: Camera,
    look: CameraLookTarget,
) {
    world.spawn((
        Position {
            position: translation,
        },
        camera,
        look,
    ));
}

pub fn spawn_coordinate_grid(world: &mut World, half_extent: f32, step: f32) {
    assert!(
        half_extent > 0.0 && step > 0.0,
        "half_extent и step должны быть положительными"
    );
    let data = build_grid_vertices(half_extent, step);
    let verts = (data.len() / 6) as usize;
    let mesh = Mesh::new_interleaved_pos3_color3(&data, verts);
    world.spawn((
        Position::default(),
        RenderMesh {
            mesh,
            topology: MeshTopology::Lines,
        },
        SpinAnimation::disabled(),
    ));
}

/// Куб с центром в `position`. `spin` — анимационный компонент ([`SpinAnimation::disabled()`] если не нужен поворот).
///
/// Возвращает [`Entity`], чтобы на него можно было сослаться, например из [`CameraLookTarget::Entity`].
pub fn spawn_cube(world: &mut World, position: Vector3<f32>, spin: SpinAnimation) -> Entity {
    let data = build_cube_vertex_data();
    let mesh = Mesh::new_interleaved_pos3_color3(&data, 36);
    world.spawn((
        Position { position },
        RenderMesh {
            mesh,
            topology: MeshTopology::Triangles,
        },
        spin,
    ))
}
