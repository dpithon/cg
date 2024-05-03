use auto_ops::impl_op_ex;
use std::fmt;

use crate::{nearly_equal, nearly_zero, Quad};

#[derive(Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl Quad for Vector {
    fn get_quad(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
pub fn check_base(i: &Vector, j: &Vector, k: &Vector) -> Result<(), &'static str> {
    if !i.is_normalized() {
        Err("i is not unit vector")
    } else if !j.is_normalized() {
        Err("j is not unit vector")
    } else if !k.is_normalized() {
        Err("k is not unit vector")
    } else if !i.is_normal_to(j) {
        Err("i is not normal to j")
    } else if !j.is_normal_to(k) {
        Err("j is not normal to k")
    } else if !k.is_normal_to(i) {
        Err("k is not normal to i")
    } else if !i.nearly_equal(&(j ^ k)) {
        Err("i != j ^ k")
    } else if !j.nearly_equal(&(k ^ i)) {
        Err("j != k ^ i")
    } else if !k.nearly_equal(&(i ^ j)) {
        Err("k != i ^ j")
    } else {
        Ok(())
    }
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0. }
    }

    pub fn unit(self) -> Vector {
        // TODO: unit OR normalize ??
        let len = self.length();
        Vector::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn square_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.square_length().sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if nearly_zero(len) {
            panic!("zero vector!")
        }

        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn nearly_equal(&self, v: &Vector) -> bool {
        nearly_equal(self.x, v.x) && nearly_equal(self.y, v.y) && nearly_equal(self.z, v.z)
    }

    pub fn nearly_zero(&self) -> bool {
        nearly_zero(self.x) && nearly_zero(self.y) && nearly_zero(self.z)
    }

    pub fn is_normalized(&self) -> bool {
        nearly_equal(self.square_length(), 1.)
    }

    pub fn is_normal_to(&self, v: &Vector) -> bool {
        nearly_zero(self * v)
    }
}

impl_op_ex!(+|lhs: &Vector, rhs: &Vector| -> Vector {
    Vector::new(
        lhs.x + rhs.x,
        lhs.y + rhs.y,
        lhs.z + rhs.z,
    )
});

impl_op_ex!(+= |lhs: &mut Vector, rhs: &Vector| {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    lhs.z += rhs.z;
});

impl_op_ex!(-|lhs: &Vector, rhs: &Vector| -> Vector {
    Vector::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
});

impl_op_ex!(-= |lhs: &mut Vector, rhs: &Vector| {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
    lhs.z -= rhs.z;
});

impl_op_ex!(-|v: &Vector| -> Vector { Vector::new(-v.x, -v.y, -v.z) });

impl_op_ex!(*|lhs: &Vector, rhs: &Vector| -> f64 { lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z });

impl_op_ex!(*|lhs: f64, rhs: &Vector| -> Vector {
    Vector::new(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z)
});

impl_op_ex!(*= |lhs: &mut Vector, rhs: f64| {
    lhs.x *= rhs;
    lhs.y *= rhs;
    lhs.z *= rhs;
});

impl_op_ex!(^ |lhs: &Vector, rhs: &Vector| -> Vector {
    Vector::new(
        lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.z * rhs.x - lhs.x * rhs.z,
        lhs.x * rhs.y - lhs.y * rhs.x,
    )
});

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    #[test]
    fn vector_1() {
        let p = Vector::new(1., 2., 3.);
        assert!(p.x == 1. && p.y == 2. && p.z == 3. && p.w == 0.);
    }

    #[test]
    fn vector_2() {
        let a = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.x == 2. && p.y == 0. && p.z == 1. && p.w == 0.);
    }

    #[test]
    fn translate_1() {
        let a = Point::new(1., 2., 3.);
        let a_again = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.x == 2. && p.y == 0. && p.z == 1. && p.w == 0.);

        let a_t = &a + &p;
        assert!(a_t.x == b.x && a_t.y == b.y && a_t.z == b.z);
        assert!(a_again.x == a.x && a_again.y == a.y && a_again.z == a.z);
    }

    #[test]
    fn translate_2() {
        let mut a = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.x == 2. && p.y == 0. && p.z == 1. && p.w == 0.);

        a += &p;
        assert!(a.x == b.x && a.y == b.y && a.z == b.z);
    }

    #[test]
    fn add_point_1() {
        let a = Point::new(1., 2., 3.);
        let v = Vector::new(3., 2., 4.);
        let b = &a + &v;

        assert!(b.x == 4. && b.y == 4. && b.z == 7.);
        assert!(v.x == 3.);
    }

    #[test]
    fn add_point_2() {
        let mut a = Point::new(1., 2., 3.);
        let v = Vector::new(3., 2., 4.);
        a += &v;

        assert!(a.x == 4. && a.y == 4. && a.z == 7.);
        assert!(v.x == 3.);
    }

    #[test]
    fn mul_vector_1() {
        let u = Vector::new(3., 2., 4.);
        let v = Vector::new(4., 5., 1.);
        let s = u * v;

        assert!(s == 26.);
    }

    #[test]
    fn check_base() {
        assert!(I * J == 0.);
        assert!(J * K == 0.);
        assert!(K * I == 0.);

        assert!(I.length() == 1.);
        assert!(J.length() == 1.);
        assert!(K.length() == 1.);

        let i = J ^ K;
        let j = K ^ I;
        let k = I ^ J;

        assert!(i.x == 1. && i.y == 0. && i.z == 0. && i.w == 0.);
        assert!(j.x == 0. && j.y == 1. && j.z == 0. && j.w == 0.);
        assert!(k.x == 0. && k.y == 0. && k.z == 1. && k.w == 0.);
    }

    #[test]
    fn check_unit_1() {
        let v = Vector::new(12., 6., -5.);
        let u = v.unit();

        assert!(u.is_normalized());
    }

    #[test]
    fn check_unit_2() {
        let mut v = Vector::new(12., 6., -5.);
        v.normalize();

        assert!(v.is_normalized());
    }
}
