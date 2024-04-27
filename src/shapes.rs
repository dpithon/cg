use super::{Matrix, Ray};

pub trait Shapes {
    fn set_camera_cs(&mut self, cam_to_rcs: &Matrix);
    fn intersect(&self, ray: &Ray) -> bool;
    fn intersect_min(&self, ray: &Ray) -> Option<f64>;
}

mod ball;

pub use ball::Ball;
