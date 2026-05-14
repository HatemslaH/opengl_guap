//! Системы ECS: анимация вращения и отрисовка мешей.

use crate::ecs::components::{
    Camera, CameraKeyboardOrbit, CameraLookTarget, KeyboardOrbitKeys, Light, LightKind, Material,
    Position, RenderMesh, Rotation, Scale, SpinAnimation,
};
use crate::graphics::{
    MAX_DIRECTIONAL_LIGHTS, MAX_POINT_LIGHTS, MeshTopology, ShaderProgram,
    camera_eye_for_look_at_target, camera_view_projection_matrix,
    camera_yaw_pitch_deg_from_look_direction, model_matrix, set_opaque_depth_blend,
    set_transparent_depth_blend, view_projection_matrix,
};
use cgmath::{InnerSpace, Vector3};
use hecs::{Entity, World};

/// Для сущностей с [`SpinAnimation`] + [`Rotation`] и `enabled == true` добавляет скорости к углам [`Rotation`].
pub fn spin_animation_system(world: &mut World, dt: f32) {
    if dt <= 0.0 {
        return;
    }
    for (_, (spin, rot)) in world.query_mut::<(&SpinAnimation, &mut Rotation)>() {
        if spin.enabled {
            rot.xyz.y += spin.velocity_y * dt;
            rot.xyz.x += spin.velocity_x * dt;
        }
    }
}

/// Для сущностей с [`Position`] + [`Camera`] + [`CameraLookTarget`] + [`CameraKeyboardOrbit`] и `orbit.enabled`
/// двигает глаз по сфере вокруг цели по клавишам из `keys` (см. [`KeyboardOrbitKeys`]).
///
/// Вызывайте **до** [`camera_look_at_system`], чтобы затем пересчитать [`Rotation`].
pub fn camera_keyboard_orbit_system(world: &mut World, keys: &KeyboardOrbitKeys, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    let work: Vec<(Entity, Vector3<f32>, CameraLookTarget, CameraKeyboardOrbit)> = world
        .query::<(&Position, &Camera, &CameraLookTarget, &CameraKeyboardOrbit)>()
        .into_iter()
        .map(|(e, (pos, _cam, look, orbit))| (e, pos.position, look.clone(), orbit.clone()))
        .collect();

    for (entity, eye, look, mut orbit) in work {
        if !orbit.enabled {
            continue;
        }

        let target = match &look {
            CameraLookTarget::World(p) => *p,
            CameraLookTarget::Entity(target_e) => match world.get::<&Position>(*target_e) {
                Ok(t) => t.position,
                Err(_) => continue,
            },
        };

        let dir = target - eye;
        if !orbit.initialized {
            let dist = dir.magnitude();
            if dist > 1e-4
                && let Some((yaw, pitch)) = camera_yaw_pitch_deg_from_look_direction(dir)
            {
                orbit.yaw_deg = yaw;
                orbit.pitch_deg = pitch;
                orbit.distance = dist;
                orbit.initialized = true;
            }
        }

        if !orbit.initialized {
            continue;
        }

        if keys.right {
            orbit.yaw_deg -= orbit.yaw_speed_deg_per_sec * dt;
        }
        if keys.left {
            orbit.yaw_deg += orbit.yaw_speed_deg_per_sec * dt;
        }
        if keys.up {
            orbit.pitch_deg -= orbit.pitch_speed_deg_per_sec * dt;
        }
        if keys.down {
            orbit.pitch_deg += orbit.pitch_speed_deg_per_sec * dt;
        }
        orbit.pitch_deg = orbit
            .pitch_deg
            .clamp(orbit.pitch_min_deg, orbit.pitch_max_deg);

        let new_eye =
            camera_eye_for_look_at_target(target, orbit.distance, orbit.yaw_deg, orbit.pitch_deg);

        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            pos.position = new_eye;
        }
        if let Ok(mut o) = world.get::<&mut CameraKeyboardOrbit>(entity) {
            *o = orbit;
        }
    }
}

