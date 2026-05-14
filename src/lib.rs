//! Учебный OpenGL 3.3 (Core) на Rust: окно через glutin/winit, простой рендер.
//!
//! **С чего начать:** [`app::runner::run`] — точка входа. Логика окна в [`app::glutin_app`],
//! низкоуровневый GL в [`graphics`], ECS и сущности в [`ecs`] и [`scene`].

pub mod app;
pub mod ecs;
pub mod graphics;
pub mod scene;

/// Запуск цикла событий и рендера. Ошибки инициализации паникуют с русским сообщением (учебный код).
pub fn run() {
    app::runner::run();
}
