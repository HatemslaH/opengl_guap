//! Window, OpenGL surface, context and binding with ECS scene.
//!
//! Here we don't add entity components — only window events and system order.

use crate::engine::ecs::systems::{physics_system, render_mesh_system};
use crate::engine::graphics::{FpsOverlay, FrameHudMetrics, ShaderProgram, enable_depth_test};
use crate::engine::input::action::ActionMap;
use crate::engine::input::raw::RawInputState;
use crate::engine::scene::Scene;
use crate::game::components::camera_follow_system;
use crate::game::input::action::GameAction;
use crate::game::scenes::demo2::{build_demo2, build_demo2_default};
use crate::game::scenes::demo3::build_demo3;
use crate::game::systems::{camera_look_at_system, player_movement_system};
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId};

pub struct GlutinApp {
    window: Option<Window>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    shader: Option<ShaderProgram>,
    fps_overlay: Option<FpsOverlay>,
    scene: Option<Scene>,
    /// Moment of the end of the previous frame (for `dt` animation and FPS).
    last_frame_instant: Option<Instant>,
    raw_input: RawInputState,
    game_action_map: ActionMap<GameAction>,
    /// Rotation of all meshes around the world axis Y (degrees); light sources are stationary in the world.
    scene_root_yaw_deg: f32,
    /// CPU time for the previous frame's `swap_buffers` (shown on HUD as "swap").
    last_swap_cpu_ms: f32,
}

impl Default for GlutinApp {
    fn default() -> Self {
        Self::new(ActionMap::default())
    }
}

impl GlutinApp {
    pub fn new(game_action_map: ActionMap<GameAction>) -> Self {
        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            shader: None,
            fps_overlay: None,
            scene: None,
            last_frame_instant: None,
            raw_input: RawInputState::default(),
            game_action_map,
            scene_root_yaw_deg: 0.0,
            last_swap_cpu_ms: 0.0,
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
                    .expect("no suitable OpenGL config")
            })
            .expect("failed to collect window and display");

        let window = window.expect("window should be created together with the display");
        let raw_handle = window.window_handle().expect("window without raw handle");
        let display = gl_config.display();

        let context_attrs = ContextAttributesBuilder::new().build(Some(raw_handle.as_raw()));

        let gl_context = unsafe {
            display
                .create_context(&gl_config, &context_attrs)
                .expect("failed to create OpenGL context")
        };

        let (width, height): (u32, u32) = window.inner_size().into();
        let surface_attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_handle.as_raw(),
            NonZeroU32::new(width).expect("window width is not zero"),
            NonZeroU32::new(height).expect("window height is not zero"),
        );

        let gl_surface = unsafe {
            display
                .create_window_surface(&gl_config, &surface_attrs)
                .expect("failed to create window surface")
        };

        let gl_context = gl_context
            .make_current(&gl_surface)
            .expect("failed to make context current");

        if let Err(e) = gl_surface.set_swap_interval(&gl_context, SwapInterval::DontWait) {
            eprintln!(
                "Failed to disable V-Sync: {e:?}. FPS counter will be around the monitor frequency (~144 Hz)."
            );
        }

        gl::load_with(|s| {
            let s = CString::new(s).expect("GL function name without internal NUL");
            display.get_proc_address(&s)
        });

        let shader = ShaderProgram::new_colored_mesh();
        enable_depth_test();
        let scene = build_demo2_default();
        let fps_overlay = FpsOverlay::new();

        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
        self.window = Some(window);
        self.shader = Some(shader);
        self.fps_overlay = Some(fps_overlay);
        self.scene = Some(scene);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = self
                    .last_frame_instant
                    .map(|t| now.saturating_duration_since(t).as_secs_f32())
                    .unwrap_or(0.0)
                    .clamp(0.0, 0.25);
                self.last_frame_instant = Some(now);

                let (w_px, h_px) = self
                    .window
                    .as_ref()
                    .map(|w| {
                        let s = w.inner_size();
                        (s.width.max(1), s.height.max(1))
                    })
                    .unwrap_or((800, 600));
                let aspect = w_px as f32 / h_px as f32;

                if let (Some(shader), Some(scene)) = (&self.shader, &mut self.scene) {
                    let actions = self.game_action_map.resolve(&self.raw_input);

                    // camera_keyboard_orbit_system(&mut scene.world, &actions, dt);
                    camera_follow_system(&mut scene.world);
                    camera_look_at_system(&mut scene.world);
                    player_movement_system(&mut scene.world, &actions, dt);
                    physics_system(&mut scene.world, dt);

                    const SCENE_YAW_DEG_PER_SEC: f32 = 52.0;
                    if actions.is_active(GameAction::RotateLeft) {
                        self.scene_root_yaw_deg -= SCENE_YAW_DEG_PER_SEC * dt;
                    }
                    if actions.is_active(GameAction::RotateRight) {
                        self.scene_root_yaw_deg += SCENE_YAW_DEG_PER_SEC * dt;
                    }

                    unsafe {
                        gl::ClearColor(0.1, 0.1, 0.15, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }

                    let t_scene = Instant::now();
                    let scene_stats = render_mesh_system(
                        &mut scene.world,
                        shader,
                        aspect,
                        self.scene_root_yaw_deg,
                    );
                    let scene_ms = t_scene.elapsed().as_secs_f32() * 1000.0;

                    let hud = FrameHudMetrics {
                        dt_secs: dt,
                        scene_cpu_ms: scene_ms,
                        last_swap_cpu_ms: self.last_swap_cpu_ms,
                        scene: scene_stats,
                    };
                    if let Some(fps) = self.fps_overlay.as_mut() {
                        fps.draw(w_px, h_px, &hud);
                    }
                }

                if let (Some(ctx), Some(surface)) = (&self.gl_context, &self.gl_surface) {
                    let t_swap = Instant::now();
                    surface.swap_buffers(ctx).expect("window buffer swap");
                    self.last_swap_cpu_ms = t_swap.elapsed().as_secs_f32() * 1000.0;
                }

                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::Focused(focused) => {
                if !focused {
                    self.raw_input = RawInputState::default();
                    self.game_action_map = ActionMap::default();
                    self.last_frame_instant = None;
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    self.raw_input.set_key(code, pressed);
                }
            }

            WindowEvent::Resized(size) => {
                let nw = NonZeroU32::new(size.width.max(1)).expect("window width after resize");
                let nh = NonZeroU32::new(size.height.max(1)).expect("window height after resize");
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