/// Для сущностей с [`Position`] + [`Camera`] + [`CameraLookTarget`] выставляет углы [`Rotation`],
/// чтобы взгляд был на мировую точку или на [`Position`] указанной сущности.
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
        if let Ok(mut rot) = world.get::<&mut Rotation>(camera_e) {
            rot.xyz.x = pitch;
            rot.xyz.y = yaw;
            rot.xyz.z = 0.0;
        }
    }
}

fn light_radiance(light: &Light) -> Vector3<f32> {
    Vector3::new(
        light.color.r * light.intensity,
        light.color.g * light.intensity,
        light.color.b * light.intensity,
    )
}

/// Собирает до [`MAX_DIRECTIONAL_LIGHTS`] направленных и [`MAX_POINT_LIGHTS`] точечных источников.
fn collect_lights(world: &World) -> LightUpload {
    let mut out = LightUpload::default();
    for (e, (light,)) in world.query::<(&Light,)>().into_iter() {
        let rad = light_radiance(light);
        match light.kind {
            LightKind::Directional { toward_light } => {
                if out.dir_count < MAX_DIRECTIONAL_LIGHTS {
                    out.dir_toward[out.dir_count] = toward_light;
                    out.dir_radiance[out.dir_count] = rad;
                    out.dir_count += 1;
                }
            }
            LightKind::Point {
                constant,
                linear,
                quadratic,
            } => {
                if out.point_count < MAX_POINT_LIGHTS
                    && let Ok(pos) = world.get::<&Position>(e)
                {
                    out.point_pos[out.point_count] = pos.position;
                    out.point_radiance[out.point_count] = rad;
                    out.point_atten[out.point_count] = Vector3::new(constant, linear, quadratic);
                    out.point_count += 1;
                }
            }
        }
    }
    out
}

#[derive(Clone, Copy)]
struct LightUpload {
    dir_toward: [Vector3<f32>; MAX_DIRECTIONAL_LIGHTS],
    dir_radiance: [Vector3<f32>; MAX_DIRECTIONAL_LIGHTS],
    dir_count: usize,
    point_pos: [Vector3<f32>; MAX_POINT_LIGHTS],
    point_radiance: [Vector3<f32>; MAX_POINT_LIGHTS],
    point_atten: [Vector3<f32>; MAX_POINT_LIGHTS],
    point_count: usize,
}

impl Default for LightUpload {
    fn default() -> Self {
        Self {
            dir_toward: [Vector3::new(0.0, 0.0, 0.0); MAX_DIRECTIONAL_LIGHTS],
            dir_radiance: [Vector3::new(0.0, 0.0, 0.0); MAX_DIRECTIONAL_LIGHTS],
            dir_count: 0,
            point_pos: [Vector3::new(0.0, 0.0, 0.0); MAX_POINT_LIGHTS],
            point_radiance: [Vector3::new(0.0, 0.0, 0.0); MAX_POINT_LIGHTS],
            point_atten: [Vector3::new(0.0, 0.0, 0.0); MAX_POINT_LIGHTS],
            point_count: 0,
        }
    }
}

impl LightUpload {
    fn upload(&self, shader: &ShaderProgram) {
        shader.set_frame_lights(
            &self.dir_toward[..self.dir_count],
            &self.dir_radiance[..self.dir_count],
            &self.point_pos[..self.point_count],
            &self.point_radiance[..self.point_count],
            &self.point_atten[..self.point_count],
        );
    }
}

