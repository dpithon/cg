use core::fmt;

use crate::geometry::quad::AsQuad;
use crate::geometry::vector::Vector;
use auto_ops::impl_op_ex;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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
    fn point_1() {
        let p = Point::new(1., 2., 3.);
        assert!(p.x == 1. && p.y == 2. && p.z == 3. && p.w == 1.);
    }
}
