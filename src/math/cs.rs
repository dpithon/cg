use std::fmt;

use crate::{nearly_equal, Matrix, Point, Quad, Vector, I, ID_MATRIX, J, K, O};

pub struct Cs {
    pub o: Point,
    pub i: Vector,
    pub j: Vector,
    pub k: Vector,
    pub lcs_to_rcs: Matrix, // local cs to reference cs
    pub rcs_to_lcs: Matrix, // reference cs to local cs
}

pub const STD_CS: Cs = Cs {
    o: O,
    i: I,
    j: J,
    k: K,
    lcs_to_rcs: ID_MATRIX,
    rcs_to_lcs: ID_MATRIX,
};

impl fmt::Display for Cs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "O:{}\nI:{}\nJ:{}\nK:{}", self.o, self.i, self.j, self.k)
    }
}

impl Default for Cs {
    fn default() -> Self {
        STD_CS
    }
}

impl Cs {
    pub fn compute_matrices(o: &Point, i: &Vector, j: &Vector, k: &Vector) -> (Matrix, Matrix) {
        (
            Matrix::from_columns(i, j, k, o),
            Matrix::from_lines(
                &Quad::new(i.x, i.y, i.z, -i.x * o.x - i.y * o.y - i.z * o.z),
                &Quad::new(j.x, j.y, j.z, -j.x * o.x - j.y * o.y - j.z * o.z),
                &Quad::new(k.x, k.y, k.z, -k.x * o.x - k.y * o.y - k.z * o.z),
                &Quad::new(0., 0., 0., 1.),
            ),
        )
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

    pub fn new(o: &Point, i: &Vector, j: &Vector, k: &Vector) -> Cs {
        let (lcs_to_rcs, rcs_to_lcs) = Cs::compute_matrices(o, i, j, k);

        Cs {
            o: Point::from(o),
            i: Vector::from(i),
            j: Vector::from(j),
            k: Vector::from(k),
            lcs_to_rcs,
            rcs_to_lcs,
        }
    }

    pub fn build(o: &Point, i: &Vector, j: &Vector, k: &Vector) -> Result<Cs, &'static str> {
        Cs::check_base(i, j, k)?;
        Ok(Cs::new(o, i, j, k))
    }

    pub fn build_from_k(o: &Point, k: &Vector) -> Cs {
        let cs: Cs;
        assert!(nearly_equal(k.length(), 1.));

        if k.nearly_equal(&J) {
            cs = Cs::new(o, &-I, &K, &J);
        } else if k.nearly_equal(&(-1. * &J)) {
            cs = Cs::new(o, &-K, &I, &-J);
        } else {
            let fact_j = &J - (k.y * k);
            let j = (1. / fact_j.length()) * fact_j;
            let i = &j ^ k;
            cs = Cs::new(o, &i, &j, k);
        }

        cs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector};

    #[test]
    fn check_build_1() {
        let cs = Cs::new(&STD_CS.o, &STD_CS.i, &STD_CS.j, &STD_CS.k);
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_2() {
        let cs = Cs::new(&STD_CS.o, &(2. * STD_CS.i), &STD_CS.j, &STD_CS.k);
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_3() {
        let cs = Cs::new(&STD_CS.o, &STD_CS.i, &(2. * STD_CS.j), &STD_CS.k);
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_4() {
        let cs = Cs::new(&STD_CS.o, &STD_CS.i, &STD_CS.j, &(2. * STD_CS.k));
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_5() {
        let cs = Cs::new(&STD_CS.o, &STD_CS.j, &STD_CS.i, &STD_CS.k);
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_6() {
        let cs = Cs::new(&STD_CS.o, &STD_CS.i, &STD_CS.k, &STD_CS.j);
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
            panic!("{e}");
        }
    }

    #[test]
    #[should_panic]
    fn check_build_7() {
        let cs = Cs::new(&STD_CS.o, &STD_CS.k, &STD_CS.j, &STD_CS.i);
        if let Err(e) = Cs::check_base(&cs.i, &cs.j, &cs.k) {
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

        let _ = Cs::new(&p, &i, &j, &k);
        if let Err(e) = Cs::check_base(&STD_CS.i, &STD_CS.j, &STD_CS.k) {
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

        let _ = Cs::new(&p, &i, &j, &k);
        if let Err(e) = Cs::check_base(&STD_CS.i, &STD_CS.j, &STD_CS.k) {
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

        let _ = Cs::new(&p, &i, &j, &k);
        if let Err(e) = Cs::check_base(&STD_CS.i, &STD_CS.j, &STD_CS.k) {
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
