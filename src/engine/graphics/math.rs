//! Frame math: VP, model matrix, packing `mat4` for OpenGL.

use cgmath::{Deg, Matrix, Matrix3, Matrix4, Point3, Rad, SquareMatrix, Vector3, perspective};

/// Converts `cgmath::Matrix4` to 16 `f32` in **column-major** order for `glUniformMatrix4fv` / GLSL `mat4`.
/// `mat3` for `glUniformMatrix3fv` / GLSL `mat3` (columns).
pub fn matrix3_column_major(m: &Matrix3<f32>) -> [f32; 9] {
    [
        m.x.x, m.x.y, m.x.z, //
        m.y.x, m.y.y, m.y.z, //
        m.z.x, m.z.y, m.z.z, //
    ]
}

/// Normal matrix: transposed inverse of the top left `3×3` of `model`
/// (for uniform scale and pure rotation coincides with the linear part of `model`).
pub fn normal_matrix3_from_model(model: &Matrix4<f32>) -> Matrix3<f32> {
    let m = Matrix3::new(
        model.x.x, model.x.y, model.x.z, //
        model.y.x, model.y.y, model.y.z, //
        model.z.x, model.z.y, model.z.z, //
    );
    m.invert()
        .map(|inv| inv.transpose())
        .unwrap_or_else(Matrix3::identity)
}

pub fn matrix4_column_major(m: &Matrix4<f32>) -> [f32; 16] {
    [
        m.x.x, m.x.y, m.x.z, m.x.w, //
        m.y.x, m.y.y, m.y.z, m.y.w, //
        m.z.x, m.z.y, m.z.z, m.z.w, //
        m.w.x, m.w.y, m.w.z, m.w.w, //
    ]
}

/// Matrix `proj * view` (the camera is not «embedded» in the model — the rotation is set on the entity).
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

/// The look direction in world coordinates: `yaw` / `pitch` in radians (like the FPS camera).
fn camera_forward_world(yaw_rad: f32, pitch_rad: f32) -> Vector3<f32> {
    Vector3::new(
        yaw_rad.sin() * pitch_rad.cos(),
        pitch_rad.sin(),
        -yaw_rad.cos() * pitch_rad.cos(),
    )
}

/// View matrix: position `eye`, orientation through `yaw`/`pitch`, world right-handed.
pub fn camera_view_matrix(eye: Vector3<f32>, yaw_rad: f32, pitch_rad: f32) -> Matrix4<f32> {
    let f = camera_forward_world(yaw_rad, pitch_rad);
    let target = Point3::new(eye.x + f.x, eye.y + f.y, eye.z + f.z);
    Matrix4::look_at_rh(
        Point3::new(eye.x, eye.y, eye.z),
        target,
        Vector3::new(0.0, 1.0, 0.0),
    )
}

/// Camera angles in degrees from the vector «looking at» in world axes (the same yaw/pitch convention as [`camera_view_matrix`]).
///
/// Returns [`None`], if the direction is zero or almost vertical up/down (ambiguous `yaw`).
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

/// Full matrix `proj * view` for a camera with a given FOV and clipping planes.
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

/// The eye position on the sphere around `target`: looking at the target with the angles [`camera_view_matrix`] (`yaw` / `pitch` in **degrees**) at a distance `distance`.
#[inline]
pub fn camera_eye_for_look_at_target(
    target: Vector3<f32>,
    distance: f32,
    yaw_deg: f32,
    pitch_deg: f32,
) -> Vector3<f32> {
    let f = camera_forward_world(yaw_deg.to_radians(), pitch_deg.to_radians());
    target - f * distance
}

/// Local model: translation + rotation around Y, X, Z (angles `rotation_deg` in **degrees**: `.y`, `.x`, `.z`), then scale.
pub fn model_matrix(
    translation: Vector3<f32>,
    rotation_deg: Vector3<f32>,
    scale: Vector3<f32>,
) -> Matrix4<f32> {
    let x = rotation_deg.x.to_radians();
    let y = rotation_deg.y.to_radians();
    let z = rotation_deg.z.to_radians();
    Matrix4::from_translation(translation)
        * Matrix4::from_angle_y(Rad(y))
        * Matrix4::from_angle_x(Rad(x))
        * Matrix4::from_angle_z(Rad(z))
        * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z)
}
