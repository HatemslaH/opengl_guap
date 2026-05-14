//! Сцена: состав объектов и точка расширения для новых фигур.
//!
//! **Не** помещайте сюда код компиляции шейдеров или создания контекста — это [`crate::graphics`] и [`crate::app`].

pub mod cube;
pub mod drawable;

pub use cube::Cube;
pub use drawable::{DrawContext, Drawable};

/// Владеет нарисуемыми объектами; порядок в векторе — порядок отрисовки.
pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Демо-сцена: один цветной куб.
    pub fn with_demo_cube() -> Self {
        let mut s = Self::new();
        s.add(Box::new(Cube::new()));
        s
    }

    pub fn add(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }

    pub fn draw_all(&self, ctx: &DrawContext<'_>) {
        for obj in &self.objects {
            obj.draw(ctx);
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
