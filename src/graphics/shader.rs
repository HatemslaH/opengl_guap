//! Компиляция и линковка GLSL, работа с uniform.

use crate::graphics::math::matrix4_column_major;
use cgmath::Matrix4;
use std::ffi::CString;

const VERT_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec3 vColor;
    uniform mat4 uMVP;
    void main()
    {
        gl_Position = uMVP * vec4(aPos, 1.0);
        vColor = aColor;
    }
"#;

const FRAG_SRC: &str = r#"
    #version 330 core
    in vec3 vColor;
    out vec4 FragColor;
    uniform vec3 uMatRgb;
    uniform float uMatAlpha;
    // 1 — цвет из вершины (сетка); 0 — из uniform материала.
    uniform int uUseVertexColor;
    void main()
    {
        if (uUseVertexColor != 0) {
            FragColor = vec4(vColor, 1.0);
        } else {
            FragColor = vec4(uMatRgb, uMatAlpha);
        }
    }
"#;

/// Связанная программа OpenGL и известные uniform-локации.
pub struct ShaderProgram {
    id: u32,
    mvp_location: i32,
    mat_rgb_location: i32,
    mat_alpha_location: i32,
    use_vertex_color_location: i32,
}

impl ShaderProgram {
    /// Создаёт программу с парой шейдеров позиция+цвет и uniform `uMVP`.
    pub fn new_colored_mesh() -> Self {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let src = CString::new(VERT_SRC).expect("исходник вершинного шейдера без NUL");
            gl::ShaderSource(vs, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            let src = CString::new(FRAG_SRC).expect("исходник фрагментного шейдера без NUL");
            gl::ShaderSource(fs, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(fs);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vs);
            gl::AttachShader(id, fs);
            gl::LinkProgram(id);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            let mvp_name = CString::new("uMVP").expect("имя uniform без NUL");
            let mat_rgb_name = CString::new("uMatRgb").expect("имя uniform без NUL");
            let mat_alpha_name = CString::new("uMatAlpha").expect("имя uniform без NUL");
            let use_vertex_color_name = CString::new("uUseVertexColor").expect("имя uniform без NUL");
            let mvp_location = gl::GetUniformLocation(id, mvp_name.as_ptr());
            let mat_rgb_location = gl::GetUniformLocation(id, mat_rgb_name.as_ptr());
            let mat_alpha_location = gl::GetUniformLocation(id, mat_alpha_name.as_ptr());
            let use_vertex_color_location = gl::GetUniformLocation(id, use_vertex_color_name.as_ptr());

            Self {
                id,
                mvp_location,
                mat_rgb_location,
                mat_alpha_location,
                use_vertex_color_location,
            }
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn mvp_location(&self) -> i32 {
        self.mvp_location
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Записывает `mat4 uMVP` (столбцы, как в GLSL).
    pub fn set_mvp(&self, mvp: &Matrix4<f32>) {
        let cols = matrix4_column_major(mvp);
        unsafe {
            gl::UniformMatrix4fv(self.mvp_location, 1, gl::FALSE, cols.as_ptr());
        }
    }

    /// `true` — фрагментный цвет из атрибута вершины (линии сетки); `false` — из [`Self::set_material_rgba`].
    pub fn set_vertex_color_mode(&self, use_vertex_color: bool) {
        unsafe {
            gl::Uniform1i(
                self.use_vertex_color_location,
                if use_vertex_color { 1 } else { 0 },
            );
        }
    }

    /// RGB и альфа материала (в шейдере умножение с вершинным цветом не делается — только uniform).
    pub fn set_material_rgba(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Uniform3f(self.mat_rgb_location, r, g, b);
            gl::Uniform1f(self.mat_alpha_location, a);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}
