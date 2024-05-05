use super::quad::Quad;
use super::{nearly_equal, Matrix, Point, Vector};
use super::{I, ID_MATRIX, J, K};

pub struct Cs {
    lcs_to_rcs: Matrix, // local cs to reference cs
    rcs_to_lcs: Matrix, // reference cs to local cs
}

impl Default for Cs {
    fn default() -> Cs {
        Cs {
            lcs_to_rcs: ID_MATRIX,
            rcs_to_lcs: ID_MATRIX,
        }
    }
}

impl Cs {
    pub fn new() -> Cs {
        Cs {
            lcs_to_rcs: ID_MATRIX,
            rcs_to_lcs: ID_MATRIX,
        }
    }

    pub fn get_matrix_to_lcs(&self) -> &Matrix {
        &self.rcs_to_lcs
    }

    pub fn get_matrix_to_rcs(&self) -> &Matrix {
        &self.lcs_to_rcs
    }

    pub fn scale(&mut self, f: f64) {
        let mat1 = Matrix::scaling(f) * &self.lcs_to_rcs;
        let mat2 = &self.rcs_to_lcs * Matrix::scaling(1. / f);
        self.lcs_to_rcs = mat1;
        self.rcs_to_lcs = mat2;
    }

    pub fn translate(&mut self, v: &Vector) {
        let mat1 = Matrix::translation(v) * &self.lcs_to_rcs;
        let mat2 = &self.rcs_to_lcs * Matrix::translation(&(-v));
        self.lcs_to_rcs = mat1;
        self.rcs_to_lcs = mat2;
    }

    pub fn rotate_x(&mut self, deg: f64) {
        let mat1 = Matrix::rotation_x(deg) * &self.lcs_to_rcs;
        let mat2 = &self.rcs_to_lcs * Matrix::rotation_x(-deg);
        self.lcs_to_rcs = mat1;
        self.rcs_to_lcs = mat2;
    }

    pub fn rotate_y(&mut self, deg: f64) {
        let mat1 = Matrix::rotation_y(deg) * &self.lcs_to_rcs;
        let mat2 = &self.rcs_to_lcs * Matrix::rotation_y(-deg);
        self.lcs_to_rcs = mat1;
        self.rcs_to_lcs = mat2;
    }

    pub fn rotate_z(&mut self, deg: f64) {
        let mat1 = Matrix::rotation_z(deg) * &self.lcs_to_rcs;
        let mat2 = &self.rcs_to_lcs * Matrix::rotation_z(-deg);
        self.lcs_to_rcs = mat1;
        self.rcs_to_lcs = mat2;
    }

    pub fn complete_cs(&mut self, o: &Point, k: &Vector) {
        assert!(nearly_equal(k.length(), 1.));

        if k.nearly_equal(&J) {
            self.set_lcs(o, &-I, &K, &J).unwrap();
        } else if k.nearly_equal(&(-1. * &J)) {
            self.set_lcs(o, &-K, &I, &-J).unwrap();
        } else {
            let j = (&J - (k.y * k)).unit();
            let i = &j ^ k;
            self.set_lcs(o, &i, &j, k).unwrap();
        }
        self.compute_reverse_base();
    }

    fn set_lcs(
        &mut self,
        o: &Point,
        i: &Vector,
        j: &Vector,
        k: &Vector,
    ) -> Result<(), &'static str> {
        Vector::check_base(i, j, k)?;

        self.lcs_to_rcs =
            Matrix::from_columns(i.get_quad(), j.get_quad(), k.get_quad(), o.get_quad());

        Ok(())
    }

    fn compute_reverse_base(&mut self) {
        // FIXME: Works only for unit vectors !
        let i = self.lcs_to_rcs.get_i();
        let j = self.lcs_to_rcs.get_j();
        let k = self.lcs_to_rcs.get_k();
        let o = self.lcs_to_rcs.get_o();

        self.rcs_to_lcs = Matrix::from_lines(
            [i.x, i.y, i.z, -i.x * o.x - i.y * o.y - i.z * o.z],
            [j.x, j.y, j.z, -j.x * o.x - j.y * o.y - j.z * o.z],
            [k.x, k.y, k.z, -k.x * o.x - k.y * o.y - k.z * o.z],
            [0., 0., 0., 1.],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector};

    #[test]
    fn check_build_8() {
        let p = Point::new(1., 2., 3.);
        let i = Vector::new(36.2067, 67.43, -15.011).unit();
        let mut j = &J - i.y * &i;
        j.normalize();
        let k = &i ^ &j;

        let mut cs = Cs::new();
        if let Err(e) = cs.set_lcs(&p, &i, &j, &k) {
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

        let mut cs = Cs::new();
        if let Err(e) = cs.set_lcs(&p, &i, &j, &k) {
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

        let mut cs = Cs::new();
        if let Err(e) = cs.set_lcs(&p, &i, &j, &k) {
            panic!("{e}");
        }
    }
}
