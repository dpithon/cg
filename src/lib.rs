mod math;
mod pinhole;
mod ray;
mod shapes;

pub use math::{
    check_base, deg_to_rad, nearly_equal, nearly_zero, rad_to_deg, Cs, Matrix, Point, Quad,
    SphCoord, Vector, I, ID_MATRIX, J, K, O, POINT_I, POINT_J, POINT_K, VEC_0,
};

pub use pinhole::{Camera, Focale};

pub use ray::Ray;

pub use shapes::{Ball, Cylinder, Shapes};
