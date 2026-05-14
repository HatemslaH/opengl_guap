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
    void main()
    {
        FragColor = vec4(vColor, 1.0);
    }
"#;

/// Связанная программа OpenGL и известные uniform-локации.
pub struct ShaderProgram {
    id: u32,
    mvp_location: i32,
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
            let mvp_location = gl::GetUniformLocation(id, mvp_name.as_ptr());

            Self { id, mvp_location }
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
