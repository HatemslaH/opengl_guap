//! Enabling depth test (Z-buffer).

/// Enables `GL_DEPTH_TEST` for correct overlapping of near and far triangles.
pub fn enable_depth_test() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}
