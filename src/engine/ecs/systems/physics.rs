use hecs::World;

use crate::engine::ecs::{Position, components::Velocity};

pub fn physics_system(world: &mut World, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    for (_, (position, velocity)) in world.query_mut::<(&mut Position, &Velocity)>() {
        position.position += velocity.linear * dt;
    }
}
