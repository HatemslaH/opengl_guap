//! Математика кадра: MVP и упаковка матрицы для OpenGL (столбцовый порядок).

use cgmath::{Deg, Matrix4, Point3, Rad, Vector3, perspective};

/// Преобразует `cgmath::Matrix4` в 16 `f32` в **столбцовом** порядке для `glUniformMatrix4fv` / GLSL `mat4`.
pub fn matrix4_column_major(m: &Matrix4<f32>) -> [f32; 16] {
    [
        m.x.x, m.x.y, m.x.z, m.x.w, //
        m.y.x, m.y.y, m.y.z, m.y.w, //
        m.z.x, m.z.y, m.z.z, m.z.w, //
        m.w.x, m.w.y, m.w.z, m.w.w, //
    ]
}

/// Собирает матрицу `proj * view * model` для вращающегося куба в центре сцены (как в исходном примере).
pub fn mvp_matrix(elapsed_secs: f32, aspect: f32) -> Matrix4<f32> {
    let model = Matrix4::from_angle_y(Rad(elapsed_secs * 0.7))
        * Matrix4::from_angle_x(Rad(elapsed_secs * 0.35));
    let view = Matrix4::look_at_rh(
        Point3::new(0.0, 0.0, 2.8),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let proj = perspective(Deg(45.0), aspect, 0.1, 100.0);
    proj * view * model
}
