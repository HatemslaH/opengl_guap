//! Запуск `EventLoop` и приложения с обработчиком winit.

use crate::app::glutin_app::GlutinApp;
use winit::event_loop::EventLoop;

/// Точка входа библиотеки: создаёт цикл событий и передаёт управление [`GlutinApp`].
pub fn run() {
    let event_loop = EventLoop::new().expect("не удалось создать EventLoop");
    let mut app = GlutinApp::new();
    event_loop
        .run_app(&mut app)
        .expect("цикл событий завершился с ошибкой");
}
