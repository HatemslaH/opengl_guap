//! Creation of entities in [`super::Scene`] (hecs `World`).

use std::sync::Arc;

use crate::engine::{
    ecs::{Camera, Light, Material, Position, RenderMesh, Rotation, Scale},
    graphics::{Mesh, MeshTopology},
    scene::primitives::{
        build_capsule_vertex_data, build_cube_vertex_data, build_cylinder_vertex_data,
        build_grid_vertices, build_sphere_vertex_data,
    },
};
use crate::game::components::{CameraKeyboardOrbit, CameraLookTarget};
use hecs::{Entity, World};

/// Camera: [`Position`] (eye), [`Rotation`] (orientation in degrees, field `xyz`), [`Camera`] (FOV, near/far). Without mesh — not rendered.
pub fn spawn_camera(world: &mut World, position: Position, camera: Camera) -> Entity {
    world.spawn((position, Rotation::default(), camera))
}

/// Camera with automatic rotation on target ([`CameraLookTarget`]) — see [`crate::ecs::systems::camera_look_at_system`].
pub fn spawn_camera_with_look(
    world: &mut World,
    position: Position,
    camera: Camera,
    look: CameraLookTarget,
) -> Entity {
    world.spawn((position, Rotation::default(), camera, look))
}

/// Like [`spawn_camera_with_look`], plus optional orbital control with keyboard ([`CameraKeyboardOrbit`]).
pub fn spawn_camera_with_look_and_keyboard_orbit(
    world: &mut World,
    position: Position,
    camera: Camera,
    look: CameraLookTarget,
    orbit: CameraKeyboardOrbit,
) -> Entity {
    world.spawn((position, Rotation::default(), camera, look, orbit))
}

/// Directional light without [`Position`] — direction is set in [`LightKind::Directional`].
pub fn spawn_directional_light(world: &mut World, light: Light) {
    world.spawn((light,));
}

/// Point light: [`LightKind::Point`] + world position from [`Position`].
pub fn spawn_point_light(world: &mut World, position: Position, light: Light) {
    world.spawn((position, light));
}

pub fn spawn_coordinate_grid(world: &mut World, half_extent: f32, step: f32) {
    assert!(
        half_extent > 0.0 && step > 0.0,
        "half_extent and step must be positive"
    );
    let data = build_grid_vertices(half_extent, step);
    let verts = (data.len() / 6) as usize;
    let mesh = Arc::new(Mesh::new_interleaved_pos3_color3(&data, verts));
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

/// Cube with center in `position`. `spin` — animation; `material` — if [`None`], triangles are not rendered.
///
/// Returns [`Entity`], so that it can be referred to, for example from [`CameraLookTarget::Entity`].
pub fn spawn_cube(
    world: &mut World,
    position: Position,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    material: Option<Material>,
) -> Entity {
    let data = build_cube_vertex_data();
    let mesh = Arc::new(Mesh::new_interleaved_pos3_color3_normal3(&data, 36));
    spawn_mesh_entity(world, position, rotation, scale, material, mesh)
}

/// Sphere (model radius `0.5`, like the cube `±0.5`). See [`build_sphere_vertex_data`].
pub fn spawn_sphere(
    world: &mut World,
    position: Position,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    material: Option<Material>,
) -> Entity {
    let data = build_sphere_vertex_data(0.5, 18, 36);
    let verts = data.len() / 9;
    let mesh = Arc::new(Mesh::new_interleaved_pos3_color3_normal3(&data, verts));
    spawn_mesh_entity(world, position, rotation, scale, material, mesh)
}

/// Cylinder along Y: radius `0.5`, half-height `0.5`, with disks on the ends.
pub fn spawn_cylinder(
    world: &mut World,
    position: Position,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    material: Option<Material>,
) -> Entity {
    let data = build_cylinder_vertex_data(0.5, 0.5, 36, true);
    let verts = data.len() / 9;
    let mesh = Arc::new(Mesh::new_interleaved_pos3_color3_normal3(&data, verts));
    spawn_mesh_entity(world, position, rotation, scale, material, mesh)
}

/// Capsule along Y: radius `0.35`, half-length of the stem `0.3`.
pub fn spawn_capsule(
    world: &mut World,
    position: Position,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    material: Option<Material>,
) -> Entity {
    let data = build_capsule_vertex_data(0.35, 0.3, 8, 28, 28);
    let verts = data.len() / 9;
    let mesh = Arc::new(Mesh::new_interleaved_pos3_color3_normal3(&data, verts));
    spawn_mesh_entity(world, position, rotation, scale, material, mesh)
}

/// Spawns a mesh entity with optional [`Material`] (triangles require material to draw).
pub fn spawn_mesh_entity(
    world: &mut World,
    position: Position,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    material: Option<Material>,
    mesh: Arc<Mesh>,
) -> Entity {
    let rot = rotation.unwrap_or_default();
    let scale = scale.unwrap_or_default();
    let render = RenderMesh {
        mesh,
        topology: MeshTopology::Triangles,
    };
    if let Some(m) = material {
        world.spawn((position, rot, scale, render, m))
    } else {
        world.spawn((position, rot, scale, render))
    }
}
