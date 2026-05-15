//! Run `EventLoop` and application with winit handler.

use crate::app::glutin_app::GlutinApp;
use winit::event_loop::EventLoop;

/// Entry point of the library: creates an event loop and passes control to [`GlutinApp`].
pub fn run() {
    let event_loop = EventLoop::new().expect("failed to create EventLoop");
    let mut app = GlutinApp::new();
    event_loop
        .run_app(&mut app)
        .expect("event loop terminated with an error");
}
