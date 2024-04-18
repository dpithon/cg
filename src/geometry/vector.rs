use crate::geometry::quad::AsQuad;
use auto_ops::impl_op_ex;
use core::fmt;

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub const VEC_0: Vector = Vector {
    x: 0.,
    y: 0.,
    z: 0.,
    w: 0.,
};

pub const I: Vector = Vector {
    x: 1.,
    y: 0.,
    z: 0.,
    w: 0.,
};

pub const J: Vector = Vector {
    x: 0.,
    y: 1.,
    z: 0.,
    w: 0.,
};

pub const K: Vector = Vector {
    x: 0.,
    y: 0.,
    z: 1.,
    w: 0.,
};

impl AsQuad for Vector {
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
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector")
            .field("x: ", &self.x)
            .field("y: ", &self.y)
            .field("z: ", &self.z)
            .field("w: ", &self.w)
            .finish()
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(&self) -> Vector {
        let l = self.length();
        Vector::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn unit_in_place(&mut self) -> &mut Vector {
        let l = self.length();

        self.x /= l;
        self.y /= l;
        self.z /= l;

        self
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
    use crate::geometry::point::*;

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
        assert!(a_t.x == b.x && a_t.y == b.y && a_t.z == b.z && a_t.w == 1.);
        assert!(a_again.x == a.x && a_again.y == a.y && a_again.z == a.z && a_again.w == 1.);
    }

    #[test]
    fn translate_2() {
        let mut a = Point::new(1., 2., 3.);
        let b = Point::new(3., 2., 4.);
        let p = &b - &a;
        assert!(p.x == 2. && p.y == 0. && p.z == 1. && p.w == 0.);

        a += &p;
        assert!(a.x == b.x && a.y == b.y && a.z == b.z && a.w == 1.);
    }

    #[test]
    fn add_point_1() {
        let a = Point::new(1., 2., 3.);
        let v = Vector::new(3., 2., 4.);
        let b = &a + &v;

        assert!(b.x == 4. && b.y == 4. && b.z == 7. && b.w == 1.);
        assert!(v.x == 3.);
    }

    #[test]
    fn add_point_2() {
        let mut a = Point::new(1., 2., 3.);
        let v = Vector::new(3., 2., 4.);
        a += &v;

        assert!(a.x == 4. && a.y == 4. && a.z == 7. && a.w == 1.);
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

        assert!((u.length() - 1.).abs() < 0.0001);
    }

    #[test]
    fn check_unit_2() {
        let mut v = Vector::new(12., 6., -5.);
        v.unit_in_place();

        assert!((v.length() - 1.).abs() < 0.0001);
    }
}
