use cgmath::Vector3;
use hecs::{Entity, World};

use crate::{engine::ecs::Position, game::components::CameraFollow};

pub fn camera_follow_system(world: &mut World) {
    let targets: Vec<(Entity, Vector3<f32>)> = world
        .query::<(&CameraFollow,)>()
        .iter()
        .filter_map(|(cam_e, (follow,))| {
            let target_pos = world.get::<&Position>(follow.target).ok()?.position;
            Some((cam_e, target_pos + follow.offset))
        })
        .collect();

    for (cam_e, new_pos) in targets {
        if let Ok(mut pos) = world.get::<&mut Position>(cam_e) {
            pos.position = new_pos;
        }
    }
}
