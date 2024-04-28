mod camera;
mod math;
mod ray;
mod shapes;

pub use math::{
    deg_to_rad, nearly_equal, nearly_zero, rad_to_deg, AsQuad, Cs, Matrix, Point, Quad, SphCoord,
    Vector, I, ID_MATRIX, J, K, O, POINT_I, POINT_J, POINT_K, STD_CS, VEC_0,
};

pub use camera::{Focale, PinholeCamera, PinholeSettings};

pub use ray::Ray;

pub use shapes::{Ball, Cylinder, Shapes};
