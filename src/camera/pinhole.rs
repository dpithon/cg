use std::fmt::Display;

use super::{Focale, PinholeSettings, Sampler};
use crate::{Cs, Point, Vector};

#[derive(Default)]
pub struct Pinhole {
    settings: PinholeSettings,
    cs: Cs,
}

impl Display for Pinhole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.settings)
    }
}

impl Pinhole {
    pub fn new(settings: PinholeSettings) -> Pinhole {
        let cs = Cs::build_from_k(&settings.get_location(), &settings.get_heading());
        Pinhole { settings, cs }
    }

    pub fn move_to(&mut self, location: Point) -> &mut Pinhole {
        self.settings.move_to(location);
        self
    }

    pub fn look_at(&mut self, target: Point) -> &mut Pinhole {
        self.settings.look_at(target);
        self
    }

    pub fn set_heading(&mut self, heading: Vector) -> &mut Pinhole {
        self.settings.set_heading(heading);
        self
    }

    pub fn set_focale(&mut self, focale: Focale) -> &mut Pinhole {
        self.settings.set_focale(focale);
        self
    }

    pub fn set_image_size(&mut self, width: u32, height: u32) -> &mut Pinhole {
        self.settings.set_image_size(width, height);
        self
    }

    pub fn setup(&mut self) {
        self.cs = Cs::build_from_k(&self.settings.get_location(), &self.settings.get_heading());
    }

    pub fn iter(&self) -> Sampler {
        Sampler::new(&self.settings.image_size, self.settings.focale.get_focale())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn cam_4() {
        let mut theta: f64 = 0.;
        let step = 0.01;

        while theta < std::f64::consts::PI {
            let mut phy: f64 = 0.;
            while phy < 2. * std::f64::consts::PI {
                let mut cam = Pinhole::default();

                cam.move_to(Point::new(1., 2., 3.))
                    .look_at(SphCoord::build(12., theta, phy).into_point())
                    .set_focale(Focale::AngleDeg(90.))
                    .set_image_size(640, 480);

                if let Err(e) = Cs::check_base(&cam.cs.i, &cam.cs.j, &cam.cs.k) {
                    panic!("{e}");
                }
                phy += step;
            }
            theta += step;
        }
    }

    #[test]
    fn cam_5() {
        let cam = Pinhole::default();
        for ray in cam.iter() {
            println!("{}", ray);
        }
        println!("{cam}");
    }
}
