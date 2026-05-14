//! Включение теста глубины (Z-буфер).

/// Включает `GL_DEPTH_TEST` для корректного перекрытия ближних и дальних треугольников.
pub fn enable_depth_test() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}
