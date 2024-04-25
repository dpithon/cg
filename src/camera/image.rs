use std::fmt::Display;

pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

pub const DEFAULT_WIDTH: u32 = 320;
pub const DEFAULT_HEIGHT: u32 = 240;

impl Default for ImageSize {
    fn default() -> ImageSize {
        ImageSize {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
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
