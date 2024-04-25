use std::fmt::Display;

use super::Focale;
use super::ImageSize;
use super::Orientation;

use crate::{Point, Vector};

#[derive(Default)]
pub struct PinholeSettings {
    pub orientation: Orientation,
    pub image_size: ImageSize,
    pub focale: Focale,
}

impl Display for PinholeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            self.orientation, self.focale, self.image_size
        )
    }
}

impl PinholeSettings {
    pub fn new() -> PinholeSettings {
        PinholeSettings::default()
    }

    pub fn move_to(&mut self, location: Point) -> &mut PinholeSettings {
        self.orientation.move_to(location);
        self
    }

    pub fn look_at(&mut self, target: Point) -> &mut PinholeSettings {
        self.orientation.look_at(target);
        self
    }

    pub fn set_heading(&mut self, heading: Vector) -> &mut PinholeSettings {
        self.orientation.set_heading(heading);
        self
    }

    pub fn set_focale(&mut self, focale: Focale) -> &mut PinholeSettings {
        self.focale = focale;
        self
    }

    pub fn set_image_size(&mut self, width: u32, height: u32) -> &mut PinholeSettings {
        self.image_size = ImageSize::new(width, height);
        self
    }

    pub fn get_location(&self) -> &Point {
        self.orientation.get_location()
    }

    pub fn compute_heading(&self) -> Vector {
        self.orientation.compute_heading()
    }

    pub fn get_focale(&self) -> f64 {
        self.focale.get_focale()
    }
}
