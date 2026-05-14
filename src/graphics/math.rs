//! Математика кадра: VP, модельная матрица, упаковка `mat4` для OpenGL.

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

/// Матрица `proj * view` (камера не «вшита» в модель — вращение задаётся на сущности).
pub fn view_projection_matrix(aspect: f32) -> Matrix4<f32> {
    camera_view_projection_matrix(
        Vector3::new(0.0, 0.0, 2.8),
        0.0,
        0.0,
        45.0,
        aspect,
        0.1,
        100.0,
    )
}

/// Направление взгляда в мировых координатах: `yaw` / `pitch` в радианах (как у FPS-камеры).
fn camera_forward_world(yaw_rad: f32, pitch_rad: f32) -> Vector3<f32> {
    Vector3::new(
        yaw_rad.sin() * pitch_rad.cos(),
        pitch_rad.sin(),
        -yaw_rad.cos() * pitch_rad.cos(),
    )
}

/// Матрица вида: позиция `eye`, ориентация через `yaw`/`pitch`, мир праворукий.
pub fn camera_view_matrix(eye: Vector3<f32>, yaw_rad: f32, pitch_rad: f32) -> Matrix4<f32> {
    let f = camera_forward_world(yaw_rad, pitch_rad);
    let target = Point3::new(eye.x + f.x, eye.y + f.y, eye.z + f.z);
    Matrix4::look_at_rh(
        Point3::new(eye.x, eye.y, eye.z),
        target,
        Vector3::new(0.0, 1.0, 0.0),
    )
}

/// Углы камеры в градусах из вектора «куда смотреть» в мировых осях (то же соглашение yaw/pitch, что у [`camera_view_matrix`]).
///
/// Возвращает [`None`], если направление нулевое или почти вертикально вверх/вниз (неоднозначный `yaw`).
pub fn camera_yaw_pitch_deg_from_look_direction(dir: Vector3<f32>) -> Option<(f32, f32)> {
    let len_sq = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z;
    if len_sq < 1e-12 {
        return None;
    }
    let d = dir / len_sq.sqrt();
    let pitch_rad = d.y.clamp(-1.0, 1.0).asin();
    let cos_pitch = pitch_rad.cos();
    if cos_pitch.abs() < 1e-5 {
        return None;
    }
    let yaw_rad = d.x.atan2(-d.z);
    Some((yaw_rad.to_degrees(), pitch_rad.to_degrees()))
}

/// Полная матрица `proj * view` для камеры с заданным FOV и плоскостей отсечения.
pub fn camera_view_projection_matrix(
    eye: Vector3<f32>,
    yaw_rad: f32,
    pitch_rad: f32,
    fovy_deg: f32,
    aspect: f32,
    z_near: f32,
    z_far: f32,
) -> Matrix4<f32> {
    let view = camera_view_matrix(eye, yaw_rad, pitch_rad);
    let proj = perspective(Deg(fovy_deg), aspect, z_near, z_far);
    proj * view
}

/// Локальная модель: перенос + поворот вокруг Y, затем X (как в старой анимации куба).
pub fn model_matrix(translation: Vector3<f32>, yaw_rad: f32, pitch_rad: f32) -> Matrix4<f32> {
    Matrix4::from_translation(translation)
        * Matrix4::from_angle_y(Rad(yaw_rad))
        * Matrix4::from_angle_x(Rad(pitch_rad))
}
