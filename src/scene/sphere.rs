//! Сфера: UV-сетка в локальных координатах, центр в начале координат.

use std::f32::consts::PI;

fn push_vertex(v: &mut Vec<f32>, p: [f32; 3], c: [f32; 3], n: [f32; 3]) {
    v.extend_from_slice(&[
        p[0], p[1], p[2], //
        c[0], c[1], c[2], //
        n[0], n[1], n[2],
    ]);
}

/// `latitude_segments` ≥ 2, `longitude_segments` ≥ 3.
/// Интерливинг `xyz` + `rgb` + `nxnynz`; нормали — сглаженные (от центра сферы).
pub fn build_sphere_vertex_data(
    radius: f32,
    latitude_segments: u32,
    longitude_segments: u32,
) -> Vec<f32> {
    let lat = latitude_segments.max(2);
    let lon = longitude_segments.max(3);
    let white = [1.0_f32, 1.0, 1.0];
    let est = (lat as usize * lon as usize * 6) * 9;
    let mut v = Vec::with_capacity(est);

    for i in 0..lat {
        let v0 = i as f32 / lat as f32;
        let v1 = (i + 1) as f32 / lat as f32;
        let lat0 = (v0 - 0.5) * PI;
        let lat1 = (v1 - 0.5) * PI;
        let (sin_lat0, cos_lat0) = lat0.sin_cos();
        let (sin_lat1, cos_lat1) = lat1.sin_cos();

        for j in 0..lon {
            let u0 = j as f32 / lon as f32;
            let u1 = (j + 1) as f32 / lon as f32;
            let lon0 = u0 * PI * 2.0;
            let lon1 = u1 * PI * 2.0;
            let (sin_lon0, cos_lon0) = lon0.sin_cos();
            let (sin_lon1, cos_lon1) = lon1.sin_cos();

            let p00 = [
                radius * cos_lat0 * cos_lon0,
                radius * sin_lat0,
                radius * cos_lat0 * sin_lon0,
            ];
            let p10 = [
                radius * cos_lat0 * cos_lon1,
                radius * sin_lat0,
                radius * cos_lat0 * sin_lon1,
            ];
            let p11 = [
                radius * cos_lat1 * cos_lon1,
                radius * sin_lat1,
                radius * cos_lat1 * sin_lon1,
            ];
            let p01 = [
                radius * cos_lat1 * cos_lon0,
                radius * sin_lat1,
                radius * cos_lat1 * sin_lon0,
            ];

            let n00 = {
                let m = (p00[0] * p00[0] + p00[1] * p00[1] + p00[2] * p00[2]).sqrt();
                [p00[0] / m, p00[1] / m, p00[2] / m]
            };
            let n10 = {
                let m = (p10[0] * p10[0] + p10[1] * p10[1] + p10[2] * p10[2]).sqrt();
                [p10[0] / m, p10[1] / m, p10[2] / m]
            };
            let n11 = {
                let m = (p11[0] * p11[0] + p11[1] * p11[1] + p11[2] * p11[2]).sqrt();
                [p11[0] / m, p11[1] / m, p11[2] / m]
            };
            let n01 = {
                let m = (p01[0] * p01[0] + p01[1] * p01[1] + p01[2] * p01[2]).sqrt();
                [p01[0] / m, p01[1] / m, p01[2] / m]
            };

            push_vertex(&mut v, p00, white, n00);
            push_vertex(&mut v, p10, white, n10);
            push_vertex(&mut v, p11, white, n11);

            push_vertex(&mut v, p00, white, n00);
            push_vertex(&mut v, p11, white, n11);
            push_vertex(&mut v, p01, white, n01);
        }
    }

    debug_assert_eq!(v.len() % 9, 0);
    v
}
