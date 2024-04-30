use super::Shapes;
use crate::{Cs, Matrix, Ray, Vector, ID_MATRIX, J};

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
            cam_to_lcs: ID_MATRIX,
        }
    }
}

impl Shapes for Cylinder {
    fn set_camera_cs(&mut self, cam_to_rcs: &Matrix) {
        self.cam_to_lcs = &self.cs.rcs_to_lcs * cam_to_rcs;
    }

    fn scale(&mut self, f: f64) {
        self.cs.scale(f);
    }

    fn translate(&mut self, v: &Vector) {
        self.cs.translate(v);
    }

    fn rotate_x(&mut self, deg: f64) {
        self.cs.rotate_x(deg);
    }

    fn rotate_y(&mut self, deg: f64) {
        self.cs.rotate_y(deg);
    }

    fn rotate_z(&mut self, deg: f64) {
        self.cs.rotate_z(deg);
    }
    fn intersect(&self, _ray: &crate::Ray) -> bool {
        false
    }

    fn intersect_min(&self, ray: &crate::Ray) -> Option<f64> {
        let ray = Ray {
            v: &self.cam_to_lcs * &ray.v,
            o: &self.cam_to_lcs * &ray.o,
        };

        let val = ray.o.q.x * ray.o.q.x + ray.o.q.z * ray.o.q.z;
        if (&ray.v ^ &J).nearly_zero() && val <= self.radius2 {
            // FIXME: ray.v ^ J => ray.v.y ~ 0
            Some(std::f64::MIN_POSITIVE)
        } else {
            let a = ray.v.q.x * ray.v.q.x + ray.v.q.z * ray.v.q.z;
            let b = 2. * (ray.v.q.x * ray.o.q.x + ray.v.q.z * ray.o.q.z);
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
