use auto_ops::impl_op_ex;
use std::fmt;

pub trait AsQuad {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
    fn get_w(&self) -> f64;
}

pub struct Quad {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Quad {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quad {
        Quad { x, y, z, w }
    }
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl AsQuad for Quad {
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

impl_op_ex!(*|lhs: &Quad, rhs: &Quad| -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
});
