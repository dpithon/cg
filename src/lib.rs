mod math;
mod pinhole;
mod ray;
mod shapes;

pub use math::{deg_to_rad, nearly_equal, nearly_zero, rad_to_deg};
pub use math::{Cs, Matrix, Point, SphCoord, Vector};
pub use math::{I, J, K, O, POINT_I, POINT_J, POINT_K, VEC_0};

pub use pinhole::{Camera, Focale};
pub use ray::Ray;
pub use shapes::{Ball, Cylinder, Shapes};
