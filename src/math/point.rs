use auto_ops::impl_op_ex;
use std::fmt;

use crate::{Quad, Vector};

#[derive(Clone)]
pub struct Point {
    pub q: Quad,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.q.fmt(f)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.q.fmt(f)
    }
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            q: Quad { x, y, z, w: 1. },
        }
    }

    pub fn nearly_equal(&self, p: &Point) -> bool {
        self.q.nearly_equal(&p.q)
    }

    pub fn nearly_zero(&self) -> bool {
        self.q.nearly_zero()
    }
}

impl_op_ex!(+ |lhs: &Point, rhs: &Vector| -> Point {
    Point {
        q: Quad {
            x: lhs.q.x + rhs.q.x,
            y: lhs.q.y + rhs.q.y,
            z: lhs.q.z + rhs.q.z,
            w: 1.
        }
    }
});

impl_op_ex!(+= |lhs: &mut Point, rhs: &Vector| {
    lhs.q.x += rhs.q.x;
    lhs.q.y += rhs.q.y;
    lhs.q.z += rhs.q.z;
});

impl_op_ex!(-|lhs: &Point, rhs: &Point| -> Vector {
    Vector {
        q: Quad {
            x: lhs.q.x - rhs.q.x,
            y: lhs.q.y - rhs.q.y,
            z: lhs.q.z - rhs.q.z,
            w: 0.,
        },
    }
});

#[cfg(test)]
mod tests {

    use super::*;
    use crate::math::consts::*;

    #[test]
    fn point_o() {
        let p = O;
        assert!(p.q.x == 0. && p.q.y == 0. && p.q.z == 0. && p.q.w == 1.);
    }

    #[test]
    fn point_p() {
        let p = Point::new(1., 2., 3.);
        assert!(p.q.x == 1. && p.q.y == 2. && p.q.z == 3. && p.q.w == 1.);
    }
}
