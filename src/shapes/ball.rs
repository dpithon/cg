use crate::{nearly_equal, Cs, Matrix, Point, Ray, Shapes, I, ID_MATRIX, J, K};

pub struct Ball {
    pub cs: Cs,
    pub cam_to_lcs: Matrix,
    pub radius: f64,
}

impl Ball {
    pub fn build(center: &Point, radius: f64) -> Ball {
        assert!(radius > 0.);

        Ball {
            cs: Cs::new(center, &I, &J, &K),
            cam_to_lcs: ID_MATRIX,
            radius,
        }
    }
}

impl Shapes for Ball {
    fn set_camera_cs(&mut self, cam_to_rcs: &Matrix) {
        self.cam_to_lcs = &self.cs.rcs_to_lcs * cam_to_rcs;
    }

    fn intersect(&self, ray: &Ray) -> bool {
        let ray = Ray {
            v: &self.cam_to_lcs * &ray.v,
            o: &self.cam_to_lcs * &ray.o,
        };

        let b = 2. * (ray.v.x * ray.o.x + ray.v.y * ray.o.y + ray.v.z * ray.o.z);
        let c =
            ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z - self.radius * self.radius;
        let delta = b * b - 4. * c;

        delta >= 0.
    }
    fn intersect_min(&self, ray: &Ray) -> Option<f64> {
        let ray = Ray {
            v: &self.cam_to_lcs * &ray.v,
            o: &self.cam_to_lcs * &ray.o,
        };
        assert!(nearly_equal(ray.v.length(), 1.));

        let b = 2. * (ray.v.x * ray.o.x + ray.v.y * ray.o.y + ray.v.z * ray.o.z);
        let c =
            ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z - self.radius * self.radius;
        let delta = b * b - 4. * c;

        if delta >= 0. {
            let k = (-b - delta.sqrt()) / 2.;
            if k > 0. {
                return Some(k);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
