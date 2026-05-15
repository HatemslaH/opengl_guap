use cgmath::Vector3;

use crate::engine::graphics::Color;

/// Type of light source; later you can add `Spot`, `Area` and etc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightKind {
    /// Parallel rays: direction **to** the source in world coordinates (where the hemisphere «lights»).
    /// The entity position is not used by the renderer.
    Directional { toward_light: Vector3<f32> },
    /// Point in space: the world position is taken from [`Position`] on the same entity.
    Point {
        constant: f32,
        linear: f32,
        quadratic: f32,
    },
}

impl LightKind {
    /// Directional light with a normalized direction to the source.
    pub fn directional_toward_light(direction: Vector3<f32>) -> Self {
        Self::Directional {
            toward_light: Self::normalize_or_fallback(direction, Vector3::new(0.0, 1.0, 0.0)),
        }
    }

    /// Point light with typical distance attenuation.
    pub fn point_default_attenuation() -> Self {
        Self::Point {
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }
    }

    fn normalize_or_fallback(v: Vector3<f32>, fallback: Vector3<f32>) -> Vector3<f32> {
        let n = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        if n > 1e-6 { v / n } else { fallback }
    }
}

/// Light source on the scene: type ([`LightKind`]), color and overall brightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub kind: LightKind,
    pub color: Color,
    /// Multiplier of intensity on top of `color` (1.0 — «as is»).
    pub intensity: f32,
}

impl Light {
    pub fn new(kind: LightKind, color: Color, intensity: f32) -> Self {
        Self {
            kind,
            color,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional_toward_light_is_unit() {
        let k = LightKind::directional_toward_light(Vector3::new(3.0, 0.0, 4.0));
        let LightKind::Directional { toward_light } = k else {
            panic!("expected directional");
        };
        let n = (toward_light.x * toward_light.x
            + toward_light.y * toward_light.y
            + toward_light.z * toward_light.z)
            .sqrt();
        assert!((n - 1.0).abs() < 1e-5);
    }
}
