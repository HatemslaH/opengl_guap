/// RGB in **linear 0.0–1.0** (as in the OpenGL shader).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Components 0–255 (sRGB window), dividing by 255 is an educational approximation without gamma correction.
    pub fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red as f32 / 255.0,
            g: green as f32 / 255.0,
            b: blue as f32 / 255.0,
        }
    }
}
