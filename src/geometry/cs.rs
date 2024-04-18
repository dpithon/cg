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
    pub fn build(o: Point, i: Vector, j: Vector, k: Vector) -> Cs {
        assert!((&i.length() - 1.).abs() < 0.001);
        assert!((&j.length() - 1.).abs() < 0.001);
        assert!((&k.length() - 1.).abs() < 0.001);

        assert!((&i * &j).abs() < 0.001);
        assert!((&j * &k).abs() < 0.001);
        assert!((&k * &i).abs() < 0.001);
        Cs { o, i, j, k }
    }

    pub fn check(cs: &Cs) {
        let i = &cs.j ^ &cs.k;
        let j = &cs.k ^ &cs.i;
        let k = &cs.i ^ &cs.j;

        assert!((cs.i.length() - 1.).abs() < 0.0001);
        assert!((cs.j.length() - 1.).abs() < 0.0001);
        assert!((cs.k.length() - 1.).abs() < 0.0001);

        assert!((&cs.i * &cs.j).abs() < 0.0001);
        assert!((&cs.j * &cs.k).abs() < 0.0001);
        assert!((&cs.k * &cs.i).abs() < 0.0001);

        assert!(
            (i.x - cs.i.x).abs() < 0.0001
                && (i.y - cs.i.y).abs() < 0.0001
                && (i.z - cs.i.z).abs() < 0.0001
        );

        assert!(
            (j.x - cs.j.x).abs() < 0.0001
                && (j.y - cs.j.y).abs() < 0.0001
                && (j.z - cs.j.z).abs() < 0.0001
        );

        //       assert!(
        //           (k.x - cs.k.x).abs() < 0.0001
        //        && (k.y - cs.k.y).abs() < 0.0001
        //        && (k.z - cs.k.z).abs() < 0.0001
        //       );
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
        assert!(cs.o.w == 1.);
        assert!(cs.i.x == 1.);
        assert!(cs.i.y == 0.);
        assert!(cs.i.z == 0.);
        assert!(cs.i.w == 0.);
        assert!(cs.j.x == 0.);
        assert!(cs.j.y == 1.);
        assert!(cs.j.z == 0.);
        assert!(cs.j.w == 0.);
        assert!(cs.k.x == 0.);
        assert!(cs.k.y == 0.);
        assert!(cs.k.z == 1.);
        assert!(cs.k.w == 0.);

        cs.o.x = 3.;
        assert!(cs.o.x == 3.);
    }

    #[test]
    fn check_cs_2() {
        let mut cs = Cs::default();
        assert!(cs.o.x == 0.);
        assert!(cs.o.y == 0.);
        assert!(cs.o.z == 0.);
        assert!(cs.o.w == 1.);
        assert!(cs.i.x == 1.);
        assert!(cs.i.y == 0.);
        assert!(cs.i.z == 0.);
        assert!(cs.i.w == 0.);
        assert!(cs.j.x == 0.);
        assert!(cs.j.y == 1.);
        assert!(cs.j.z == 0.);
        assert!(cs.j.w == 0.);
        assert!(cs.k.x == 0.);
        assert!(cs.k.y == 0.);
        assert!(cs.k.z == 1.);
        assert!(cs.k.w == 0.);

        cs.o.x = 3.;

        let mut cs2 = Cs::default();
        assert!(cs2.o.x == 0.);
        assert!(cs2.o.y == 0.);
        assert!(cs2.o.z == 0.);
        assert!(cs2.o.w == 1.);
        assert!(cs2.i.x == 1.);
        assert!(cs2.i.y == 0.);
        assert!(cs2.i.z == 0.);
        assert!(cs2.i.w == 0.);
        assert!(cs2.j.x == 0.);
        assert!(cs2.j.y == 1.);
        assert!(cs2.j.z == 0.);
        assert!(cs2.j.w == 0.);
        assert!(cs2.k.x == 0.);
        assert!(cs2.k.y == 0.);
        assert!(cs2.k.z == 1.);
        assert!(cs2.k.w == 0.);

        cs2.o.x = 4.;
        assert!(cs.o.x == 3.);
        assert!(cs2.o.x == 4.);
    }
}
