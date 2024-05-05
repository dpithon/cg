use super::Shapes;
use crate::{Cs, Matrix, Ray, J};

pub struct Cylinder {
    pub cs: Cs,
    pub cam_to_lcs: Matrix,
    pub radius: f64,
    pub radius2: f64,
}

impl Cylinder {
    pub fn build(radius: f64) -> Cylinder {
        assert!(radius > 0.);

        let cs = Cs::new();

        Cylinder {
            cs,
            radius,
            radius2: radius * radius,
            cam_to_lcs: Matrix::default(),
        }
    }
}

impl Shapes for Cylinder {
    fn set_transform(&mut self, m: Matrix) {
        self.cam_to_lcs = m;
    }

    fn get_matrix_to_lcs(&self) -> &Matrix {
        self.cs.get_matrix_to_lcs()
    }

    fn get_matrix_to_rcs(&self) -> &Matrix {
        self.cs.get_matrix_to_rcs()
    }

    fn set_shape_cs(&mut self, cs: Cs) {
        self.cs = cs;
    }

    fn intersect(&self, _ray: &crate::Ray) -> bool {
        false
    }

    fn intersect_min(&self, ray: &crate::Ray) -> Option<f64> {
        let ray = Ray {
            v: &self.cam_to_lcs * &ray.v,
            o: &self.cam_to_lcs * &ray.o,
        };

        let val = ray.o.x * ray.o.x + ray.o.z * ray.o.z;
        if (&ray.v ^ &J).nearly_zero() && val <= self.radius2 {
            // FIXME: ray.v ^ J => ray.v.y ~ 0
            Some(std::f64::MIN_POSITIVE)
        } else {
            let a = ray.v.x * ray.v.x + ray.v.z * ray.v.z;
            let b = 2. * (ray.v.x * ray.o.x + ray.v.z * ray.o.z);
            let c = val - self.radius2;

            let delta = b * b - 4. * a * c;
            if delta > 0. {
                let k = (-b - delta.sqrt()) / (2. * a);
                if k > 0. {
                    Some(k)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
