use std::fmt;

use crate::{Point, Vector, I, J, K, O};

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
    use crate::{Point, Vector};

    #[test]
    fn check_build_1() {
        if let Err(e) = Cs::build(CS.o, CS.i, CS.j, CS.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_2() {
        if let Err(e) = Cs::build(CS.o, 2. * CS.i, CS.j, CS.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_3() {
        if let Err(e) = Cs::build(CS.o, CS.i, 2. * CS.j, CS.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_4() {
        if let Err(e) = Cs::build(CS.o, CS.i, CS.j, 2. * CS.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_5() {
        if let Err(e) = Cs::build(CS.o, CS.j, CS.i, CS.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_6() {
        if let Err(e) = Cs::build(CS.o, CS.i, CS.k, CS.j) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_7() {
        if let Err(e) = Cs::build(CS.o, CS.k, CS.j, CS.i) {
            panic!("{e}");
        }
    }

    #[test]
    fn check_build_8() {
        let p = Point::new(1., 2., 3.);
        let i = Vector::new(36.2067, 67.43, -15.011).unit();
        let mut j = &J - i.y * &i;
        j.normalize();
        let k = &i ^ &j;

        if let Err(e) = Cs::build(p, i, j, k) {
            panic!("{e}");
        }
    }

    #[test]
    fn check_build_9() {
        let p = Point::new(1., 2., 3.);
        let i = Vector::new(5.34, -15.13, 73.22).unit();
        let mut j = &J - i.y * &i;
        j.normalize();
        let k = &i ^ &j;

        if let Err(e) = Cs::build(p, i, j, k) {
            panic!("{e}");
        }
    }

    #[test]
    fn check_build_10() {
        let p = Point::new(1., 2., 3.);
        let i = Vector::new(-75.34, -1.3, 2.73).unit();
        let mut j = &J - i.y * &i;
        j.normalize();
        let k = &i ^ &j;

        if let Err(e) = Cs::build(p, i, j, k) {
            panic!("{e}");
        }
    }
    #[test]
    fn check_default_cs() {
        let cs = Cs::default();
        assert!(cs.o.nearly_equal(&O));
        assert!(cs.i.nearly_equal(&I));
        assert!(cs.j.nearly_equal(&J));
    }
}
