//! Per-frame GPU-oriented counters filled by the renderer (CPU-side; each [`gl::DrawArrays`] /
//! [`gl::DrawArraysInstanced`] ≈ one draw).

/// Counts from [`crate::engine::ecs::systems::render::render_mesh_system`].
#[derive(Clone, Copy, Default, Debug)]
pub struct SceneRenderStats {
    /// Total [`gl::DrawArrays`] issued for scene meshes (lines + opaque tris + transparent tris).
    pub draw_calls: u32,
    pub lines_drawn: u32,
    pub opaque_triangles_drawn: u32,
    pub transparent_triangles_drawn: u32,
}

impl SceneRenderStats {
    pub(crate) fn record_line_draw(&mut self) {
        self.lines_drawn += 1;
        self.draw_calls += 1;
    }

    pub(crate) fn record_triangle_draw(&mut self, transparent: bool) {
        if transparent {
            self.transparent_triangles_drawn += 1;
        } else {
            self.opaque_triangles_drawn += 1;
        }
        self.draw_calls += 1;
    }
}
