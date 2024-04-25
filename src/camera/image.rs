use std::fmt::Display;

pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "image    : {}x{}", self.width, self.height)
    }
}
impl ImageSize {
    pub fn new(width: u32, height: u32) -> ImageSize {
        ImageSize { width, height }
    }
}
