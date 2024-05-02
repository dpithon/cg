use crate::{nearly_equal, BindToCs, Cs, Matrix, Ray, Shapes, ID_MATRIX};

pub struct Ball {
    pub cs: Cs,
    pub cam_to_lcs: Matrix,
    pub radius: f64,
}

impl Ball {
    pub fn build(radius: f64) -> Ball {
        assert!(radius > 0.);
        let cs = Cs::new();

        Ball {
            cs,
            radius,
            cam_to_lcs: ID_MATRIX,
        }
    }
}

impl BindToCs for Ball {
    fn get_cs(&self) -> &mut Cs {
        &mut self.cs
    }
}

impl Shapes for Ball {
    fn compute_camcs_to_shapecs(&mut self, cam: &crate::PinholeCamera) {
        self.cam_to_lcs = &self.cs.rcs_to_lcs * cam.get_lcs_to_rcs();
    }

    fn set_shape_cs(&mut self, cs: Cs) {
        self.cs = cs;
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
                Some(k)
            } else {
                None
            }
        } else {
            None
        }
    }
}
