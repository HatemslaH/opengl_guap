# opengl_guap

Учебный проект: окно **winit** + контекст **glutin**, рендер **OpenGL 3.3 Core** на Rust.

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
| [`lib.rs`](src/lib.rs) | Корень библиотеки крейта: модули `app`, `graphics`, `scene` и функция `run()`. |
| [`app/`](src/app/) | Цикл событий, окно, поверхность и контекст OpenGL; порядок `Clear` → шейдер → сцена → `swap_buffers`. |
| [`graphics/`](src/graphics/) | Шейдеры, VAO/VBO ([`Mesh`](src/graphics/mesh.rs)), тест глубины, матрица MVP для кадра. |
| [`scene/`](src/scene/) | Трейт «нарисуемый объект», список [`Scene`](src/scene/mod.rs), пример — [`Cube`](src/scene/cube.rs). Тип [`Scene`](src/scene/mod.rs) объявлен в [`mod.rs`](src/scene/mod.rs), чтобы не дублировать имя подмодуля `scene`. |

Подробные комментарии на русском — в начале каждого модуля (`//!`).

## Как добавить свою фигуру вместо (или вместе с) куба

1. **Создайте тип** в `src/scene/` (например `pyramid.rs`) и объявите `pub mod pyramid` в [`scene/mod.rs`](src/scene/mod.rs).
2. **Реализуйте** [`Drawable`](src/scene/drawable.rs): в `draw` привяжите свой VAO и вызовите отрисовку (шейдер и `uMVP` к этому моменту уже выставлены в [`GlutinApp`](src/app/glutin_app.rs), при необходимости используйте [`DrawContext`](src/scene/drawable.rs) для доступа к шейдеру и матрице).
3. **Зарегистрируйте объект** в сцене: в [`GlutinApp::resumed`](src/app/glutin_app.rs) после `Scene::with_demo_cube()` создайте сцену через [`Scene::new`](src/scene/mod.rs) и вызовите [`scene.add(Box::new(ВашаФигура::new()))`](src/scene/mod.rs) (или расширьте фабрику сцены отдельным методом).

Общий шейдер сейчас один (позиция + цвет + `uMVP`). Для другого пайплайна логично добавить второй [`ShaderProgram`](src/graphics/shader.rs) и выбирать его в объекте или на уровне сцены.

## Проверки качества

```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```
