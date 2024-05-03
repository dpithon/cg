use super::{Cs, Ray};
use crate::{Camera, Locked, Matrix};

pub trait Shapes {
    fn get_matrix_to_lcs(&self) -> &Matrix;
    fn get_matrix_to_rcs(&self) -> &Matrix;
    fn set_transform(&mut self, m: Matrix);

    fn set_shape_cs(&mut self, cs: Cs);
    fn compute_camcs_to_shapecs(&mut self, cam: &Camera<Locked>) {
        self.set_transform(self.get_matrix_to_lcs() * cam.get_matrix_to_rcs());
    }
    fn intersect(&self, ray: &Ray) -> bool;
    fn intersect_min(&self, ray: &Ray) -> Option<f64>;
}

mod ball;
mod cylinder;

pub use ball::Ball;
pub use cylinder::Cylinder;
