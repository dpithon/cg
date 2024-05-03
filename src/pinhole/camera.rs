use std::{fmt::Display, marker::PhantomData};

use super::{Focale, ImageSize, Sampler};
use crate::{Cs, Matrix, Point, Vector, O, POINT_K};

pub struct Locked;
pub struct Unlocked;

pub struct Camera<State = Unlocked> {
    location: Point,
    look_at: Point,
    image_size: ImageSize,
    focale: Focale,
    cs: Cs,
    state: PhantomData<State>,
}

impl Display for Camera {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            location: O,
            look_at: POINT_K,
            image_size: ImageSize::default(),
            focale: Focale::default(),
            cs: Cs::default(),
            state: PhantomData,
        }
    }
}

impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }
}

impl Camera<Unlocked> {
    pub fn move_to(mut self, p: Point) -> Self {
        self.location = p;
        self
    }

    pub fn look_at(mut self, p: Point) -> Self {
        self.look_at = p;
        self
    }

    pub fn set_focale(mut self, focale: Focale) -> Self {
        self.focale = focale;
        self
    }

    pub fn set_image_size(mut self, width: u32, height: u32) -> Self {
        self.image_size = ImageSize::new(width, height);
        self
    }

    pub fn lock(self) -> Camera<Locked> {
        let cam: Camera<Locked> = Camera {
            location: self.location,
            look_at: self.look_at,
            image_size: self.image_size,
            focale: self.focale,
            cs: Cs::default(),
            state: PhantomData,
        };
        cam
    }

    pub fn complete_lcs(mut self, p: &Point, k: &Vector) -> Self {
        self.cs.complete_lcs(p, k);
        self
    }
}

impl Camera<Locked> {
    pub fn iter(&self) -> Sampler {
        Sampler::new(&self.image_size, self.focale.get_focale())
    }

    pub fn get_cs(&self) -> &Cs {
        &self.cs
    }

    pub fn get_matrix_to_lcs(&self) -> &Matrix {
        self.cs.get_matrix_to_lcs()
    }

    pub fn get_matrix_to_rcs(&self) -> &Matrix {
        self.cs.get_matrix_to_rcs()
    }

    pub fn unlock(self) -> Camera<Unlocked> {
        let cam: Camera<Unlocked> = Camera {
            location: self.location,
            look_at: self.look_at,
            image_size: self.image_size,
            focale: self.focale,
            cs: self.cs,
            state: PhantomData,
        };
        cam
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
                let cam = Camera::new()
                    .complete_lcs(
                        &Point::new(1., 2., 3.),
                        &SphCoord::build(12., theta, phy).into_vector().unit(),
                    )
                    .set_focale(Focale::AngleDeg(90.))
                    .set_image_size(640, 480)
                    .lock();
                if let Err(e) = check_base(&cam.cs.get_i(), &cam.cs.get_j(), &cam.cs.get_k()) {
                    panic!("{e}");
                }
                phy += step;
            }
            theta += step;
        }
    }

    #[test]
    fn cam_5() {
        let cam = Camera::default()
            .move_to(Point::new(1., 12., 3.))
            .look_at(Point::new(-12., 34., -4.3))
            .set_image_size(1, 1)
            .lock();
        for (_, _, ray) in cam.iter() {
            println!("{}", ray);
        }
    }
}
