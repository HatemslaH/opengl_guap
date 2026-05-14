//! Компоненты ECS: положение, опциональное вращение, GPU-меш.

use crate::graphics::{Mesh, MeshTopology};
use cgmath::Vector3;
use hecs::Entity;

/// Положение сущности в мире (центр модели в мировых координатах).
#[derive(Clone, Debug)]
pub struct Position {
    pub position: Vector3<f32>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

/// Вращение вокруг Y и X с интеграцией фаз; можно **отключить** (`enabled = false`) — тогда модель не крутится.
///
/// Подключайте к любой сущности с [`RenderMesh`]: система [`crate::ecs::systems::spin_animation_system`] обновляет только сущности с этим компонентом.
#[derive(Clone, Debug)]
pub struct SpinAnimation {
    pub enabled: bool,
    /// Рад/сек вокруг мировой оси Y.
    pub velocity_y: f32,
    /// Рад/сек вокруг локальной X после поворота Y (как в старом демо).
    pub velocity_x: f32,
    pub phase_y: f32,
    pub phase_x: f32,
}

impl SpinAnimation {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            velocity_y: 0.0,
            velocity_x: 0.0,
            phase_y: 0.0,
            phase_x: 0.0,
        }
    }

    /// То же вращение, что было «зашито» в MVP раньше.
    pub fn demo_orbit() -> Self {
        Self {
            enabled: true,
            velocity_y: 0.7,
            velocity_x: 0.35,
            phase_y: 0.0,
            phase_x: 0.0,
        }
    }
}

/// Ссылка на загруженный в GPU меш и тип примитива.
pub struct RenderMesh {
    pub mesh: Mesh,
    pub topology: MeshTopology,
}

/// RGB в **линейном 0.0–1.0** (как в шейдере OpenGL).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Компоненты 0–255 (sRGB-окно), деление на 255 — учебное приближение без гамма-коррекции.
    pub fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red as f32 / 255.0,
            g: green as f32 / 255.0,
            b: blue as f32 / 255.0,
        }
    }
}

/// Тип источника света; позже сюда можно добавить `Spot`, `Area` и т.д.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightKind {
    /// Параллельные лучи: направление **к** источнику в мировых координатах (куда «светит» полусфера).
    /// Позиция сущности не используется рендером.
    Directional { toward_light: Vector3<f32> },
    /// Точка в пространстве: мировая позиция берётся из [`Position`] на той же сущности.
    Point {
        constant: f32,
        linear: f32,
        quadratic: f32,
    },
}

impl LightKind {
    /// Направленный свет с нормализованным направлением к источнику.
    pub fn directional_toward_light(direction: Vector3<f32>) -> Self {
        Self::Directional {
            toward_light: normalize_or_fallback(direction, Vector3::new(0.0, 1.0, 0.0)),
        }
    }

    /// Точечный свет с типичным затуханием по расстоянию.
    pub fn point_default_attenuation() -> Self {
        Self::Point {
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }
    }
}

fn normalize_or_fallback(v: Vector3<f32>, fallback: Vector3<f32>) -> Vector3<f32> {
    let n = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if n > 1e-6 { v / n } else { fallback }
}

/// Источник света на сцене: вид ([`LightKind`]), цвет и общая яркость.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub kind: LightKind,
    pub color: Color,
    /// Множитель интенсивности поверх `color` (1.0 — «как есть»).
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

/// Параметры отражения (Blinn–Phong) поверх базового [`Material::color`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceLighting {
    /// Вклад окружающего освещения на альбедо (масштабирует цвет материала).
    pub ambient: f32,
    /// Вклад диффузной составляющей.
    pub diffuse: f32,
    /// Цвет блика (часто белый для диэлектриков, цветной для металлов — упрощённо).
    pub specular_color: Color,
    /// Показатель блеска (чем больше — тем уже и ярче блик).
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

/// Материал для мешей с треугольниками: цвет, непрозрачность и параметры освещения ([`SurfaceLighting`]).
///
/// Без этого компонента **треугольники** не рисуются (куб «прозрачный» / отсутствует в кадре).
/// Линии (сетка) по-прежнему используют только цвет вершин — материал к ним не нужен.
///
/// При `opacity >= 1.0` объект идёт в непрозрачный проход (без `GL_BLEND`, с записью в Z-буфер).
/// При `0.0 < opacity < 1.0` — отдельный проход с смешиванием (дороже по состоянию конвейера).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    /// 1.0 — полностью непрозрачно; меньше — альфа-смешивание.
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

/// Камера на сцене: позиция — в [`Position`], здесь ориентация и проекция.
///
/// `yaw_deg` и `pitch_deg` — в **градусах**: горизонтальный поворот вокруг Y и наклон вверх/вниз.
/// Направление взгляда при `(0°, 0°)` совпадает с прежней камерой «с +Z на центр».
///
/// Если на ту же сущность добавлен [`CameraLookTarget`], [`crate::ecs::systems::camera_look_at_system`]
/// каждый кадр перезаписывает эти углы под цель.
///
/// Если в мире несколько сущностей с `Camera`, [`crate::ecs::systems::render_mesh_system`] берёт **первую**
/// из обхода запроса — держите одну активную камеру или явно порядок спавна.
#[derive(Clone, Debug)]
pub struct Camera {
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    /// Вертикальный угол обзора в градусах.
    pub fovy_deg: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            fovy_deg: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
}

impl Camera {
    pub fn new(yaw_deg: f32, pitch_deg: f32, fovy_deg: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            yaw_deg,
            pitch_deg,
            fovy_deg,
            z_near,
            z_far,
        }
    }
}

/// Цель взгляда для камеры: положите на ту же сущность, что и [`Camera`] + [`Position`].
/// Система [`crate::ecs::systems::camera_look_at_system`] каждый кадр перезаписывает `yaw_deg` / `pitch_deg`
/// у [`Camera`], чтобы смотреть на точку или на другую сущность с [`Position`].
///
/// Если цель — [`CameraLookTarget::Entity`] и сущность уже не в мире, углы не меняются.
#[derive(Clone, Debug)]
pub enum CameraLookTarget {
    /// Фиксированная мировая точка (центр сцены, маркер и т.д.).
    World(Vector3<f32>),
    /// Центр [`Position::position`] другой сущности (например куба).
    Entity(Entity),
}

impl CameraLookTarget {
    pub fn world(point: Vector3<f32>) -> Self {
        Self::World(point)
    }

    pub fn entity(target: Entity) -> Self {
        Self::Entity(target)
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
