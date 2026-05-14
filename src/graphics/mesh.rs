//! VAO/VBO и отрисовка массива треугольников (`GL_TRIANGLES`).

use std::ffi::c_void;

/// Интерливинг `vec3 позиция` + `vec3 цвет` на вершину, два атрибута с индексами 0 и 1.
pub struct Mesh {
    vao: u32,
    _vbo: u32,
    vertex_count: i32,
}

impl Mesh {
    /// `vertices` — последовательность `[x,y,z,r,g,b, ...]`; `vertex_count` — число вершин (не число float).
    pub fn new_interleaved_pos3_color3(vertices: &[f32], vertex_count: usize) -> Self {
        debug_assert_eq!(
            vertices.len(),
            vertex_count * 6,
            "ожидается 6 float на вершину (xyz + rgb)"
        );

        let stride = (6 * std::mem::size_of::<f32>()) as i32;

        unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

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

            Self {
                vao,
                _vbo: vbo,
                vertex_count: vertex_count as i32,
            }
        }
    }

    /// Рисует треугольники; шейдер и uniform должны быть уже настроены вызывающим кодом.
    pub fn draw_triangles(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
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
    }
}
