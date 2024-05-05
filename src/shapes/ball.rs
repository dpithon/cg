use crate::{nearly_equal, Cs, Matrix, Ray, Shapes};

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
            cam_to_lcs: Matrix::default(),
        }
    }
}

impl Shapes for Ball {
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
