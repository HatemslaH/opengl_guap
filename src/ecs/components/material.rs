use crate::graphics::Color;

/// Reflection parameters (Blinn–Phong) on top of the base [`Material::color`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceLighting {
    /// Contribution of ambient lighting on albedo (scales the material color).
    pub ambient: f32,
    /// Contribution of diffuse component.
    pub diffuse: f32,
    /// Color of the specular highlight (usually white for dielectrics, colored for metals — simplified).
    pub specular_color: Color,
    /// Shininess factor (the higher the value, the narrower and brighter the highlight).
    pub shininess: f32,
}

impl Default for SurfaceLighting {
    fn default() -> Self {
        Self {
            ambient: 0.15,
            diffuse: 1.0,
            specular_color: Color::new(1.0, 1.0, 1.0),
            shininess: 48.0,
        }
    }
}

/// Material for meshes with triangles: color, opacity and lighting parameters ([`SurfaceLighting`]).
///
/// Without this component **triangles** are not rendered (the cube is «transparent» / absent in the frame).
/// Lines (grid) still use only the color of the vertices — the material is not needed for them.
///
/// When `opacity >= 1.0` the object goes into an opaque pass (without `GL_BLEND`, with writing to the Z-buffer).
/// When `0.0 < opacity < 1.0` — a separate pass with mixing (more expensive by the pipeline state).
///
/// [Note: when writing to the Z-buffer, the opaque pass is more efficient than the transparent pass,
/// because it avoids the overhead of blending and depth testing.]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    /// 1.0 — fully opaque; less — alpha blending.
    pub opacity: f32,
    pub surface: SurfaceLighting,
}

impl Material {
    pub fn new(color: Color, opacity: f32) -> Self {
        Self {
            color,
            opacity,
            surface: SurfaceLighting::default(),
        }
    }

    pub fn opaque(color: Color) -> Self {
        Self {
            color,
            opacity: 1.0,
            surface: SurfaceLighting::default(),
        }
    }

    pub fn with_surface(mut self, surface: SurfaceLighting) -> Self {
        self.surface = surface;
        self
    }

    #[inline]
    pub fn is_fully_opaque(self) -> bool {
        self.opacity >= 1.0 - 1e-5
    }

    #[inline]
    pub fn has_transparency(self) -> bool {
        !self.is_fully_opaque()
    }

    #[inline]
    pub fn is_visible(self) -> bool {
        self.opacity > 1e-5
    }
}
