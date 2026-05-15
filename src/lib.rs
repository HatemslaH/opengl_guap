//! Educational OpenGL 3.3 (Core) on Rust: window through glutin/winit, simple render.
//!
//! **Where to start:** [`app::runner::run`] — entry point. Window logic in [`app::glutin_app`],
//! low-level GL in [`graphics`], ECS and entities in [`ecs`] and [`scene`].

pub mod app;
pub mod ecs;
pub mod game;
pub mod graphics;
pub mod scene;

/// Run the event loop and render. Errors during initialization panic with a Russian message (educational code).
pub fn run() {
    app::runner::run();
}