/// Рисует сущности с [`Position`] + [`Rotation`] + [`Scale`] + [`RenderMesh`].
///
/// - **Линии** — только цвет вершины (сетка), без материала.
/// - **Треугольники** — только если есть [`Material`]; без материала меш не рисуется.
/// - Непрозрачные треугольники (`opacity ≈ 1`) — без смешивания и с записью в Z-буфер;
///   полупрозрачные — отдельный проход с `GL_BLEND` и без записи в глубину.
///
/// Матрица вида×проекция — с первой сущности [`Position`] + [`Rotation`] + [`Camera`]; если камеры нет — [`view_projection_matrix`].
///
/// Полупрозрачные треугольники сортируются **от дальнего к ближнему** к камере (painter's algorithm): иначе при
/// выключенной записи в Z-буфер задний объект ошибочно рисуется поверх переднего.
///
/// Источники [`Light`] в мире передаются во фрагментный шейдер (Blinn–Phong); сетка рисуется без учёта света.
pub fn render_mesh_system(world: &mut World, shader: &ShaderProgram, aspect: f32) {
    let (vp, camera_eye) = if let Some((_, (pos, rot, cam))) =
        (&mut world.query::<(&Position, &Rotation, &Camera)>())
            .into_iter()
            .next()
    {
        (
            camera_view_projection_matrix(
                pos.position,
                rot.xyz.y.to_radians(),
                rot.xyz.x.to_radians(),
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

    let lights = collect_lights(world);

    shader.use_program();
    shader.set_camera_pos(camera_eye);
    lights.upload(shader);

    set_opaque_depth_blend();

    // --- Линии: вершинный цвет, без материала ---
    shader.set_vertex_color_mode(true);
    for (_, (transform, rot, scale, render)) in
        &mut world.query::<(&Position, &Rotation, &Scale, &RenderMesh)>()
    {
        if render.topology != MeshTopology::Lines {
            continue;
        }
        let model = model_matrix(transform.position, rot.xyz, scale.xyz);
        let mvp = vp * model;
        shader.set_model_normal_mvp(&model, &mvp);
        render.mesh.draw(render.topology);
    }

    // --- Непрозрачные треугольники (материал) ---
    shader.set_vertex_color_mode(false);
    for (_, (transform, rot, scale, render, mat)) in
        &mut world.query::<(&Position, &Rotation, &Scale, &RenderMesh, &Material)>()
    {
        if render.topology != MeshTopology::Triangles {
            continue;
        }
        if !mat.is_visible() || mat.has_transparency() {
            continue;
        }
        let model = model_matrix(transform.position, rot.xyz, scale.xyz);
        let mvp = vp * model;
        shader.set_model_normal_mvp(&model, &mvp);
        shader.set_material_rgba(mat.color.r, mat.color.g, mat.color.b, mat.opacity);
        shader.set_surface_lighting(
            mat.surface.ambient,
            mat.surface.diffuse,
            mat.surface.specular_color.r,
            mat.surface.specular_color.g,
            mat.surface.specular_color.b,
            mat.surface.shininess,
        );
        render.mesh.draw(render.topology);
    }

    // --- Полупрозрачные треугольники (от дальнего к ближнему — без записи в глубину порядок критичен) ---
    set_transparent_depth_blend();
    shader.set_vertex_color_mode(false);

    let mut transparent: Vec<(f32, Entity)> =
        (&mut world.query::<(&Position, &Rotation, &Scale, &RenderMesh, &Material)>())
            .into_iter()
            .filter_map(|(e, (pos, _rot, _scale, render, mat))| {
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
        let Ok(mut q) =
            world.query_one::<(&Position, &Rotation, &Scale, &RenderMesh, &Material)>(e)
        else {
            continue;
        };
        let Some((transform, rot, scale, render, mat)) = q.get() else {
            continue;
        };
        let model = model_matrix(transform.position, rot.xyz, scale.xyz);
        let mvp = vp * model;
        shader.set_model_normal_mvp(&model, &mvp);
        shader.set_material_rgba(mat.color.r, mat.color.g, mat.color.b, mat.opacity);
        shader.set_surface_lighting(
            mat.surface.ambient,
            mat.surface.diffuse,
            mat.surface.specular_color.r,
            mat.surface.specular_color.g,
            mat.surface.specular_color.b,
            mat.surface.shininess,
        );
        render.mesh.draw(render.topology);
    }

    set_opaque_depth_blend();
}
