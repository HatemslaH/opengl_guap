//! Сцена на ECS ([`hecs::World`]): сетка, кубы и другие сущности как наборы компонентов.
//!
//! Рендер и анимация — в [`crate::ecs::systems`], не здесь.
//!
//! Демо [`Scene::with_demo1`]: три разноцветных точечных света, куб и примитивы (сфера, капсула, цилиндр)
//! с одним альбедо и разными [`SurfaceLighting`]; вращение группы мешей вокруг оси Y — клавиши **[** / **]**
//! (обработка в [`GlutinApp`](crate::app::glutin_app::GlutinApp)).

pub mod capsule;
pub mod cube;
pub mod cylinder;
pub mod grid;
pub mod spawn;
pub mod sphere;

pub use crate::ecs::{
    Camera, CameraKeyboardOrbit, CameraLookTarget, Color, KeyboardOrbitKeys, KeyboardSceneRootKeys,
    Light, LightKind, Material, Position, RenderMesh, Rotation, Scale, SpinAnimation,
    SurfaceLighting,
};
pub use spawn::{
    spawn_camera, spawn_camera_with_look, spawn_camera_with_look_and_keyboard_orbit, spawn_capsule,
    spawn_coordinate_grid, spawn_cube, spawn_cylinder, spawn_directional_light, spawn_point_light,
    spawn_sphere,
};

use cgmath::Vector3;
use hecs::{Entity, World};

/// Удобная цепочка вместо старого `Box::new(Cube::new())`: позиция задаётся явно.
///
/// ```ignore
/// Cube::at(vec3(2.0, 0.0, -1.0)).spawn(&mut scene);
/// Cube::at(vec3(0.0, 0.5, 0.0))
///     .with_spin(SpinAnimation::demo_orbit())
///     .spawn(&mut scene);
/// ```
pub struct Cube;

impl Cube {
    pub fn at(translation: Vector3<f32>) -> CubeSpawn {
        CubeSpawn {
            translation,
            spin: SpinAnimation::disabled(),
            material: None,
        }
    }
}

pub struct CubeSpawn {
    translation: Vector3<f32>,
    spin: SpinAnimation,
    material: Option<Material>,
}

impl CubeSpawn {
    /// Подключить (или заменить) анимацию вращения к этому кубу.
    pub fn with_spin(mut self, spin: SpinAnimation) -> Self {
        self.spin = spin;
        self
    }

    /// Материал куба; без вызова — куб без компонента [`Material`] (в кадре не виден).
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }

    /// Зарегистрировать сущность в мире.
    pub fn spawn(self, scene: &mut Scene) -> Entity {
        spawn::spawn_cube(
            &mut scene.world,
            self.translation,
            None,
            None,
            self.spin,
            self.material,
        )
    }
}

/// Владеет [`World`](hecs::World) и всеми сущностями сцены.
pub struct Scene {
    pub world: World,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn with_demo0() -> Self {
        let mut s = Self::new();

        let cube = spawn_capsule(
            &mut s.world,
            Vector3::new(0.0, 0.0, 0.0),
            None,
            Some(Scale::new(1.0, 1.0, 1.0)),
            SpinAnimation::disabled(),
            Some(Material::opaque(Color::from_rgb8(22, 1, 244))),
        );

        spawn_camera_with_look_and_keyboard_orbit(
            &mut s.world,
            Vector3::new(-2.2, 2.4, 3.0),
            Camera::new(88.0, 0.1, 100.0),
            CameraLookTarget::Entity(cube),
            CameraKeyboardOrbit::default(),
        );

        s
    }

    pub fn with_demo1() -> Self {
        let mut s = Self::new();
        spawn_coordinate_grid(&mut s.world, 8.0, 1.0);

        // Три точечных источника: разные позиции и цвета.
        spawn_point_light(
            &mut s.world,
            Vector3::new(3.0, 2.4, 1.2),
            Light::new(
                LightKind::point_default_attenuation(),
                Color::from_rgb8(255, 195, 150),
                1.25,
            ),
        );
        spawn_point_light(
            &mut s.world,
            Vector3::new(-2.8, 1.3, -0.8),
            Light::new(
                LightKind::point_default_attenuation(),
                Color::from_rgb8(110, 200, 255),
                1.1,
            ),
        );
        spawn_point_light(
            &mut s.world,
            Vector3::new(0.2, 0.9, -3.2),
            Light::new(
                LightKind::point_default_attenuation(),
                Color::from_rgb8(210, 130, 255),
                1.0,
            ),
        );

        let albedo = Color::from_rgb8(208, 208, 218);

        let surf_gloss = SurfaceLighting {
            ambient: 0.1,
            diffuse: 0.92,
            specular_color: Color::new(1.0, 1.0, 1.0),
            shininess: 140.0,
        };
        let surf_matte = SurfaceLighting {
            ambient: 0.26,
            diffuse: 0.52,
            specular_color: Color::new(0.7, 0.72, 0.75),
            shininess: 14.0,
        };
        let surf_metal = SurfaceLighting {
            ambient: 0.09,
            diffuse: 0.58,
            specular_color: Color::new(0.82, 0.88, 1.0),
            shininess: 88.0,
        };
        let surf_plastic = SurfaceLighting {
            ambient: 0.15,
            diffuse: 0.85,
            specular_color: Color::new(1.0, 0.96, 0.88),
            shininess: 42.0,
        };
        let surf_soft = SurfaceLighting {
            ambient: 0.2,
            diffuse: 0.68,
            specular_color: Color::new(0.92, 0.98, 1.0),
            shininess: 28.0,
        };

        let cube = spawn_cube(
            &mut s.world,
            Vector3::new(0.0, 0.35, 0.0),
            None,
            Some(Scale::new(1.15, 0.7, 1.15)),
            SpinAnimation::disabled(),
            Some(Material::opaque(albedo).with_surface(surf_gloss)),
        );
        let _ = spawn_sphere(
            &mut s.world,
            Vector3::new(2.0, 0.5, 0.6),
            None,
            Some(Scale::new(0.55, 0.55, 0.55)),
            SpinAnimation::disabled(),
            Some(Material::opaque(albedo).with_surface(surf_matte)),
        );
        let _ = spawn_capsule(
            &mut s.world,
            Vector3::new(-1.7, 0.65, 0.9),
            None,
            Some(Scale::new(0.75, 0.75, 0.75)),
            SpinAnimation::disabled(),
            Some(Material::opaque(albedo).with_surface(surf_metal)),
        );
        let _ = spawn_cylinder(
            &mut s.world,
            Vector3::new(0.6, 0.5, -2.1),
            None,
            Some(Scale::new(0.65, 0.55, 0.65)),
            SpinAnimation::disabled(),
            Some(Material::opaque(albedo).with_surface(surf_plastic)),
        );
        let _ = spawn_cube(
            &mut s.world,
            Vector3::new(-1.2, 0.32, -1.6),
            None,
            Some(Scale::new(0.5, 0.85, 0.5)),
            SpinAnimation::disabled(),
            Some(Material::opaque(albedo).with_surface(surf_soft)),
        );

        spawn_camera_with_look_and_keyboard_orbit(
            &mut s.world,
            Vector3::new(-2.2, 2.4, 3.0),
            Camera::new(88.0, 0.1, 100.0),
            CameraLookTarget::Entity(cube),
            CameraKeyboardOrbit::default(),
        );
        s
    }

    pub fn spawn_grid_default(&mut self) {
        spawn_coordinate_grid(&mut self.world, 8.0, 1.0);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
