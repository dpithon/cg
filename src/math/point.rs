use auto_ops::impl_op_ex;
use std::fmt;

use crate::{nearly_equal, AsQuad, Vector};

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

pub const O: Point = Point {
    x: 0.,
    y: 0.,
    z: 0.,
    w: 1.,
};

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

impl AsQuad for Point {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_z(&self) -> f64 {
        self.z
    }
    fn get_w(&self) -> f64 {
        self.w
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1.0 }
    }

    pub fn from(p: &Point) -> Point {
        Point {
            x: p.x,
            y: p.y,
            z: p.z,
            w: p.w,
        }
    }

    pub fn get_w(&self) -> f64 {
        self.w
    }

    pub fn nearly_equal(&self, p: &Point) -> bool {
        nearly_equal(self.x, p.x)
            && nearly_equal(self.y, p.y)
            && nearly_equal(self.z, p.z)
            && nearly_equal(self.w, p.w)
    }
}

impl_op_ex!(+ |lhs: &Point, rhs: &Vector| -> Point {
    Point::new(
        lhs.x + rhs.x,
        lhs.y + rhs.y,
        lhs.z + rhs.z
    )
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

    #[test]
    fn point_o() {
        let p = O;
        assert!(p.x == 0. && p.y == 0. && p.z == 0. && p.w == 1.);
        assert_eq!(p.get_w(), 1.)
    }

    #[test]
    fn point_p() {
        let p = Point::new(1., 2., 3.);
        assert!(p.x == 1. && p.y == 2. && p.z == 3. && p.w == 1.);
        assert_eq!(p.get_w(), 1.)
    }

    fn as_quad(p: &impl AsQuad) -> (f64, f64, f64, f64) {
        (p.get_x(), p.get_y(), p.get_z(), p.get_w())
    }

    #[test]
    fn point_as_quad() {
        let p = Point::new(1., 2., 3.);
        let (x, y, z, w) = as_quad(&p);
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
        assert_eq!(p.z, z);
        assert_eq!(p.w, w);
    }
}
