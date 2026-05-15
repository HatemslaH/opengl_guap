//! Создание сущностей в [`super::Scene`] (hecs `World`).

use super::capsule::build_capsule_vertex_data;
use super::cube::build_cube_vertex_data;
use super::cylinder::build_cylinder_vertex_data;
use super::grid::build_grid_vertices;
use super::sphere::build_sphere_vertex_data;
use crate::ecs::{
    Camera, CameraKeyboardOrbit, CameraLookTarget, Light, Material, Position, RenderMesh, Rotation,
    Scale, SpinAnimation,
};
use crate::graphics::{Mesh, MeshTopology};
use cgmath::Vector3;
use hecs::{Entity, World};

/// Камера: [`Position`] (глаз), [`Rotation`] (ориентация в градусах, поле `xyz`), [`Camera`] (FOV, near/far). Без меша — не рисуется.
pub fn spawn_camera(world: &mut World, translation: Vector3<f32>, camera: Camera) -> Entity {
    world.spawn((
        Position {
            position: translation,
        },
        Rotation::default(),
        camera,
    ))
}

/// Камера с автоповоротом на цель ([`CameraLookTarget`]) — см. [`crate::ecs::systems::camera_look_at_system`].
pub fn spawn_camera_with_look(
    world: &mut World,
    translation: Vector3<f32>,
    camera: Camera,
    look: CameraLookTarget,
) -> Entity {
    world.spawn((
        Position {
            position: translation,
        },
        Rotation::default(),
        camera,
        look,
    ))
}

/// Как [`spawn_camera_with_look`], плюс опциональное орбитальное управление с клавиатуры ([`CameraKeyboardOrbit`]).
pub fn spawn_camera_with_look_and_keyboard_orbit(
    world: &mut World,
    translation: Vector3<f32>,
    camera: Camera,
    look: CameraLookTarget,
    orbit: CameraKeyboardOrbit,
) -> Entity {
    world.spawn((
        Position {
            position: translation,
        },
        Rotation::default(),
        camera,
        look,
        orbit,
    ))
}

/// Направленный свет без [`Position`] — направление задаётся в [`LightKind::Directional`].
pub fn spawn_directional_light(world: &mut World, light: Light) {
    world.spawn((light,));
}

/// Точечный свет: [`LightKind::Point`] + мировая позиция из [`Position`].
pub fn spawn_point_light(world: &mut World, position: Vector3<f32>, light: Light) {
    world.spawn((Position { position }, light));
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
        Rotation::default(),
        Scale::default(),
        RenderMesh {
            mesh,
            topology: MeshTopology::Lines,
        },
    ));
}

/// Куб с центром в `position`. `spin` — анимация; `material` — если [`None`], треугольники не рисуются.
///
/// Возвращает [`Entity`], чтобы на него можно было сослаться, например из [`CameraLookTarget::Entity`].
pub fn spawn_cube(
    world: &mut World,
    position: Vector3<f32>,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    spin: SpinAnimation,
    material: Option<Material>,
) -> Entity {
    let data = build_cube_vertex_data();
    let mesh = Mesh::new_interleaved_pos3_color3_normal3(&data, 36);
    spawn_mesh_entity(world, position, rotation, scale, spin, material, mesh)
}

/// Сфера (радиус модели `0.5`, как у куба `±0.5`). См. [`build_sphere_vertex_data`].
pub fn spawn_sphere(
    world: &mut World,
    position: Vector3<f32>,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    spin: SpinAnimation,
    material: Option<Material>,
) -> Entity {
    let data = build_sphere_vertex_data(0.5, 18, 36);
    let verts = data.len() / 9;
    let mesh = Mesh::new_interleaved_pos3_color3_normal3(&data, verts);
    spawn_mesh_entity(world, position, rotation, scale, spin, material, mesh)
}

/// Цилиндр вдоль Y: радиус `0.5`, полувысота `0.5`, с дисками на торцах.
pub fn spawn_cylinder(
    world: &mut World,
    position: Vector3<f32>,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    spin: SpinAnimation,
    material: Option<Material>,
) -> Entity {
    let data = build_cylinder_vertex_data(0.5, 0.5, 36, true);
    let verts = data.len() / 9;
    let mesh = Mesh::new_interleaved_pos3_color3_normal3(&data, verts);
    spawn_mesh_entity(world, position, rotation, scale, spin, material, mesh)
}

/// Капсула вдоль Y: радиус `0.35`, половина длины ствола `0.3`.
pub fn spawn_capsule(
    world: &mut World,
    position: Vector3<f32>,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    spin: SpinAnimation,
    material: Option<Material>,
) -> Entity {
    let data = build_capsule_vertex_data(0.35, 0.3, 8, 28, 28);
    let verts = data.len() / 9;
    let mesh = Mesh::new_interleaved_pos3_color3_normal3(&data, verts);
    spawn_mesh_entity(world, position, rotation, scale, spin, material, mesh)
}

fn spawn_mesh_entity(
    world: &mut World,
    position: Vector3<f32>,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    spin: SpinAnimation,
    material: Option<Material>,
    mesh: Mesh,
) -> Entity {
    let pos = Position { position };
    let rot = rotation.unwrap_or_default();
    let scale = scale.unwrap_or_default();
    let render = RenderMesh {
        mesh,
        topology: MeshTopology::Triangles,
    };
    if let Some(m) = material {
        world.spawn((pos, rot, scale, render, spin, m))
    } else {
        world.spawn((pos, rot, scale, render, spin))
    }
}
