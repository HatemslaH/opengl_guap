//! Text FPS in the left upper corner of the window (NDC, separate GL program).

use font8x8::{BASIC_FONTS, UnicodeFonts};
use std::ffi::CString;

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

/// Overlay «FPS …» in the left upper corner; draw after the scene, on top of the frame.
pub struct FpsOverlay {
    program: u32,
    vao: u32,
    vbo: u32,
    scratch: Vec<f32>,
    fps_ema: f32,
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
            scratch: Vec::with_capacity(12_000),
            fps_ema: -1.0,
        }
    }

    /// Updates the smoothed FPS by `dt_secs` and draws the text on top of the frame.
    pub fn draw(&mut self, width_px: u32, height_px: u32, dt_secs: f32) {
        let w = width_px.max(1) as f32;
        let h = height_px.max(1) as f32;

        if dt_secs > 1e-7 {
            let inst = 1.0 / dt_secs;
            self.fps_ema = if self.fps_ema < 0.0 {
                inst
            } else {
                self.fps_ema * 0.9 + inst * 0.1
            };
        }

        let text = format!("FPS {:.0}", self.fps_ema.max(0.0));
        let space = BASIC_FONTS.get(' ').expect("space in BASIC_FONTS");

        self.scratch.clear();
        let scale = 2.5_f32;
        let gap_px = 2.0_f32;
        let margin = 6.0_f32;
        let mut pen_x = margin;
        let pen_y = margin;
        let rgb = [0.35_f32, 1.0_f32, 0.48_f32];

        for ch in text.chars() {
            let g = BASIC_FONTS.get(ch).unwrap_or(space);
            for (row, &byte) in g.iter().enumerate() {
                for bit in 0..8u32 {
                    if byte & (1 << bit) != 0 {
                        let px0 = pen_x + bit as f32 * scale;
                        let py0 = pen_y + row as f32 * scale;
                        let px1 = px0 + scale;
                        let py1 = py0 + scale;
                        push_quad_pixels(&mut self.scratch, (px0, py0, px1, py1), w, h, rgb);
                    }
                }
            }
            pen_x += 8.0 * scale + gap_px;
        }

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
