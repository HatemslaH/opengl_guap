use hecs::World;

use crate::engine::ecs::components::Velocity;
use crate::engine::input::action::ActionState;
use crate::game::components::Player;
use crate::game::input::action::GameAction;

/// Sets horizontal [`Velocity::linear`] from WASD (world XZ). Diagonals are normalized so
/// strafing is not faster than cardinal movement. Vertical `linear.y` is left unchanged (gravity / jump).
pub fn player_movement_system(world: &mut World, actions: &ActionState<GameAction>, _dt: f32) {
    for (_, (player, velocity)) in world.query_mut::<(&Player, &mut Velocity)>() {
        let mut ix = 0.0f32;
        let mut iz = 0.0f32;
        if actions.is_active(GameAction::MoveRight) {
            ix += 1.0;
        }
        if actions.is_active(GameAction::MoveLeft) {
            ix -= 1.0;
        }
        if actions.is_active(GameAction::MoveForward) {
            iz -= 1.0;
        }
        if actions.is_active(GameAction::MoveBackward) {
            iz += 1.0;
        }

        let len_sq = ix * ix + iz * iz;
        let (dx, dz) = if len_sq > 1e-12 {
            let inv = len_sq.sqrt().recip();
            (ix * inv, iz * inv)
        } else {
            (0.0, 0.0)
        };

        let vy = velocity.linear.y;
        velocity.linear.x = dx * player.speed;
        velocity.linear.y = vy;
        velocity.linear.z = dz * player.speed;
    }
}
