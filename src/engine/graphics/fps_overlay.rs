//! Debug HUD in the upper-left (NDC, separate GL program): FPS, frame / scene / overlay / present
//! CPU times (ms), and scene draw-call counts from [`SceneRenderStats`].

use font8x8::{BASIC_FONTS, UnicodeFonts};
use std::ffi::CString;

use super::frame_stats::SceneRenderStats;

/// Values for one frame, measured in [`crate::app::glutin_app::GlutinApp`] (CPU wall time around GL).
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameHudMetrics {
    pub dt_secs: f32,
    /// Wall time inside [`crate::engine::ecs::systems::render::render_mesh_system`].
    pub scene_cpu_ms: f32,
    /// Wall time for the previous frame's buffer swap (`swap_buffers`), for HUD display.
    pub last_swap_cpu_ms: f32,
    pub scene: SceneRenderStats,
}

const VERT: &str = r#"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec3 aColor;
out vec3 vColor;
void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
    vColor = aColor;
}
"#;

const FRAG: &str = r#"#version 330 core
in vec3 vColor;
out vec4 FragColor;
void main() {
    FragColor = vec4(vColor, 1.0);
}
"#;

fn compile_shader(stage: u32, src: &str) -> u32 {
    unsafe {
        let id = gl::CreateShader(stage);
        let c = CString::new(src).expect("GLSL without internal NUL");
        gl::ShaderSource(id, 1, &c.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        let mut ok: i32 = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut ok);
        if ok == gl::FALSE as i32 {
            let mut len: i32 = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len.max(0) as usize];
            if !buf.is_empty() {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buf.as_mut_ptr().cast());
            }
            let msg = String::from_utf8_lossy(&buf);
            panic!("FPS overlay: shader compilation error: {msg}");
        }
        id
    }
}

fn link_program(vs: u32, fs: u32) -> u32 {
    unsafe {
        let id = gl::CreateProgram();
        gl::AttachShader(id, vs);
        gl::AttachShader(id, fs);
        gl::LinkProgram(id);
        let mut ok: i32 = 0;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut ok);
        if ok == gl::FALSE as i32 {
            let mut len: i32 = 0;
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len.max(0) as usize];
            if !buf.is_empty() {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buf.as_mut_ptr().cast());
            }
            let msg = String::from_utf8_lossy(&buf);
            panic!("FPS overlay: program linking error: {msg}");
        }
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        id
    }
}

#[inline]
fn pixel_to_ndc(px: f32, py: f32, w: f32, h: f32) -> (f32, f32) {
    let x = (px + 0.5) / w * 2.0 - 1.0;
    let y = 1.0 - (py + 0.5) / h * 2.0;
    (x, y)
}

fn push_quad_pixels(
    buf: &mut Vec<f32>,
    (px0, py0, px1, py1): (f32, f32, f32, f32),
    w: f32,
    h: f32,
    rgb: [f32; 3],
) {
    let (tlx, tly) = pixel_to_ndc(px0, py0, w, h);
    let (trx, try_) = pixel_to_ndc(px1, py0, w, h);
    let (blx, bly) = pixel_to_ndc(px0, py1, w, h);
    let (brx, bry) = pixel_to_ndc(px1, py1, w, h);
    for (x, y) in [
        (tlx, tly),
        (trx, try_),
        (blx, bly),
        (trx, try_),
        (brx, bry),
        (blx, bly),
    ] {
        buf.extend_from_slice(&[x, y, rgb[0], rgb[1], rgb[2]]);
    }
}

struct BitmapFontBatch<'a> {
    scratch: &'a mut Vec<f32>,
    w: f32,
    h: f32,
    scale: f32,
    gap_px: f32,
}

impl BitmapFontBatch<'_> {
    fn append_line(&mut self, text: &str, mut pen_x: f32, pen_y: f32, rgb: [f32; 3]) {
        let space = BASIC_FONTS.get(' ').expect("space in BASIC_FONTS");
        for ch in text.chars() {
            let g = BASIC_FONTS.get(ch).unwrap_or(space);
            for (row, &byte) in g.iter().enumerate() {
                for bit in 0..8u32 {
                    if byte & (1 << bit) != 0 {
                        let px0 = pen_x + bit as f32 * self.scale;
                        let py0 = pen_y + row as f32 * self.scale;
                        let px1 = px0 + self.scale;
                        let py1 = py0 + self.scale;
                        push_quad_pixels(self.scratch, (px0, py0, px1, py1), self.w, self.h, rgb);
                    }
                }
            }
            pen_x += 8.0 * self.scale + self.gap_px;
        }
    }
}

