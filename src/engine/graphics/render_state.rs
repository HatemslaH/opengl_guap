//! Render pipeline state for opaque and transparent rendering.

/// Opaque meshes: writing to the Z-buffer, without blending.
pub fn set_opaque_depth_blend() {
    unsafe {
        gl::DepthMask(gl::TRUE);
        gl::Disable(gl::BLEND);
    }
}

/// Transparent meshes: blending by alpha, without writing to the depth (typical order after the opaque pass).
pub fn set_transparent_depth_blend() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::DepthMask(gl::FALSE);
    }
}
