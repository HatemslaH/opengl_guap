//! Сцена на ECS ([`hecs::World`]): сетка, кубы и другие сущности как наборы компонентов.
//!
//! Рендер и анимация — в [`crate::ecs::systems`], не здесь.

pub mod cube;
pub mod grid;
pub mod spawn;

pub use crate::ecs::{
    Camera, CameraLookTarget, Color, Material, Position, RenderMesh, SpinAnimation,
};
pub use spawn::{spawn_camera, spawn_camera_with_look, spawn_coordinate_grid, spawn_cube};

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
        spawn::spawn_cube(&mut scene.world, self.translation, self.spin, self.material)
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

    pub fn with_demo() -> Self {
        let mut s = Self::new();
        spawn_coordinate_grid(&mut s.world, 8.0, 1.0);
        let cube = spawn_cube(
            &mut s.world,
            Vector3::new(0.0, 0.0, 0.0),
            SpinAnimation::disabled(),
            Some(Material::new(Color::from_rgb8(120, 180, 255), 1.0)),
        );
        let _cube1 = spawn_cube(
            &mut s.world,
            Vector3::new(1.0, -0.5, -1.0),
            SpinAnimation::disabled(),
            Some(Material::new(Color::from_rgb8(152, 50, 51), 0.5)),
        );
        spawn_camera_with_look(
            &mut s.world,
            Vector3::new(-2.0, 2.0, 2.8),
            Camera::new(0.0, 0.0, 90.0, 0.1, 100.0),
            CameraLookTarget::Entity(cube),
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
