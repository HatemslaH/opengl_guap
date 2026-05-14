//! Состояние конвейера для непрозрачной и прозрачной отрисовки.

/// Непрозрачные меши: запись в Z-буфер, без смешивания.
pub fn set_opaque_depth_blend() {
    unsafe {
        gl::DepthMask(gl::TRUE);
        gl::Disable(gl::BLEND);
    }
}

/// Полупрозрачные меши: смешивание по альфе, без записи в глубину (типовой порядок после непрозрачного прохода).
pub fn set_transparent_depth_blend() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::DepthMask(gl::FALSE);
    }
}
