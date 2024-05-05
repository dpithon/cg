use std::fmt::Display;

use super::{Focale, ImageSize, Sampler};
use crate::{Cs, Matrix, Point, O, POINT_K};

pub struct Camera {
    location: Point,
    look_at: Point,
    moved: bool,
    image_size: ImageSize,
    focale: Focale,
    cs: Cs,
}

impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ready")
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            location: O,
            look_at: POINT_K,
            moved: false,
            image_size: ImageSize::default(),
            focale: Focale::default(),
            cs: Cs::default(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }

    pub fn iter(&mut self) -> Sampler {
        if self.moved {
            let a = self.location.clone();
            let b = self.look_at.clone();

            self.move_and_point_to(&a, &b);
        }
        Sampler::new(&self.image_size, self.focale.get_focale())
    }

    pub fn get_matrix_to_lcs(&self) -> &Matrix {
        self.cs.get_matrix_to_lcs()
    }

    pub fn get_matrix_to_rcs(&self) -> &Matrix {
        self.cs.get_matrix_to_rcs()
    }

    pub fn move_to(&mut self, p: Point) -> &mut Self {
        self.location = p;
        self.moved = true;
        self
    }

    pub fn look_at(&mut self, p: Point) -> &mut Self {
        self.look_at = p;
        self.moved = true;
        self
    }

    pub fn move_and_point_to(&mut self, a: &Point, b: &Point) -> &mut Self {
        self.cs.complete_cs(a, &(b - a).unit());
        self.moved = false;
        self
    }

    pub fn set_focale(&mut self, focale: Focale) -> &mut Self {
        self.focale = focale;
        self
    }

    pub fn set_image_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.image_size = ImageSize::new(width, height);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_base, Camera, Focale, Point, SphCoord};

    #[test]
    fn cam_4() {
        let mut theta: f64 = 0.;
        let step = 0.01;

        while theta < std::f64::consts::PI {
            let mut phy: f64 = 0.;
            while phy < 2. * std::f64::consts::PI {
                let mut cam = Camera::new();
                cam.move_and_point_to(
                    &Point::new(1., 2., 3.),
                    &SphCoord::build(12., theta, phy).into_point(),
                )
                .set_focale(Focale::AngleDeg(90.))
                .set_image_size(640, 480);
                let lcs = cam.cs.get_matrix_to_lcs();
                if let Err(e) = check_base(&lcs.get_i(), &lcs.get_j(), &lcs.get_k()) {
                    panic!("{e}");
                }
                phy += step;
            }
            theta += step;
        }
    }

    #[test]
    fn cam_5() {
        let mut cam = Camera::new();
        cam.move_to(Point::new(1., 12., 3.))
            .look_at(Point::new(-12., 34., -4.3))
            .set_image_size(1, 1);
        for (_, _, ray) in cam.iter() {
            println!("{}", ray);
        }
    }
}
