use std::fmt::Display;

use super::{Focale, ImageSize, PinholeCamera};
use crate::{Cs, Point, Vector};

#[derive(Default)]
pub struct PinholeSettings {
    cs: Cs,
    image_size: ImageSize,
    focale: Focale,
}

impl Display for PinholeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.focale, self.image_size)
    }
}

impl PinholeSettings {
    pub fn new() -> PinholeSettings {
        PinholeSettings::default()
    }

    pub fn set_lcs(&mut self, cs: Cs) -> &mut PinholeSettings {
        self.cs = cs;
        self
    }

    pub fn set_focale(&mut self, focale: Focale) -> &mut PinholeSettings {
        self.focale = focale;
        self
    }

    pub fn get_focale(&self) -> f64 {
        self.focale.get_focale()
    }

    pub fn set_image_size(&mut self, width: u32, height: u32) -> &mut PinholeSettings {
        self.image_size = ImageSize::new(width, height);
        self
    }

    pub fn build_camera(&self) -> PinholeCamera {
        PinholeCamera::new(self.cs, self.image_size, self.get_focale())
    }
}