/// Upper-left debug HUD; draw after the scene, on top of the frame.
pub struct FpsOverlay {
    program: u32,
    vao: u32,
    vbo: u32,
    scratch: Vec<f32>,
    fps_ema: f32,
    /// Smoothed CPU time for building + drawing this HUD (previous frame's value used while building text).
    hud_cpu_ms_ema: f32,
}

#[allow(clippy::new_without_default)]
impl FpsOverlay {
    pub fn new() -> Self {
        let program = {
            let vs = compile_shader(gl::VERTEX_SHADER, VERT);
            let fs = compile_shader(gl::FRAGMENT_SHADER, FRAG);
            link_program(vs, fs)
        };

        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            let stride = (5 * std::mem::size_of::<f32>()) as i32;
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            let color_off = (2 * std::mem::size_of::<f32>()) as *const std::ffi::c_void;
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, color_off);
            gl::EnableVertexAttribArray(1);
            gl::BindVertexArray(0);
        }

        Self {
            program,
            vao,
            vbo,
            scratch: Vec::with_capacity(64_000),
            fps_ema: -1.0,
            hud_cpu_ms_ema: -1.0,
        }
    }

    /// Updates smoothed FPS from `hud.dt_secs` and draws several lines of stats.
    pub fn draw(&mut self, width_px: u32, height_px: u32, hud: &FrameHudMetrics) {
        let t_total = std::time::Instant::now();
        let w = width_px.max(1) as f32;
        let h = height_px.max(1) as f32;
        let dt = hud.dt_secs;

        if dt > 1e-7 {
            let inst = 1.0 / dt;
            self.fps_ema = if self.fps_ema < 0.0 {
                inst
            } else {
                self.fps_ema * 0.9 + inst * 0.1
            };
        }

        let frame_ms = dt * 1000.0;
        let s = hud.scene;
        let hud_label = self.hud_cpu_ms_ema.max(0.0);
        let line1 = format!(
            "FPS {:>4.0}  frame {:>6.2}ms  swap {:>5.2}ms",
            self.fps_ema.max(0.0),
            frame_ms,
            hud.last_swap_cpu_ms
        );
        let line2 = format!("3D {:>6.2}ms  HUD {:>5.2}ms", hud.scene_cpu_ms, hud_label);
        let line3 = format!(
            "draws {:>4} (+1 HUD)  L:{} T:{} A:{}",
            s.draw_calls, s.lines_drawn, s.opaque_triangles_drawn, s.transparent_triangles_drawn
        );

        let scale = 2.5_f32;
        let gap_px = 2.0_f32;
        let margin = 6.0_f32;
        let line_step = 8.0 * scale + 5.0;
        let rgb_a = [0.35_f32, 1.0_f32, 0.48_f32];
        let rgb_b = [0.45_f32, 0.88_f32, 1.0_f32];
        let rgb_c = [1.0_f32, 0.92_f32, 0.35_f32];

        self.scratch.clear();
        let mut batch = BitmapFontBatch {
            scratch: &mut self.scratch,
            w,
            h,
            scale,
            gap_px,
        };
        batch.append_line(&line1, margin, margin, rgb_a);
        batch.append_line(&line2, margin, margin + line_step, rgb_b);
        batch.append_line(&line3, margin, margin + 2.0 * line_step, rgb_c);

        let n = (self.scratch.len() / 5) as i32;
        if n <= 0 {
            return;
        }

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(self.scratch.as_slice()) as isize,
                self.scratch.as_ptr().cast(),
                gl::STREAM_DRAW,
            );
            gl::DrawArrays(gl::TRIANGLES, 0, n);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
            gl::Enable(gl::DEPTH_TEST);
        }

        let hud_ms = t_total.elapsed().as_secs_f32() * 1000.0;
        self.hud_cpu_ms_ema = if self.hud_cpu_ms_ema < 0.0 {
            hud_ms
        } else {
            self.hud_cpu_ms_ema * 0.9 + hud_ms * 0.1
        };
    }
}

impl Drop for FpsOverlay {
    fn drop(&mut self) {
        unsafe {
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.program != 0 {
                gl::DeleteProgram(self.program);
            }
        }
    }
}
