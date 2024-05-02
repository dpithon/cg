use super::{focale::DEFAULT_FOCALE, ImageSize, Sampler};
use crate::{BindToCs, Cs, Matrix, Ray, ID_MATRIX};

pub struct PinholeCamera {
    pub cs: Cs,
    focale: f64,
    image_size: ImageSize, // TODO: use Image
}

impl Default for PinholeCamera {
    fn default() -> Self {
        PinholeCamera {
            cs: Cs::default(),
            focale: DEFAULT_FOCALE,
            image_size: ImageSize::default(),
        }
    }
}

impl BindToCs for PinholeCamera {
    fn get_cs(&self) -> &mut Cs {
        &mut self.cs
    }
}

impl PinholeCamera {
    pub fn new() -> PinholeCamera {
        PinholeCamera::default()
    }

    pub fn iter(&self) -> Sampler {
        Sampler::new(&self.image_size, self.focale)
    }

    pub fn get_lcs_to_rcs(&self) -> &Matrix {
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
    use crate::{check_base, Cs, Focale, PinholeSettings, Point, SphCoord, Vector};

    #[test]
    fn cam_4() {
        let mut theta: f64 = 0.;
        let step = 0.01;

        while theta < std::f64::consts::PI {
            let mut phy: f64 = 0.;
            while phy < 2. * std::f64::consts::PI {
                let mut cs = Cs::new();
                cs.compute_lcs_with(
                    &Point::new(1., 2., 3.),
                    &SphCoord::build(12., theta, phy).into_vector().unit(),
                );

                let cam = PinholeSettings::default()
                    .set_cam_cs(cs)
                    .set_focale(Focale::AngleDeg(90.))
                    .set_image_size(640, 480)
                    .build_camera();
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
        let cam = PinholeSettings::default()
            .move_to(Point::new(1., 12., 3.))
            .look_at(Point::new(-12., 34., -4.3))
            .set_image_size(1, 1)
            .build_camera();
        for (_, _, ray) in cam.iter() {
            println!("{}", ray);
        }
    }
}
