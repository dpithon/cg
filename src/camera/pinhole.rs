use super::{ImageSize, Sampler};
use crate::{Cs, Matrix, Ray};

pub struct PinholeCamera {
    pub cs: Cs,
    focale: f64,
    image_size: ImageSize, // TODO: use Image
}

impl PinholeCamera {
    pub fn new(cs: Cs, image_size: ImageSize, focale: f64) -> PinholeCamera {
        PinholeCamera {
            cs,
            image_size,
            focale,
        }
    }

    pub fn iter(&self) -> Sampler {
        Sampler::new(&self.image_size, self.focale)
    }

    pub fn get_matrix(&self) -> &Matrix {
        &self.cs.lcs_to_rcs
    }

    pub fn to_world(&self, ray: &Ray) -> Ray {
        Ray {
            o: &self.cs.lcs_to_rcs * &ray.o,
            v: &self.cs.lcs_to_rcs * &ray.v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Focale, PinholeSettings, Point, SphCoord};

    #[test]
    fn cam_4() {
        let mut theta: f64 = 0.;
        let step = 0.01;

        while theta < std::f64::consts::PI {
            let mut phy: f64 = 0.;
            while phy < 2. * std::f64::consts::PI {
                let cam = PinholeSettings::default()
                    .move_to(Point::new(1., 2., 3.))
                    .look_at(SphCoord::build(12., theta, phy).into_point())
                    .set_focale(Focale::AngleDeg(90.))
                    .set_image_size(640, 480)
                    .build_camera();

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
        let cam = PinholeSettings::default()
            .move_to(Point::new(1., 12., 3.))
            .look_at(Point::new(-12., 34., -4.3))
            .set_image_size(1, 1)
            .build_camera();
        for ray in cam.iter() {
            println!("{}", ray);
        }
    }
}
