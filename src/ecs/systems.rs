//! Системы ECS: анимация вращения и отрисовка мешей.

use crate::ecs::components::{
    Camera, CameraLookTarget, Material, Position, RenderMesh, SpinAnimation,
};
use crate::graphics::{
    MeshTopology, ShaderProgram, camera_view_projection_matrix,
    camera_yaw_pitch_deg_from_look_direction, model_matrix, set_opaque_depth_blend,
    set_transparent_depth_blend, view_projection_matrix,
};
use cgmath::Vector3;
use hecs::{Entity, World};

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

/// Для сущностей с [`Transform`] + [`Camera`] + [`CameraLookTarget`] выставляет углы [`Camera`],
/// чтобы взгляд был на мировую точку или на [`Transform`] указанной сущности.
pub fn camera_look_at_system(world: &mut World) {
    let items: Vec<(hecs::Entity, Vector3<f32>, CameraLookTarget)> =
        (&mut world.query::<(&Position, &Camera, &CameraLookTarget)>())
            .into_iter()
            .map(|(e, (t, _, look))| (e, t.position, look.clone()))
            .collect();

    for (camera_e, eye, look) in items {
        let target = match &look {
            CameraLookTarget::World(p) => *p,
            CameraLookTarget::Entity(target_e) => match world.get::<&Position>(*target_e) {
                Ok(t) => t.position,
                Err(_) => continue,
            },
        };
        let dir = target - eye;
        let Some((yaw, pitch)) = camera_yaw_pitch_deg_from_look_direction(dir) else {
            continue;
        };
        if let Ok(mut cam) = world.get::<&mut Camera>(camera_e) {
            cam.yaw_deg = yaw;
            cam.pitch_deg = pitch;
        }
    }
}

/// Рисует сущности с [`Position`] + [`RenderMesh`] + [`SpinAnimation`].
///
/// - **Линии** — только цвет вершины (сетка), без материала.
/// - **Треугольники** — только если есть [`Material`]; без материала меш не рисуется.
/// - Непрозрачные треугольники (`opacity ≈ 1`) — без смешивания и с записью в Z-буфер;
///   полупрозрачные — отдельный проход с `GL_BLEND` и без записи в глубину.
///
/// Матрица вида×проекция — с первой сущности [`Position`] + [`Camera`]; если камеры нет — [`view_projection_matrix`].
///
/// Полупрозрачные треугольники сортируются **от дальнего к ближнему** к камере (painter's algorithm): иначе при
/// выключенной записи в Z-буфер задний объект ошибочно рисуется поверх переднего.
pub fn render_mesh_system(world: &mut World, shader: &ShaderProgram, aspect: f32) {
    let (vp, camera_eye) = if let Some((_, (pos, cam))) = (&mut world
        .query::<(&Position, &Camera)>())
        .into_iter()
        .next()
    {
        (
            camera_view_projection_matrix(
                pos.position,
                cam.yaw_deg.to_radians(),
                cam.pitch_deg.to_radians(),
                cam.fovy_deg,
                aspect,
                cam.z_near,
                cam.z_far,
            ),
            pos.position,
        )
    } else {
        (view_projection_matrix(aspect), Vector3::new(0.0, 0.0, 2.8))
    };

    shader.use_program();
    set_opaque_depth_blend();

    // --- Линии: вершинный цвет, без материала ---
    shader.set_vertex_color_mode(true);
    for (_, (transform, render, spin)) in
        &mut world.query::<(&Position, &RenderMesh, &SpinAnimation)>()
    {
        if render.topology != MeshTopology::Lines {
            continue;
        }
        let (yaw, pitch) = if spin.enabled {
            (spin.phase_y, spin.phase_x)
        } else {
            (0.0, 0.0)
        };
        let model = model_matrix(transform.position, yaw, pitch);
        let mvp = vp * model;
        shader.set_mvp(&mvp);
        render.mesh.draw(render.topology);
    }

    // --- Непрозрачные треугольники (материал) ---
    shader.set_vertex_color_mode(false);
    for (_, (transform, render, spin, mat)) in
        &mut world.query::<(&Position, &RenderMesh, &SpinAnimation, &Material)>()
    {
        if render.topology != MeshTopology::Triangles {
            continue;
        }
        if !mat.is_visible() || mat.has_transparency() {
            continue;
        }
        let (yaw, pitch) = if spin.enabled {
            (spin.phase_y, spin.phase_x)
        } else {
            (0.0, 0.0)
        };
        let model = model_matrix(transform.position, yaw, pitch);
        let mvp = vp * model;
        shader.set_mvp(&mvp);
        shader.set_material_rgba(mat.color.r, mat.color.g, mat.color.b, mat.opacity);
        render.mesh.draw(render.topology);
    }

    // --- Полупрозрачные треугольники (от дальнего к ближнему — без записи в глубину порядок критичен) ---
    set_transparent_depth_blend();
    shader.set_vertex_color_mode(false);

    let mut transparent: Vec<(f32, Entity)> =
        (&mut world.query::<(&Position, &RenderMesh, &SpinAnimation, &Material)>())
            .into_iter()
            .filter_map(|(e, (pos, render, _spin, mat))| {
                if render.topology != MeshTopology::Triangles {
                    return None;
                }
                if !mat.is_visible() || !mat.has_transparency() {
                    return None;
                }
                let d = pos.position - camera_eye;
                let dist_sq = d.x * d.x + d.y * d.y + d.z * d.z;
                Some((dist_sq, e))
            })
            .collect();

    transparent.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    for (_, e) in transparent {
        let Ok(mut q) = world.query_one::<(&Position, &RenderMesh, &SpinAnimation, &Material)>(e)
        else {
            continue;
        };
        let Some((transform, render, spin, mat)) = q.get() else {
            continue;
        };
        let (yaw, pitch) = if spin.enabled {
            (spin.phase_y, spin.phase_x)
        } else {
            (0.0, 0.0)
        };
        let model = model_matrix(transform.position, yaw, pitch);
        let mvp = vp * model;
        shader.set_mvp(&mvp);
        shader.set_material_rgba(mat.color.r, mat.color.g, mat.color.b, mat.opacity);
        render.mesh.draw(render.topology);
    }

    set_opaque_depth_blend();
}
