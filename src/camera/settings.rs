use std::fmt::Display;

use super::{Focale, ImageSize, Orientation, PinholeCamera};
use crate::{Cs, Point, Vector};

#[derive(Default)]
pub struct PinholeSettings {
    orientation: Orientation,
    image_size: ImageSize,
    focale: Focale,
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

    pub fn get_focale(&self) -> f64 {
        self.focale.get_focale()
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

    pub fn build_camera(&self) -> PinholeCamera {
        PinholeCamera::new(
            Cs::build_from_k(self.get_location(), &self.compute_heading()),
            self.image_size,
            self.get_focale(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Focale, PinholeSettings, Point};

    #[test]
    fn settings_1() {
        let settings = PinholeSettings::default();
        let _ = settings.build_camera();
    }

    #[test]
    fn cam_5() {
        let camera = PinholeSettings::default()
            .move_to(Point::new(1., 2., 3.))
            .look_at(Point::new(4., -2., 8.))
            .set_focale(Focale::AngleDeg(45.))
            .set_image_size(1, 1)
            .build_camera();

        for ray in camera.iter() {
            println!("{}", ray);
        }
    }
}
