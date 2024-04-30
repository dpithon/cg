use super::{Matrix, Ray, Vector};

pub trait Shapes {
    fn set_camera_cs(&mut self, cam_to_rcs: &Matrix);
    fn intersect(&self, ray: &Ray) -> bool;
    fn intersect_min(&self, ray: &Ray) -> Option<f64>;
    fn scale(&mut self, f: f64);
    fn translate(&mut self, v: &Vector);
    fn rotate_x(&mut self, deg: f64);
    fn rotate_y(&mut self, deg: f64);
    fn rotate_z(&mut self, deg: f64);
}

mod ball;
mod cylinder;

pub use ball::Ball;
pub use cylinder::Cylinder;
