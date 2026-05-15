//! Run `EventLoop` and application with winit handler.

use crate::game::input::action::GameAction;
use crate::{app::glutin_app::GlutinApp, engine::input::action::ActionMap};
use winit::{event_loop::EventLoop, keyboard::KeyCode};

/// Entry point of the library: creates an event loop and passes control to [`GlutinApp`].
pub fn run() {
    let event_loop = EventLoop::new().expect("failed to create EventLoop");
    let mut game_map = ActionMap::new();
    game_map
        .bind(KeyCode::KeyD, GameAction::MoveRight)
        .bind(KeyCode::KeyA, GameAction::MoveLeft)
        .bind(KeyCode::KeyW, GameAction::MoveForward)
        .bind(KeyCode::KeyS, GameAction::MoveBackward)
        .bind(KeyCode::KeyQ, GameAction::RotateLeft)
        .bind(KeyCode::KeyE, GameAction::RotateRight);

    let mut app = GlutinApp::new(game_map);
    event_loop
        .run_app(&mut app)
        .expect("event loop terminated with an error");
}
