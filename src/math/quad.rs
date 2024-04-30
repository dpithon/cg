//use auto_ops::impl_op_ex;
use std::fmt;

use super::{nearly_equal, nearly_zero};

#[derive(Clone)]
pub struct Quad {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quad {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quad {
        Quad { x, y, z, w }
    }

    pub fn nearly_equal(&self, q: &Quad) -> bool {
        nearly_equal(self.x, q.x)
            && nearly_equal(self.y, q.y)
            && nearly_equal(self.z, q.z)
            && nearly_equal(self.w, q.w)
    }

    pub fn nearly_zero(&self) -> bool {
        nearly_zero(self.x) && nearly_zero(self.y) && nearly_zero(self.z) // self.w does need to be
                                                                          // tested
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

//   impl_op_ex!(*|lhs: &Quad, rhs: &Quad| -> f64 {
//       lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
//   });
//
//   impl_op_ex!(+= |lhs: &mut Quad, rhs: &Quad| {
//       lhs.x += rhs.x;
//       lhs.y += rhs.y;
//       lhs.z += rhs.z;
//   });
//
//   impl_op_ex!(-|lhs: &Quad, rhs: &Quad| -> Quad {
//       Quad::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w)
//   });
