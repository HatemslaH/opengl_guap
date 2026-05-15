//! Цилиндр вдоль оси Y: дно, бок, крышка (локально центр на середине высоты).

use std::f32::consts::PI;

fn push_vertex(v: &mut Vec<f32>, p: [f32; 3], c: [f32; 3], n: [f32; 3]) {
    v.extend_from_slice(&[
        p[0], p[1], p[2], //
        c[0], c[1], c[2], //
        n[0], n[1], n[2],
    ]);
}

/// От `y = -half_height` до `y = +half_height`, радиус `radius`. `sectors` ≥ 3.
///
/// `include_caps` — плоские диски сверху и снизу (для капсулы обычно `false`, стык с полусферами).
pub fn build_cylinder_vertex_data(
    radius: f32,
    half_height: f32,
    sectors: u32,
    include_caps: bool,
) -> Vec<f32> {
    let nsec = sectors.max(3);
    let white = [1.0_f32, 1.0, 1.0];
    let cap_tris = if include_caps { nsec as usize * 2 } else { 0 };
    let mut v = Vec::with_capacity((nsec as usize * 2 + cap_tris) * 9);

    // Боковая поверхность
    for j in 0..nsec {
        let u0 = j as f32 / nsec as f32;
        let u1 = (j + 1) as f32 / nsec as f32;
        let lon0 = u0 * PI * 2.0;
        let lon1 = u1 * PI * 2.0;
        let (s0, c0) = lon0.sin_cos();
        let (s1, c1) = lon1.sin_cos();
        let nx0 = c0;
        let nz0 = s0;
        let nx1 = c1;
        let nz1 = s1;
        let bl = [radius * c0, -half_height, radius * s0];
        let br = [radius * c1, -half_height, radius * s1];
        let tr = [radius * c1, half_height, radius * s1];
        let tl = [radius * c0, half_height, radius * s0];
        let n0 = [nx0, 0.0, nz0];
        let n1 = [nx1, 0.0, nz1];
        push_vertex(&mut v, bl, white, n0);
        push_vertex(&mut v, br, white, n1);
        push_vertex(&mut v, tr, white, n1);
        push_vertex(&mut v, bl, white, n0);
        push_vertex(&mut v, tr, white, n1);
        push_vertex(&mut v, tl, white, n0);
    }

    if include_caps {
        // Нижняя крышка (y = -half_height), нормаль -Y
        let n_bot = [0.0_f32, -1.0, 0.0];
        let center_bot = [0.0_f32, -half_height, 0.0];
        for j in 0..nsec {
            let u0 = j as f32 / nsec as f32;
            let u1 = (j + 1) as f32 / nsec as f32;
            let lon0 = u0 * PI * 2.0;
            let lon1 = u1 * PI * 2.0;
            let (s0, c0) = lon0.sin_cos();
            let (s1, c1) = lon1.sin_cos();
            let p0 = [radius * c0, -half_height, radius * s0];
            let p1 = [radius * c1, -half_height, radius * s1];
            push_vertex(&mut v, center_bot, white, n_bot);
            push_vertex(&mut v, p1, white, n_bot);
            push_vertex(&mut v, p0, white, n_bot);
        }

        // Верхняя крышка (y = +half_height), нормаль +Y
        let n_top = [0.0_f32, 1.0, 0.0];
        let center_top = [0.0_f32, half_height, 0.0];
        for j in 0..nsec {
            let u0 = j as f32 / nsec as f32;
            let u1 = (j + 1) as f32 / nsec as f32;
            let lon0 = u0 * PI * 2.0;
            let lon1 = u1 * PI * 2.0;
            let (s0, c0) = lon0.sin_cos();
            let (s1, c1) = lon1.sin_cos();
            let p0 = [radius * c0, half_height, radius * s0];
            let p1 = [radius * c1, half_height, radius * s1];
            push_vertex(&mut v, center_top, white, n_top);
            push_vertex(&mut v, p0, white, n_top);
            push_vertex(&mut v, p1, white, n_top);
        }
    }

    debug_assert_eq!(v.len() % 9, 0);
    v
}
