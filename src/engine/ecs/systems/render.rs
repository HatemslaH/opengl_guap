use std::sync::Arc;

use cgmath::{Matrix4, Rad, Vector3, Vector4};
use hecs::{Entity, World};

use crate::engine::{
    ecs::{Camera, Light, LightKind, Material, Position, RenderMesh, Rotation, Scale},
    graphics::{
        InstanceData, MAX_DIRECTIONAL_LIGHTS, MAX_POINT_LIGHTS, Mesh, MeshTopology,
        SceneRenderStats, ShaderProgram, camera_view_projection_matrix, model_matrix,
        set_opaque_depth_blend, set_transparent_depth_blend, view_projection_matrix,
    },
};

/// Draws entities with [`Position`] + [`Rotation`] + [`Scale`] + [`RenderMesh`].
///
/// - **Lines** — vertex color only (mesh grid), no material.
/// - **Triangles** — only if there is a [`Material`]; meshes without material are not drawn.
/// - Opaque triangles (`opacity ≈ 1`) — without blending and with Z-buffer write;
///   semi-transparent — drawn in a separate pass with `GL_BLEND` and no depth write.
///
/// The view×projection matrix is taken from the first entity with [`Position`] + [`Rotation`] + [`Camera`];
/// if there is no camera — [`view_projection_matrix`] is used.
///
/// Semi-transparent triangles are sorted **from farthest to closest** to the camera (painter's algorithm):
/// otherwise, with depth write disabled, the rear object would be erroneously drawn on top of the front object.
///
/// [`Light`] sources in the world are passed to the fragment shader (Blinn–Phong); mesh grid is drawn unlit.
///
/// Meshes are multiplied on the left by a rotation `R_y(scene_root_yaw_deg)` around the origin:
/// **scene objects** rotate, the world positions of the light sources in the shader remain stationary.
///
/// Identical [`Mesh`] + same [`Material`] are batched into one `glDrawArraysInstanced` call.
pub fn render_mesh_system(
    world: &mut World,
    shader: &ShaderProgram,
    aspect: f32,
    scene_root_yaw_deg: f32,
) -> SceneRenderStats {
    let mut stats = SceneRenderStats::default();
    let scene_root = Matrix4::from_angle_y(Rad(scene_root_yaw_deg.to_radians()));
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
    shader.set_vp(&vp);
    shader.set_camera_pos(camera_eye);
    lights.upload(shader);

    set_opaque_depth_blend();

    // --- Lines: vertex color only, no material ---
    shader.set_vertex_color_mode(true);
    let mut line_batches: Vec<(Arc<Mesh>, Vec<InstanceData>)> = Vec::new();
    for (_, (transform, rot, scale, render)) in
        &mut world.query::<(&Position, &Rotation, &Scale, &RenderMesh)>()
    {
        if render.topology != MeshTopology::Lines {
            continue;
        }
        let model_local = model_matrix(transform.position, rot.xyz, scale.xyz);
        let model = scene_root * model_local;
        let inst = InstanceData::from_world_model(model);
        if let Some((_, instances)) = line_batches
            .iter_mut()
            .find(|(m, _)| Arc::ptr_eq(m, &render.mesh))
        {
            instances.push(inst);
        } else {
            line_batches.push((Arc::clone(&render.mesh), vec![inst]));
        }
    }
    for (mesh, instances) in line_batches {
        mesh.draw_instanced(MeshTopology::Lines, &instances);
        stats.record_line_draw();
    }

    // --- Opaque triangles (with material) ---
    shader.set_vertex_color_mode(false);
    let mut opaque_batches: Vec<(Arc<Mesh>, Material, Vec<InstanceData>)> = Vec::new();
    for (_, (transform, rot, scale, render, mat)) in
        &mut world.query::<(&Position, &Rotation, &Scale, &RenderMesh, &Material)>()
    {
        if render.topology != MeshTopology::Triangles {
            continue;
        }
        if !mat.is_visible() || mat.has_transparency() {
            continue;
        }
        let model_local = model_matrix(transform.position, rot.xyz, scale.xyz);
        let model = scene_root * model_local;
        let inst = InstanceData::from_world_model(model);
        if let Some((_, _, instances)) = opaque_batches.iter_mut().find(|(m, m2, _)| {
            Arc::ptr_eq(m, &render.mesh) && *m2 == *mat
        }) {
            instances.push(inst);
        } else {
            opaque_batches.push((Arc::clone(&render.mesh), *mat, vec![inst]));
        }
    }
    for (mesh, mat, instances) in opaque_batches {
        shader.set_material_rgba(mat.color.r, mat.color.g, mat.color.b, mat.opacity);
        shader.set_surface_lighting(
            mat.surface.ambient,
            mat.surface.diffuse,
            mat.surface.specular_color.r,
            mat.surface.specular_color.g,
            mat.surface.specular_color.b,
            mat.surface.shininess,
        );
        mesh.draw_instanced(MeshTopology::Triangles, &instances);
        stats.record_triangle_draw(false);
    }

    // --- Semi-transparent triangles (from farthest to nearest—draw order is critical without depth write) ---
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
                let p = pos.position;
                let world_center = (scene_root * Vector4::new(p.x, p.y, p.z, 1.0)).truncate();
                let d = world_center - camera_eye;
                let dist_sq = d.x * d.x + d.y * d.y + d.z * d.z;
                Some((dist_sq, e))
            })
            .collect();

    transparent.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut i = 0usize;
    while i < transparent.len() {
        let e0 = transparent[i].1;
        let Ok(mut q0) = world.query_one::<(&Position, &Rotation, &Scale, &RenderMesh, &Material)>(e0)
        else {
            i += 1;
            continue;
        };
        let Some((transform0, rot0, scale0, render0, mat0)) = q0.get() else {
            i += 1;
            continue;
        };
        let mesh0 = Arc::clone(&render0.mesh);
        let mat0 = *mat0;
        let mut instances = Vec::new();
        let model_local = model_matrix(transform0.position, rot0.xyz, scale0.xyz);
        let model = scene_root * model_local;
        instances.push(InstanceData::from_world_model(model));

        let mut j = i + 1;
        while j < transparent.len() {
            let e = transparent[j].1;
            let Ok(mut q) =
                world.query_one::<(&Position, &Rotation, &Scale, &RenderMesh, &Material)>(e)
            else {
                break;
            };
            let Some((transform, rot, scale, render, mat)) = q.get() else {
                break;
            };
            if !Arc::ptr_eq(&render.mesh, &mesh0) || *mat != mat0 {
                break;
            }
            let model_local = model_matrix(transform.position, rot.xyz, scale.xyz);
            let model = scene_root * model_local;
            instances.push(InstanceData::from_world_model(model));
            j += 1;
        }

        shader.set_material_rgba(mat0.color.r, mat0.color.g, mat0.color.b, mat0.opacity);
        shader.set_surface_lighting(
            mat0.surface.ambient,
            mat0.surface.diffuse,
            mat0.surface.specular_color.r,
            mat0.surface.specular_color.g,
            mat0.surface.specular_color.b,
            mat0.surface.shininess,
        );
        mesh0.draw_instanced(MeshTopology::Triangles, &instances);
        stats.record_triangle_draw(true);

        i = j;
    }

    set_opaque_depth_blend();
    stats
}

/// Collects up to [`MAX_DIRECTIONAL_LIGHTS`] directional and [`MAX_POINT_LIGHTS`] point light sources.
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

fn light_radiance(light: &Light) -> Vector3<f32> {
    Vector3::new(
        light.color.r * light.intensity,
        light.color.g * light.intensity,
        light.color.b * light.intensity,
    )
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
