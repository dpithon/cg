mod cameras;
mod math;

pub use math::{
    nearly_equal, nearly_zero, AsQuad, Cs, Matrix, Point, Quad, SphCoord, Vector, I, ID_MATRIX, J,
    K, O, STD_CS, VEC_0,
};

pub use cameras::{Pinhole, Ray};
