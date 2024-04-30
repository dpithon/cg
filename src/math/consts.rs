use super::{Cs, Matrix, Point, Vector};

pub const POINT_I: Point = Point::new(1., 0., 0.);
pub const POINT_J: Point = Point::new(0., 1., 0.);
pub const POINT_K: Point = Point::new(0., 0., 1.);
pub const O: Point = Point::new(0., 0., 0.);
pub const I: Vector = Vector::new(1., 0., 0.);
pub const J: Vector = Vector::new(0., 1., 0.);
pub const K: Vector = Vector::new(0., 0., 1.);
pub const VEC_0: Vector = Vector::new(0., 0., 0.);
pub const ID_MATRIX: Matrix = Matrix {
    m: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ],
};
pub const STD_CS: Cs = Cs {
    o: O,
    i: I,
    j: J,
    k: K,
    lcs_to_rcs: ID_MATRIX,
    rcs_to_lcs: ID_MATRIX,
};
