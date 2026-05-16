//! Stress scene: many lit spheres on a flat **XZ** grid (draw-call / vertex throughput test).
//!
//! Tweak [`DEMO2_SPHERES`], [`DEMO2_GRID_SPACING`], [`DEMO2_SPHERE_SCALE`] or call [`build_demo2`]
//! with custom parameters from `GlutinApp` / tests.

use std::sync::Arc;

use cgmath::vec3;

use crate::engine::ecs::components::{Camera, Light, LightKind, Material, Position, Scale};
use crate::engine::graphics::{Color, Mesh};
use crate::engine::scene::primitives::build_sphere_vertex_data;
use crate::engine::scene::{
    Scene, spawn_camera_with_look_and_keyboard_orbit, spawn_directional_light, spawn_mesh_entity,
};
use crate::game::components::{CameraKeyboardOrbit, CameraLookTarget};

/// Default sphere count for [`build_demo2_default`].
pub const DEMO2_SPHERES: usize = 5000;
/// World-space distance between neighbor sphere centers along the grid axes.
pub const DEMO2_GRID_SPACING: f32 = 1.05;
/// Uniform [`Scale`] factor for each sphere (model radius is `0.5` before scale).
pub const DEMO2_SPHERE_SCALE: f32 = 0.42;

/// Same as [`build_demo2`] with [`DEMO2_SPHERES`], [`DEMO2_GRID_SPACING`], [`DEMO2_SPHERE_SCALE`].
pub fn build_demo2_default() -> Scene {
    build_demo2(DEMO2_SPHERES, DEMO2_GRID_SPACING, DEMO2_SPHERE_SCALE)
}

/// `sphere_count` spheres on a centered **XZ** grid, each with uniform scale `sphere_scale`.
///
/// # Panics
///
/// If `sphere_count == 0`, `grid_spacing <= 0`, or `sphere_scale <= 0`.
pub fn build_demo2(sphere_count: usize, grid_spacing: f32, sphere_scale: f32) -> Scene {
    assert!(sphere_count > 0, "demo2: sphere_count must be > 0");
    assert!(grid_spacing > 0.0, "demo2: grid_spacing must be > 0");
    assert!(sphere_scale > 0.0, "demo2: sphere_scale must be > 0");

    let mut s = Scene::new();

    // One directional light keeps pixel cost predictable while scaling instance count.
    spawn_directional_light(
        &mut s.world,
        Light::new(
            LightKind::directional_toward_light(vec3(0.35, -1.0, 0.25)),
            Color::from_rgb8(255, 252, 245),
            1.0,
        ),
    );

    let cols = (sphere_count as f32).sqrt().ceil() as usize;
    let cols = cols.max(1);
    let rows = sphere_count.div_ceil(cols);

    let ox = 0.5 * (cols.saturating_sub(1)) as f32;
    let oz = 0.5 * (rows.saturating_sub(1)) as f32;

    let scale = Scale::new(sphere_scale, sphere_scale, sphere_scale);
    let albedo = Color::from_rgb8(190, 200, 215);
    let mat = Material::opaque(albedo);

    let y = 0.5 * sphere_scale;

    let sphere_data = build_sphere_vertex_data(0.5, 18, 36);
    let verts = sphere_data.len() / 9;
    let sphere_mesh = Arc::new(Mesh::new_interleaved_pos3_color3_normal3(&sphere_data, verts));

    for i in 0..sphere_count {
        let ix = i % cols;
        let iz = i / cols;
        let x = (ix as f32 - ox) * grid_spacing;
        let z = (iz as f32 - oz) * grid_spacing;
        spawn_mesh_entity(
            &mut s.world,
            Position::new(x, y, z),
            None,
            Some(scale),
            Some(mat),
            Arc::clone(&sphere_mesh),
        );
    }

    spawn_camera_with_look_and_keyboard_orbit(
        &mut s.world,
        Position::new(0.0, 8.5, 14.0),
        Camera::new(72.0, 0.1, 500.0),
        CameraLookTarget::world(vec3(0.0, 0.0, 0.0)),
        CameraKeyboardOrbit::default(),
    );

    s
}
