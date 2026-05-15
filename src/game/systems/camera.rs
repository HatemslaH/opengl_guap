use crate::ecs::Rotation;
use crate::ecs::components::Camera;
use crate::ecs::components::Position;
use crate::game::components::CameraKeyboardOrbit;
use crate::game::components::CameraLookTarget;
use crate::game::components::KeyboardOrbitKeys;
use crate::graphics::{camera_eye_for_look_at_target, camera_yaw_pitch_deg_from_look_direction};
use cgmath::{InnerSpace, Vector3};
use hecs::{Entity, World};

/// For entities with [`Position`] + [`Camera`] + [`CameraLookTarget`] + [`CameraKeyboardOrbit`] and `orbit.enabled`
/// moves the eye around the target on the sphere by keys from `keys` (see [`KeyboardOrbitKeys`]).
///
/// Call **before** [`camera_look_at_system`], to then recalculate [`Rotation`].
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

/// For entities with [`Position`] + [`Camera`] + [`CameraLookTarget`] sets the angles [`Rotation`],
/// so that the look is on the world point or on the [`Position`] of the specified entity.
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
