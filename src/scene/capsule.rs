//! Капсула вдоль Y: нижняя полусфера, цилиндрический ствол, верхняя полусфера.

use std::f32::consts::FRAC_PI_2;

use super::cylinder::build_cylinder_vertex_data;

fn append_hemisphere_y(
    out: &mut Vec<f32>,
    radius: f32,
    center_y: f32,
    latitude_rings: u32,
    longitude_segments: u32,
    bottom: bool,
) {
    let lat = latitude_rings.max(1);
    let lon = longitude_segments.max(3);
    let white = [1.0_f32, 1.0, 1.0];

    for i in 0..lat {
        let v0 = i as f32 / lat as f32;
        let v1 = (i + 1) as f32 / lat as f32;
        let (lat0, lat1) = if bottom {
            // От южного полюса (-π/2) к экватору (0)
            (-FRAC_PI_2 + v0 * FRAC_PI_2, -FRAC_PI_2 + v1 * FRAC_PI_2)
        } else {
            // От экватора (0) к северному полюсу (+π/2)
            (v0 * FRAC_PI_2, v1 * FRAC_PI_2)
        };
        let (sin_lat0, cos_lat0) = lat0.sin_cos();
        let (sin_lat1, cos_lat1) = lat1.sin_cos();

        for j in 0..lon {
            let u0 = j as f32 / lon as f32;
            let u1 = (j + 1) as f32 / lon as f32;
            let lon0 = u0 * std::f32::consts::TAU;
            let lon1 = u1 * std::f32::consts::TAU;
            let (sin_lon0, cos_lon0) = lon0.sin_cos();
            let (sin_lon1, cos_lon1) = lon1.sin_cos();

            let local00 = [
                radius * cos_lat0 * cos_lon0,
                radius * sin_lat0,
                radius * cos_lat0 * sin_lon0,
            ];
            let local10 = [
                radius * cos_lat0 * cos_lon1,
                radius * sin_lat0,
                radius * cos_lat0 * sin_lon1,
            ];
            let local11 = [
                radius * cos_lat1 * cos_lon1,
                radius * sin_lat1,
                radius * cos_lat1 * sin_lon1,
            ];
            let local01 = [
                radius * cos_lat1 * cos_lon0,
                radius * sin_lat1,
                radius * cos_lat1 * sin_lon0,
            ];

            let p00 = [local00[0], local00[1] + center_y, local00[2]];
            let p10 = [local10[0], local10[1] + center_y, local10[2]];
            let p11 = [local11[0], local11[1] + center_y, local11[2]];
            let p01 = [local01[0], local01[1] + center_y, local01[2]];

            let n00 = norm(local00);
            let n10 = norm(local10);
            let n11 = norm(local11);
            let n01 = norm(local01);

            if bottom {
                push_lit_tri(
                    out,
                    lit9(p00, white, n00),
                    lit9(p10, white, n10),
                    lit9(p11, white, n11),
                );
                push_lit_tri(
                    out,
                    lit9(p00, white, n00),
                    lit9(p11, white, n11),
                    lit9(p01, white, n01),
                );
            } else {
                push_lit_tri(
                    out,
                    lit9(p00, white, n00),
                    lit9(p11, white, n11),
                    lit9(p10, white, n10),
                );
                push_lit_tri(
                    out,
                    lit9(p00, white, n00),
                    lit9(p01, white, n01),
                    lit9(p11, white, n11),
                );
            }
        }
    }
}

#[inline]
fn norm(p: [f32; 3]) -> [f32; 3] {
    let m = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    [p[0] / m, p[1] / m, p[2] / m]
}

#[inline]
fn lit9(p: [f32; 3], c: [f32; 3], n: [f32; 3]) -> [f32; 9] {
    [
        p[0], p[1], p[2], //
        c[0], c[1], c[2], //
        n[0], n[1], n[2],
    ]
}

#[inline]
fn push_lit_tri(v: &mut Vec<f32>, a: [f32; 9], b: [f32; 9], c: [f32; 9]) {
    v.extend_from_slice(&a);
    v.extend_from_slice(&b);
    v.extend_from_slice(&c);
}

/// `cylinder_half_height` — половина длины прямого участка (между экваторами сфер).
/// Полные полусферы радиуса `radius` центрированы в `y = ±cylinder_half_height`.
pub fn build_capsule_vertex_data(
    radius: f32,
    cylinder_half_height: f32,
    hemisphere_latitude_rings: u32,
    longitude_segments: u32,
    cylinder_sectors: u32,
) -> Vec<f32> {
    let mut v = Vec::new();
    append_hemisphere_y(
        &mut v,
        radius,
        -cylinder_half_height,
        hemisphere_latitude_rings,
        longitude_segments,
        true,
    );
    v.extend_from_slice(&build_cylinder_vertex_data(
        radius,
        cylinder_half_height,
        cylinder_sectors,
        false,
    ));
    append_hemisphere_y(
        &mut v,
        radius,
        cylinder_half_height,
        hemisphere_latitude_rings,
        longitude_segments,
        false,
    );
    debug_assert_eq!(v.len() % 9, 0);
    v
}
