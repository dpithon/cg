use std::fmt::Display;

use crate::math::{Point, Vector};

pub struct Ray {
    o: Point,
    v: Vector,
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "o: {} v: {}", self.o, self.v)
    }
}

impl Ray {
    pub fn new(o: Point, v: Vector) -> Ray {
        Ray { o, v }
    }
}
