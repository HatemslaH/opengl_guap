//! Окно, поверхность OpenGL, контекст и связка с сценой.
//!
//! Сюда не добавляют геометрию конкретных моделей — только события окна и порядок вызовов рендера.

use crate::graphics::{ShaderProgram, enable_depth_test, mvp_matrix};
use crate::scene::{DrawContext, Scene};
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct GlutinApp {
    window: Option<Window>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    shader: Option<ShaderProgram>,
    scene: Option<Scene>,
    start_time: Instant,
}

impl Default for GlutinApp {
    fn default() -> Self {
        Self::new()
    }
}

impl GlutinApp {
    pub fn new() -> Self {
        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            shader: None,
            scene: None,
            start_time: Instant::now(),
        }
    }
}

impl ApplicationHandler for GlutinApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title("OpenGL Rust");
        let template = ConfigTemplateBuilder::new().with_depth_size(24);
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attrs));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|a, b| {
                        if a.num_samples() > b.num_samples() {
                            a
                        } else {
                            b
                        }
                    })
                    .expect("нет подходящего конфига OpenGL")
            })
            .expect("не удалось собрать окно и дисплей");

        let window = window.expect("окно должно быть создано вместе с дисплеем");
        let raw_handle = window.window_handle().expect("окно без raw handle");
        let display = gl_config.display();

        let context_attrs = ContextAttributesBuilder::new().build(Some(raw_handle.as_raw()));

        let gl_context = unsafe {
            display
                .create_context(&gl_config, &context_attrs)
                .expect("не удалось создать контекст OpenGL")
        };

        let (width, height): (u32, u32) = window.inner_size().into();
        let surface_attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_handle.as_raw(),
            NonZeroU32::new(width).expect("ширина окна не нулевая"),
            NonZeroU32::new(height).expect("высота окна не нулевая"),
        );

        let gl_surface = unsafe {
            display
                .create_window_surface(&gl_config, &surface_attrs)
                .expect("не удалось создать поверхность окна")
        };

        let gl_context = gl_context
            .make_current(&gl_surface)
            .expect("не удалось сделать контекст текущим");

        gl::load_with(|s| {
            let s = CString::new(s).expect("имя функции GL без внутреннего NUL");
            display.get_proc_address(&s)
        });

        let shader = ShaderProgram::new_colored_mesh();
        enable_depth_test();
        let scene = Scene::with_demo_cube();

        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
        self.window = Some(window);
        self.shader = Some(shader);
        self.scene = Some(scene);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let elapsed = self.start_time.elapsed().as_secs_f32();

                let (w_px, h_px) = self
                    .window
                    .as_ref()
                    .map(|w| {
                        let s = w.inner_size();
                        (s.width.max(1), s.height.max(1))
                    })
                    .unwrap_or((800, 600));
                let aspect = w_px as f32 / h_px as f32;

                if let (Some(shader), Some(scene)) = (&self.shader, &self.scene) {
                    let mvp = mvp_matrix(elapsed, aspect);

                    unsafe {
                        gl::ClearColor(0.1, 0.1, 0.15, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }

                    shader.use_program();
                    shader.set_mvp(&mvp);
                    let ctx = DrawContext { shader, mvp: &mvp };
                    scene.draw_all(&ctx);
                }

                if let (Some(ctx), Some(surface)) = (&self.gl_context, &self.gl_surface) {
                    surface.swap_buffers(ctx).expect("обмен буферов окна");
                }

                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                let nw = NonZeroU32::new(size.width.max(1)).expect("ширина после resize");
                let nh = NonZeroU32::new(size.height.max(1)).expect("высота после resize");
                if let (Some(ctx), Some(surface)) = (&self.gl_context, &self.gl_surface) {
                    surface.resize(ctx, nw, nh);
                }
                unsafe {
                    gl::Viewport(0, 0, nw.get() as i32, nh.get() as i32);
                }
                if let Some(win) = &self.window {
                    win.request_redraw();
                }
            }

            _ => {}
        }
    }
}
