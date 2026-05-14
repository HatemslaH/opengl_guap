//! Создание сущностей в [`super::Scene`] (hecs `World`).

use super::cube::build_cube_vertex_data;
use super::grid::build_grid_vertices;
use crate::ecs::{Camera, RenderMesh, SpinAnimation, Transform};
use crate::graphics::{Mesh, MeshTopology};
use cgmath::Vector3;
use hecs::World;

/// Камера: только [`Transform`] (глаз) и [`Camera`] (yaw/pitch в градусах, FOV, near/far). Без меша — не рисуется.
pub fn spawn_camera(world: &mut World, translation: Vector3<f32>, camera: Camera) {
    world.spawn((Transform { translation }, camera));
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
        Transform::default(),
        RenderMesh {
            mesh,
            topology: MeshTopology::Lines,
        },
        SpinAnimation::disabled(),
    ));
}

/// Куб с центром в `position`. `spin` — анимационный компонент ([`SpinAnimation::disabled()`] если не нужен поворот).
pub fn spawn_cube(world: &mut World, position: Vector3<f32>, spin: SpinAnimation) {
    let data = build_cube_vertex_data();
    let mesh = Mesh::new_interleaved_pos3_color3(&data, 36);
    world.spawn((
        Transform {
            translation: position,
        },
        RenderMesh {
            mesh,
            topology: MeshTopology::Triangles,
        },
        spin,
    ));
}
