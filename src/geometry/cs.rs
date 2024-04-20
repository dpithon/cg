use core::fmt;

use crate::geometry::point::{Point, O};
use crate::geometry::vector::{Vector, I, J, K};

pub struct Cs {
    pub o: Point,
    pub i: Vector,
    pub j: Vector,
    pub k: Vector,
}

pub const CS: Cs = Cs {
    o: O,
    i: I,
    j: J,
    k: K,
};

impl fmt::Display for Cs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "O:{}\nI:{}\nJ:{}\nK:{}", self.o, self.i, self.j, self.k)
    }
}

impl Default for Cs {
    fn default() -> Self {
        CS
    }
}

impl Cs {
    pub fn build(o: Point, i: Vector, j: Vector, k: Vector) -> Result<Cs, &'static str> {
        if !i.is_normalized() {
            Err("i is not unit vector")
        } else if !j.is_normalized() {
            Err("j is not unit vector")
        } else if !k.is_normalized() {
            Err("k is not unit vector")
        } else if !i.is_normal_to(&j) {
            eprintln!("{} * {}: {:.20}", &i, &j, &i * &j);
            Err("i is not normal to j")
        } else if !j.is_normal_to(&k) {
            Err("j is not normal to k")
        } else if !k.is_normal_to(&i) {
            Err("k is not normal to i")
        } else if !i.nearly_equal(&(&j ^ &k)) {
            Err("i != j ^ k")
        } else if !j.nearly_equal(&(&k ^ &i)) {
            Err("j != k ^ i")
        } else if !k.nearly_equal(&(&i ^ &j)) {
            Err("k != i ^ j")
        } else {
            Ok(Cs { o, i, j, k })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_cs_1() {
        let mut cs = CS;
        assert!(cs.o.x == 0.);
        assert!(cs.o.y == 0.);
        assert!(cs.o.z == 0.);
        assert!(cs.o.get_w() == 1.);
        assert!(cs.i.x == 1.);
        assert!(cs.i.y == 0.);
        assert!(cs.i.z == 0.);
        assert!(cs.i.get_w() == 0.);
        assert!(cs.j.x == 0.);
        assert!(cs.j.y == 1.);
        assert!(cs.j.z == 0.);
        assert!(cs.j.get_w() == 0.);
        assert!(cs.k.x == 0.);
        assert!(cs.k.y == 0.);
        assert!(cs.k.z == 1.);
        assert!(cs.k.get_w() == 0.);

        cs.o.x = 3.;
        assert!(cs.o.x == 3.);
    }

    #[test]
    fn check_cs_2() {
        let mut cs = Cs::default();
        assert!(cs.o.x == 0.);
        assert!(cs.o.y == 0.);
        assert!(cs.o.z == 0.);
        assert!(cs.o.get_w() == 1.);
        assert!(cs.i.x == 1.);
        assert!(cs.i.y == 0.);
        assert!(cs.i.z == 0.);
        assert!(cs.i.get_w() == 0.);
        assert!(cs.j.x == 0.);
        assert!(cs.j.y == 1.);
        assert!(cs.j.z == 0.);
        assert!(cs.j.get_w() == 0.);
        assert!(cs.k.x == 0.);
        assert!(cs.k.y == 0.);
        assert!(cs.k.z == 1.);
        assert!(cs.k.get_w() == 0.);

        cs.o.x = 3.;

        let mut cs2 = Cs::default();
        assert!(cs2.o.x == 0.);
        assert!(cs2.o.y == 0.);
        assert!(cs2.o.z == 0.);
        assert!(cs2.o.get_w() == 1.);
        assert!(cs2.i.x == 1.);
        assert!(cs2.i.y == 0.);
        assert!(cs2.i.z == 0.);
        assert!(cs2.i.get_w() == 0.);
        assert!(cs2.j.x == 0.);
        assert!(cs2.j.y == 1.);
        assert!(cs2.j.z == 0.);
        assert!(cs2.j.get_w() == 0.);
        assert!(cs2.k.x == 0.);
        assert!(cs2.k.y == 0.);
        assert!(cs2.k.z == 1.);
        assert!(cs2.k.get_w() == 0.);

        cs2.o.x = 4.;
        assert!(cs.o.x == 3.);
        assert!(cs2.o.x == 4.);
    }
}
