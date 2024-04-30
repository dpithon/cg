use super::{Cs, Ray};
use crate::PinholeCamera;

pub trait Shapes {
    fn set_shape_cs(&mut self, cs: Cs);
    fn compute_camcs_to_shapecs(&mut self, cam: &PinholeCamera);
    fn intersect(&self, ray: &Ray) -> bool;
    fn intersect_min(&self, ray: &Ray) -> Option<f64>;
}

mod ball;
mod cylinder;

pub use ball::Ball;
pub use cylinder::Cylinder;
