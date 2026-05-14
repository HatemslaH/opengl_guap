//! Системы ECS: анимация вращения и отрисовка мешей.

use crate::ecs::components::{Camera, RenderMesh, SpinAnimation, Transform};
use crate::graphics::{
    MeshTopology, ShaderProgram, camera_view_projection_matrix, model_matrix, view_projection_matrix,
};
use hecs::World;

/// Обновляет фазы [`SpinAnimation`] для сущностей, у которых `enabled == true`.
pub fn spin_animation_system(world: &mut World, dt: f32) {
    if dt <= 0.0 {
        return;
    }
    for (_, spin) in world.query_mut::<&mut SpinAnimation>() {
        if spin.enabled {
            spin.phase_y += spin.velocity_y * dt;
            spin.phase_x += spin.velocity_x * dt;
        }
    }
}

/// Рисует все сущности с [`Transform`] + [`RenderMesh`] + [`SpinAnimation`]: сначала линии, затем треугольники.
///
/// Матрица вида×проекция берётся с первой сущности [`Transform`] + [`Camera`]; если камеры нет — используется
/// прежняя фиксированная [`crate::graphics::view_projection_matrix`].
pub fn render_mesh_system(world: &mut World, shader: &ShaderProgram, aspect: f32) {
    let vp = if let Some((_, (transform, cam))) =
        (&mut world.query::<(&Transform, &Camera)>()).into_iter().next()
    {
        camera_view_projection_matrix(
            transform.translation,
            cam.yaw_deg.to_radians(),
            cam.pitch_deg.to_radians(),
            cam.fovy_deg,
            aspect,
            cam.z_near,
            cam.z_far,
        )
    } else {
        view_projection_matrix(aspect)
    };
    shader.use_program();

    for topology in [MeshTopology::Lines, MeshTopology::Triangles] {
        for (_, (transform, render, spin)) in
            &mut world.query::<(&Transform, &RenderMesh, &SpinAnimation)>()
        {
            if render.topology != topology {
                continue;
            }
            let (yaw, pitch) = if spin.enabled {
                (spin.phase_y, spin.phase_x)
            } else {
                (0.0, 0.0)
            };

            let model = model_matrix(transform.translation, yaw, pitch);
            let mvp = vp * model;
            shader.set_mvp(&mvp);
            render.mesh.draw(render.topology);
        }
    }
}
