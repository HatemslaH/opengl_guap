# opengl_guap

Учебный проект: окно **winit** + контекст **glutin**, рендер **OpenGL 3.3 Core** на Rust, сцена на **ECS** ([`hecs`](https://docs.rs/hecs/)).

## Сборка и запуск

```bash
cargo run
```

Релиз:

```bash
cargo run --release
```

## Структура каталогов `src/`

| Путь | Назначение |
|------|------------|
| [`main.rs`](src/main.rs) | Точка входа исполняемого файла: вызывает [`opengl_guap::run`](src/lib.rs). |
| [`lib.rs`](src/lib.rs) | Корень крейта: `app`, `ecs`, `graphics`, `scene`. |
| [`app/`](src/app/) | Цикл событий, окно, GL-контекст; вызов систем `spin_animation_system` и `render_mesh_system`. |
| [`ecs/`](src/ecs/) | Компоненты ([`Transform`](src/ecs/components.rs), [`SpinAnimation`](src/ecs/components.rs), [`RenderMesh`](src/ecs/components.rs)) и системы ([`systems.rs`](src/ecs/systems.rs)). |
| [`graphics/`](src/graphics/) | Шейдеры, [`Mesh`](src/graphics/mesh.rs), матрицы [`view_projection_matrix`](src/graphics/math.rs) / [`model_matrix`](src/graphics/math.rs). |
| [`scene/`](src/scene/) | [`Scene`](src/scene/mod.rs) = [`hecs::World`](https://docs.rs/hecs/), спавн сущностей ([`spawn.rs`](src/scene/spawn.rs)), геометрия куба/сетки. |

Комментарии на русском — в начале модулей (`//!`).

## ECS: как добавить куб в позицию

Сцена владеет [`Scene::world`](src/scene/mod.rs). Куб — сущность с компонентами `Transform` + `RenderMesh` + `SpinAnimation` (у сетки вращение выключено).

**Вариант 1 — цепочка (аналог «собрать и добавить»):**

```rust
use cgmath::vec3;
use opengl_guap::scene::{Cube, Scene, SpinAnimation};

let mut scene = Scene::new();
scene.spawn_grid_default();
Cube::at(vec3(2.0, 0.0, -1.5)).spawn(&mut scene);
Cube::at(vec3(0.0, 0.5, 0.0))
    .with_spin(SpinAnimation::demo_orbit())
    .spawn(&mut scene);
```

**Вариант 2 — напрямую через `spawn_cube`:**

```rust
use cgmath::vec3;
use opengl_guap::scene::{spawn_cube, Scene, SpinAnimation};

let mut scene = Scene::new();
spawn_cube(&mut scene.world, vec3(1.0, 0.0, 0.0), SpinAnimation::disabled());
```

## Анимация вращения (отключаемая)

Компонент [`SpinAnimation`](src/ecs/components.rs): поля `enabled`, `velocity_y`, `velocity_x`, накопленные `phase_*`. Система [`spin_animation_system`](src/ecs/systems.rs) обновляет фазы только если `enabled == true`. У сущности без вращения задайте [`SpinAnimation::disabled()`](src/ecs/components.rs).

Камера и сетка **не** вращаются вместе с объектом: в MVP входит только `view_projection_matrix` × **модель** сущности.

## Как добавить другую фигуру

1. Опишите вершины (как [`build_cube_vertex_data`](src/scene/cube.rs)) и создайте [`Mesh`](src/graphics/mesh.rs).
2. Вызовите [`hecs::World::spawn`](https://docs.rs/hecs/latest/hecs/struct.World.html#method.spawn) с кортежем `(Transform { translation: ... }, RenderMesh { mesh, topology }, SpinAnimation::disabled())` (или со своей анимацией).
3. При необходимости добавьте новые системы в [`ecs/systems.rs`](src/ecs/systems.rs) и вызовите их из [`GlutinApp`](src/app/glutin_app.rs) в нужном порядке.

## Проверки

```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```
