# opengl_guap

A small **Rust game-engine style** stack: **winit** windowing, **glutin** OpenGL context, **OpenGL 3.3 Core** rendering, and an **ECS** world built on [**hecs**](https://docs.rs/hecs/). The project is educational but organized like a miniature engine: **runtime** (`app`), **gameplay layer** (`game`), **simulation data** (`ecs` + `scene`), and **GPU layer** (`graphics`).

---

## Features

| Area | What you get |
|------|----------------|
| **Runtime** | Event loop, resize, GL surface/context, frame timing, optional FPS overlay |
| **ECS** | `Position`, `Rotation`, `Scale`, `Camera`, `Light`, `Material`, `RenderMesh` |
| **Scene** | `Scene` owns `hecs::World`; helpers spawn grid, cube, sphere, capsule, cylinder, lights, camera |
| **Rendering** | Lit mesh shader, depth test, transparent/opaque blend helpers, model + view–projection math |
| **Game layer** | Camera look-at target, keyboard orbit, scene-root yaw (demo) |

---

## Tech stack

- **Rust** (edition 2024)
- **OpenGL** via [`gl`](https://docs.rs/gl/)
- **Window / context**: [`winit`](https://docs.rs/winit/), [`glutin`](https://docs.rs/glutin/), [`glutin-winit`](https://docs.rs/glutin-winit/)
- **Math**: [`cgmath`](https://docs.rs/cgmath/)
- **ECS**: [`hecs`](https://docs.rs/hecs/)

---

## Quick start

```bash
cargo run
```

Release build:

```bash
cargo run --release
```

Default startup uses [`Scene::with_demo1`](src/scene/mod.rs): coordinate grid, several lit meshes, three point lights, and a camera that orbits with **W A S D** (see [Controls](#controls)).

Library entry point: [`opengl_guap::run`](src/lib.rs) → [`app::runner::run`](src/app/runner.rs).

---

## Architecture

Think of the crate in four layers (bottom = closest to hardware):

```text
┌─────────────────────────────────────────────┐
│  app          Event loop, GlutinApp, timing │
├─────────────────────────────────────────────┤
│  game         Camera orbit, look-at, input  │
├─────────────────────────────────────────────┤
│  ecs + scene  Components, World, spawners   │
├─────────────────────────────────────────────┤
│  graphics     Shaders, meshes, GL state     │
└─────────────────────────────────────────────┘
```

- **`graphics`** — Shaders, VAO/VBO [`Mesh`](src/graphics/mesh.rs), [`ShaderProgram`](src/graphics/shader.rs), math helpers ([`model_matrix`](src/graphics/math.rs), view–projection, normals). No ECS types.
- **`ecs`** — Reusable simulation/render components under [`src/ecs/components/`](src/ecs/components/) and systems under [`src/ecs/systems/`](src/ecs/systems/) (e.g. [`render_mesh_system`](src/ecs/systems/render.rs)).
- **`scene`** — [`Scene`](src/scene/mod.rs) wraps `hecs::World` and documents demo scenes; [`spawn`](src/scene/spawn.rs) builds entities (camera, lights, primitives).
- **`game`** — Gameplay-oriented pieces: [`CameraLookTarget`](src/game/components/camera_look_target.rs), [`CameraKeyboardOrbit`](src/game/components/camera_orbit.rs), keyboard state; systems in [`src/game/systems/`](src/game/systems/) are invoked from [`GlutinApp`](src/app/glutin_app.rs) in a fixed order.
- **`app`** — [`GlutinApp`](src/app/glutin_app.rs) wires winit events to systems and `render_mesh_system`; owns the active [`Scene`](src/scene/mod.rs).

Per-frame order (simplified): **camera orbit** → **camera look-at** → **optional scene-root yaw** → **clear** → **render meshes** → **FPS overlay** → **swap**.

---

## Controls

| Input | Action |
|-------|--------|
| **W A S D** | Orbit camera (when demo spawns [`CameraKeyboardOrbit`](src/game/components/camera_orbit.rs)) |
| **[** / **]** | Rotate demo scene root around world **Y** (meshes; lights stay in world space) |

---

## Module map (`src/`)

| Path | Role |
|------|------|
| [`main.rs`](src/main.rs) | Binary entry: calls `opengl_guap::run()` |
| [`lib.rs`](src/lib.rs) | Crate root: exports `app`, `ecs`, `game`, `graphics`, `scene` |
| [`app/`](src/app/) | `EventLoop`, [`GlutinApp`](src/app/glutin_app.rs), [`runner`](src/app/runner.rs) |
| [`ecs/`](src/ecs/) | ECS components and systems |
| [`game/`](src/game/) | Camera/input helpers and their systems |
| [`graphics/`](src/graphics/) | GL-facing rendering utilities |
| [`scene/`](src/scene/) | `Scene`, geometry builders, [`spawn`](src/scene/spawn.rs) |

Module-level `//!` comments in the repo are partly in Russian (course / lab style).

---

## Extending the “engine”

### Spawn an entity

Use helpers from [`scene::spawn`](src/scene/spawn.rs) or call [`World::spawn`](https://docs.rs/hecs/latest/hecs/struct.World.html#method.spawn) with a tuple of components. Lit triangle meshes need [`Material`](src/ecs/components/material.rs) (and matching vertex layout: position, color, normal — see existing primitives). The coordinate grid uses [`MeshTopology::Lines`](src/graphics/mesh.rs) and vertex color only.

Example: add a cube with opaque material (adjust imports to match your module path):

```rust
use opengl_guap::ecs::{Material, Position, Scale};
use opengl_guap::graphics::Color;
use opengl_guap::scene::{spawn_cube, Scene};

let mut scene = Scene::new();
spawn_cube(
    &mut scene.world,
    Position::new(0.0, 0.5, 0.0),
    None,
    Some(Scale::new(1.0, 1.0, 1.0)),
    Some(Material::opaque(Color::from_rgb8(90, 200, 120))),
);
```

### Add a system

1. Implement a function `fn your_system(world: &mut hecs::World, …)`.
2. If it is generic simulation/render logic, place it under `ecs/systems` and export it from [`ecs/systems/mod.rs`](src/ecs/systems/mod.rs).
3. If it depends on game-specific components, prefer `game/systems`.
4. Call it from [`GlutinApp::window_event`](src/app/glutin_app.rs) in the redraw path, **before** or **after** other systems depending on dependencies (e.g. camera before rendering).

### Add a mesh primitive

1. Build interleaved vertex data like [`build_cube_vertex_data`](src/scene/cube.rs) (position + color + normal for lit shaders).
2. Wrap with [`Mesh::new_interleaved_pos3_color3_normal3`](src/graphics/mesh.rs).
3. Spawn `(Position, Option<Rotation>, Option<Scale>, RenderMesh { mesh, topology }, Option<Material>)` as appropriate.

---

## Development

```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```

---

## License / status

Educational project; version **0.1.0** — APIs and folder layout may change as features grow.
