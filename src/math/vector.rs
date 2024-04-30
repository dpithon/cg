use auto_ops::impl_op_ex;
use std::fmt;

use crate::{nearly_equal, nearly_zero, Quad};

#[derive(Clone)]
pub struct Vector {
    pub q: Quad,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.q.fmt(f)
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.q.fmt(f)
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
        Vector {
            q: Quad { x, y, z, w: 0. },
        }
    }

    pub fn unit(self) -> Vector {
        // TODO: unit OR normalize ??
        let len = self.length();
        Vector::new(self.q.x / len, self.q.y / len, self.q.z / len)
    }

    pub fn square_length(&self) -> f64 {
        self.q.x * self.q.x + self.q.y * self.q.y + self.q.z * self.q.z
    }

    pub fn length(&self) -> f64 {
        self.square_length().sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();

        self.q.x /= len;
        self.q.y /= len;
        self.q.z /= len;
    }

    pub fn nearly_equal(&self, v: &Vector) -> bool {
        self.q.nearly_equal(&v.q)
    }

    pub fn nearly_zero(&self) -> bool {
        self.q.nearly_zero()
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
        lhs.q.x + rhs.q.x,
        lhs.q.y + rhs.q.y,
        lhs.q.z + rhs.q.z,
    )
});

impl_op_ex!(+= |lhs: &mut Vector, rhs: &Vector| {
    lhs.q.x += rhs.q.x;
    lhs.q.y += rhs.q.y;
    lhs.q.z += rhs.q.z;
});

impl_op_ex!(-|lhs: &Vector, rhs: &Vector| -> Vector {
    Vector::new(lhs.q.x - rhs.q.x, lhs.q.y - rhs.q.y, lhs.q.z - rhs.q.z)
});

impl_op_ex!(-= |lhs: &mut Vector, rhs: &Vector| {
    lhs.q.x -= rhs.q.x;
    lhs.q.y -= rhs.q.y;
    lhs.q.z -= rhs.q.z;
});

impl_op_ex!(-|v: &Vector| -> Vector { Vector::new(-v.q.x, -v.q.y, -v.q.z) });

impl_op_ex!(*|lhs: &Vector, rhs: &Vector| -> f64 {
    lhs.q.x * rhs.q.x + lhs.q.y * rhs.q.y + lhs.q.z * rhs.q.z
});

impl_op_ex!(*|lhs: f64, rhs: &Vector| -> Vector {
    Vector::new(lhs * rhs.q.x, lhs * rhs.q.y, lhs * rhs.q.z)
});

impl_op_ex!(*= |lhs: &mut Vector, rhs: f64| {
    lhs.q.x *= rhs;
    lhs.q.y *= rhs;
    lhs.q.z *= rhs;
});

impl_op_ex!(^ |lhs: &Vector, rhs: &Vector| -> Vector {
    Vector::new(
        lhs.q.y * rhs.q.z - lhs.q.z * rhs.q.y,
        lhs.q.z * rhs.q.x - lhs.q.x * rhs.q.z,
        lhs.q.x * rhs.q.y - lhs.q.y * rhs.q.x,
    )
});

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    #[test]
    fn vector_1() {
        let p = Vector::new(1., 2., 3.);
        assert!(p.q.x == 1. && p.q.y == 2. && p.q.z == 3. && p.q.w == 0.);
    }

    #[test]
    fn vector_2() {
        let a = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.q.x == 2. && p.q.y == 0. && p.q.z == 1. && p.q.w == 0.);
    }

    #[test]
    fn translate_1() {
        let a = Point::new(1., 2., 3.);
        let a_again = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.q.x == 2. && p.q.y == 0. && p.q.z == 1. && p.q.w == 0.);

        let a_t = &a + &p;
        assert!(a_t.q.x == b.q.x && a_t.q.y == b.q.y && a_t.q.z == b.q.z && a_t.q.w == 1.);
        assert!(
            a_again.q.x == a.q.x
                && a_again.q.y == a.q.y
                && a_again.q.z == a.q.z
                && a_again.q.w == 1.
        );
    }

    #[test]
    fn translate_2() {
        let mut a = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.q.x == 2. && p.q.y == 0. && p.q.z == 1. && p.q.w == 0.);

        a += &p;
        assert!(a.q.x == b.q.x && a.q.y == b.q.y && a.q.z == b.q.z && a.q.w == 1.);
    }

    #[test]
    fn add_point_1() {
        let a = Point::new(1., 2., 3.);
        let v = Vector::new(3., 2., 4.);
        let b = &a + &v;

        assert!(b.q.x == 4. && b.q.y == 4. && b.q.z == 7. && b.q.w == 1.);
        assert!(v.q.x == 3.);
    }

    #[test]
    fn add_point_2() {
        let mut a = Point::new(1., 2., 3.);
        let v = Vector::new(3., 2., 4.);
        a += &v;

        assert!(a.q.x == 4. && a.q.y == 4. && a.q.z == 7. && a.q.w == 1.);
        assert!(v.q.x == 3.);
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

        assert!(i.q.x == 1. && i.q.y == 0. && i.q.z == 0. && i.q.w == 0.);
        assert!(j.q.x == 0. && j.q.y == 1. && j.q.z == 0. && j.q.w == 0.);
        assert!(k.q.x == 0. && k.q.y == 0. && k.q.z == 1. && k.q.w == 0.);
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
