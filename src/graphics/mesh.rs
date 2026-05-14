//! VAO/VBO и отрисовка примитивов: треугольники (`GL_TRIANGLES`) и линии (`GL_LINES`).

use std::ffi::c_void;

/// Как интерпретировать вершины при вызове [`Mesh::draw`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshTopology {
    Triangles,
    Lines,
}

/// Интерливинг `vec3 позиция` + `vec3 цвет` на вершину, два атрибута с индексами 0 и 1.
///
/// Для освещённых треугольников см. [`Mesh::new_interleaved_pos3_color3_normal3`] (атрибут 2 — нормаль).
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

    /// `vertices` — `[x,y,z, r,g,b, nx,ny,nz, ...]`; `vertex_count` — число вершин.
    pub fn new_interleaved_pos3_color3_normal3(vertices: &[f32], vertex_count: usize) -> Self {
        debug_assert_eq!(
            vertices.len(),
            vertex_count * 9,
            "ожидается 9 float на вершину (xyz + rgb + normal)"
        );

        let stride = (9 * std::mem::size_of::<f32>()) as i32;

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

            let normal_offset = (6 * std::mem::size_of::<f32>()) as *const c_void;
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, normal_offset);
            gl::EnableVertexAttribArray(2);

            Self {
                vao,
                _vbo: vbo,
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

    /// Рисует треугольники; шейдер и uniform должны быть уже настроены вызывающим кодом.
    pub fn draw_triangles(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }

    /// Рисует отрезки линий; число вершин должно быть **чётным** (каждая пара — один отрезок).
    pub fn draw_lines(&self) {
        debug_assert_eq!(
            self.vertex_count % 2,
            0,
            "GL_LINES: нужно чётное число вершин"
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
    }
}
