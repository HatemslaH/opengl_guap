//! VAO/VBO and drawing primitives: triangles (`GL_TRIANGLES`) and lines (`GL_LINES`).

use std::ffi::c_void;

use cgmath::{Matrix3, Matrix4};

use crate::engine::graphics::math::normal_matrix3_from_model;

/// How to interpret vertices when calling [`Mesh::draw`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshTopology {
    Triangles,
    Lines,
}

/// Per-instance model and normal matrix (column-major, matches vertex attributes 3–9).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct InstanceData {
    pub model: Matrix4<f32>,
    pub normal_mat: Matrix3<f32>,
}

impl InstanceData {
    #[inline]
    pub fn from_world_model(model: Matrix4<f32>) -> Self {
        let normal_mat = normal_matrix3_from_model(&model);
        Self {
            model,
            normal_mat,
        }
    }
}

/// Interleaving `vec3 position` + `vec3 color` per vertex, two attributes with indices 0 and 1.
///
/// For lighted triangles see [`Mesh::new_interleaved_pos3_color3_normal3`] (attribute 2 — normal).
/// Instance attributes use locations 3–9 ([`InstanceData`]) with divisor 1.
pub struct Mesh {
    vao: u32,
    _vbo: u32,
    _instance_vbo: u32,
    vertex_count: i32,
}

fn setup_instance_vertex_attribs(stride: i32) {
    let model_offset = 0usize;
    let normal_offset = std::mem::size_of::<Matrix4<f32>>();

    unsafe {
        for col in 0..4u32 {
            let loc = 3 + col;
            let offset = (model_offset + col as usize * std::mem::size_of::<cgmath::Vector4<f32>>())
                as *const c_void;
            gl::EnableVertexAttribArray(loc);
            gl::VertexAttribPointer(loc, 4, gl::FLOAT, gl::FALSE, stride, offset);
            gl::VertexAttribDivisor(loc, 1);
        }
        for col in 0..3u32 {
            let loc = 7 + col;
            let offset = (normal_offset + col as usize * std::mem::size_of::<cgmath::Vector3<f32>>())
                as *const c_void;
            gl::EnableVertexAttribArray(loc);
            gl::VertexAttribPointer(loc, 3, gl::FLOAT, gl::FALSE, stride, offset);
            gl::VertexAttribDivisor(loc, 1);
        }
    }
}

impl Mesh {
    /// `vertices` — sequence `[x,y,z,r,g,b, ...]`; `vertex_count` — number of vertices (not number of float).
    pub fn new_interleaved_pos3_color3(vertices: &[f32], vertex_count: usize) -> Self {
        debug_assert_eq!(
            vertices.len(),
            vertex_count * 6,
            "expected 6 float per vertex (xyz + rgb)"
        );

        let stride = (6 * std::mem::size_of::<f32>()) as i32;
        let instance_stride = std::mem::size_of::<InstanceData>() as i32;

        unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            let mut instance_vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut instance_vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            let color_offset = (3 * std::mem::size_of::<f32>()) as *const c_void;
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, color_offset);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribDivisor(0, 0);
            gl::VertexAttribDivisor(1, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::STREAM_DRAW);
            setup_instance_vertex_attribs(instance_stride);

            Self {
                vao,
                _vbo: vbo,
                _instance_vbo: instance_vbo,
                vertex_count: vertex_count as i32,
            }
        }
    }

    /// `vertices` — `[x,y,z, r,g,b, nx,ny,nz, ...]`; `vertex_count` — number of vertices.
    pub fn new_interleaved_pos3_color3_normal3(vertices: &[f32], vertex_count: usize) -> Self {
        debug_assert_eq!(
            vertices.len(),
            vertex_count * 9,
            "expected 9 float per vertex (xyz + rgb + normal)"
        );

        let stride = (9 * std::mem::size_of::<f32>()) as i32;
        let instance_stride = std::mem::size_of::<InstanceData>() as i32;

        unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            let mut instance_vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut instance_vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            let color_offset = (3 * std::mem::size_of::<f32>()) as *const c_void;
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, color_offset);
            gl::EnableVertexAttribArray(1);

            let normal_offset = (6 * std::mem::size_of::<f32>()) as *const c_void;
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, normal_offset);
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribDivisor(0, 0);
            gl::VertexAttribDivisor(1, 0);
            gl::VertexAttribDivisor(2, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::STREAM_DRAW);
            setup_instance_vertex_attribs(instance_stride);

            Self {
                vao,
                _vbo: vbo,
                _instance_vbo: instance_vbo,
                vertex_count: vertex_count as i32,
            }
        }
    }

    pub fn draw(&self, topology: MeshTopology) {
        match topology {
            MeshTopology::Triangles => self.draw_triangles(),
            MeshTopology::Lines => self.draw_lines(),
        }
    }

    /// Instanced draw; uploads instance buffer then calls `glDrawArraysInstanced`.
    pub fn draw_instanced(&self, topology: MeshTopology, instances: &[InstanceData]) {
        if instances.is_empty() {
            return;
        }
        let mode = match topology {
            MeshTopology::Triangles => gl::TRIANGLES,
            MeshTopology::Lines => gl::LINES,
        };
        if topology == MeshTopology::Lines {
            debug_assert_eq!(
                self.vertex_count % 2,
                0,
                "GL_LINES: even number of vertices required"
            );
        }
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self._instance_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(instances) as isize,
                instances.as_ptr().cast(),
                gl::STREAM_DRAW,
            );
            gl::DrawArraysInstanced(mode, 0, self.vertex_count, instances.len() as i32);
        }
    }

    /// Draws triangles; the shader and uniform must already be set by the calling code.
    pub fn draw_triangles(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }

    /// Draws lines; the number of vertices must be **even** (each pair — one line).
    pub fn draw_lines(&self) {
        debug_assert_eq!(
            self.vertex_count % 2,
            0,
            "GL_LINES: even number of vertices required"
        );
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::LINES, 0, self.vertex_count);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        if self.vao != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
        if self._vbo != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self._vbo);
            }
        }
        if self._instance_vbo != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self._instance_vbo);
            }
        }
    }
}
