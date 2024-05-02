use auto_ops::impl_op_ex;
use std::fmt;

use crate::{nearly_equal, nearly_zero, Quad, Vector};

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl Quad for Point {
    fn get_quad(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1. }
    }

    pub fn nearly_equal(&self, p: &Point) -> bool {
        nearly_equal(self.x, p.x)
            && nearly_equal(self.y, p.y)
            && nearly_equal(self.z, p.z)
            && nearly_equal(self.w, p.w)
    }

    pub fn nearly_zero(&self) -> bool {
        nearly_zero(self.x) && nearly_zero(self.y) && nearly_zero(self.z) // self.w does need to be
                                                                          // tested
    }
}

impl_op_ex!(+ |lhs: &Point, rhs: &Vector| -> Point {
    Point {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
        w: 1.
    }
});

impl_op_ex!(+= |lhs: &mut Point, rhs: &Vector| {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    lhs.z += rhs.z;
});

impl_op_ex!(-|lhs: &Point, rhs: &Point| -> Vector {
    Vector::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
});

#[cfg(test)]
mod tests {

    use super::*;
    use crate::math::consts::*;

    #[test]
    fn point_o() {
        let p = O;
        assert!(p.x == 0. && p.y == 0. && p.z == 0. && p.w == 1.);
    }

    #[test]
    fn point_p() {
        let p = Point::new(1., 2., 3.);
        assert!(p.x == 1. && p.y == 2. && p.z == 3. && p.w == 1.);
    }
}
