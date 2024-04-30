use crate::{nearly_equal, Cs, Matrix, Ray, Shapes, Vector, ID_MATRIX};

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

impl Shapes for Ball {
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

    fn intersect(&self, ray: &Ray) -> bool {
        let ray = Ray {
            v: &self.cam_to_lcs * &ray.v,
            o: &self.cam_to_lcs * &ray.o,
        };

        let b = 2. * (ray.v.q.x * ray.o.q.x + ray.v.q.y * ray.o.q.y + ray.v.q.z * ray.o.q.z);
        let c = ray.o.q.x * ray.o.q.x + ray.o.q.y * ray.o.q.y + ray.o.q.z * ray.o.q.z
            - self.radius * self.radius;
        let delta = b * b - 4. * c;

        delta >= 0.
    }
    fn intersect_min(&self, ray: &Ray) -> Option<f64> {
        let ray = Ray {
            v: &self.cam_to_lcs * &ray.v,
            o: &self.cam_to_lcs * &ray.o,
        };
        assert!(nearly_equal(ray.v.length(), 1.));

        let b = 2. * (ray.v.q.x * ray.o.q.x + ray.v.q.y * ray.o.q.y + ray.v.q.z * ray.o.q.z);
        let c = ray.o.q.x * ray.o.q.x + ray.o.q.y * ray.o.q.y + ray.o.q.z * ray.o.q.z
            - self.radius * self.radius;
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
