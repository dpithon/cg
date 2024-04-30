use crate::math::vector;
use crate::{nearly_equal, Matrix, Point, Quad, Vector, I, ID_MATRIX, J, K};

pub struct Cs {
    pub lcs_to_rcs: Matrix, // local cs to reference cs
    pub rcs_to_lcs: Matrix, // reference cs to local cs
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
    pub fn compute_reverse_transformation(&mut self) {
        let i = self.get_i();
        let j = self.get_j();
        let k = self.get_k();
        let o = self.get_o();

        self.rcs_to_lcs = Matrix::from_lines(
            &Quad::new(
                i.q.x,
                i.q.y,
                i.q.z,
                -i.q.x * o.q.x - i.q.y * o.q.y - i.q.z * o.q.z,
            ),
            &Quad::new(
                j.q.x,
                j.q.y,
                j.q.z,
                -j.q.x * o.q.x - j.q.y * o.q.y - j.q.z * o.q.z,
            ),
            &Quad::new(
                k.q.x,
                k.q.y,
                k.q.z,
                -k.q.x * o.q.x - k.q.y * o.q.y - k.q.z * o.q.z,
            ),
            &Quad::new(0., 0., 0., 1.),
        );
    }

    pub fn get_o(&self) -> Point {
        Point::new(
            self.lcs_to_rcs.m[0][3],
            self.lcs_to_rcs.m[1][3],
            self.lcs_to_rcs.m[2][3],
        )
    }

    pub fn get_i(&self) -> Vector {
        Vector::new(
            self.lcs_to_rcs.m[0][0],
            self.lcs_to_rcs.m[1][0],
            self.lcs_to_rcs.m[2][0],
        )
    }

    pub fn get_j(&self) -> Vector {
        Vector::new(
            self.lcs_to_rcs.m[0][1],
            self.lcs_to_rcs.m[1][1],
            self.lcs_to_rcs.m[2][1],
        )
    }

    pub fn get_k(&self) -> Vector {
        Vector::new(
            self.lcs_to_rcs.m[0][2],
            self.lcs_to_rcs.m[1][2],
            self.lcs_to_rcs.m[2][2],
        )
    }

    pub fn new() -> Cs {
        Cs {
            lcs_to_rcs: ID_MATRIX,
            rcs_to_lcs: ID_MATRIX,
        }
    }

    pub fn scale(&mut self, f: f64) {
        let mat = Matrix::scaling(f) * &self.lcs_to_rcs;
        self.lcs_to_rcs = mat;
    }

    pub fn translate(&mut self, v: &Vector) {
        let mat = Matrix::translation(v) * &self.lcs_to_rcs;
        self.lcs_to_rcs = mat;
    }

    pub fn rotate_x(&mut self, deg: f64) {
        let mat = Matrix::rotation_x(deg) * &self.lcs_to_rcs;
        self.lcs_to_rcs = mat;
    }

    pub fn rotate_y(&mut self, deg: f64) {
        let mat = Matrix::rotation_y(deg) * &self.lcs_to_rcs;
        self.lcs_to_rcs = mat;
    }

    pub fn rotate_z(&mut self, deg: f64) {
        let mat = Matrix::rotation_z(deg) * &self.lcs_to_rcs;
        self.lcs_to_rcs = mat;
    }

    pub fn set_lcs(
        &mut self,
        o: &Point,
        i: &Vector,
        j: &Vector,
        k: &Vector,
    ) -> Result<(), &'static str> {
        vector::check_base(i, j, k)?;

        self.lcs_to_rcs = Matrix::from_columns(&i.q, &j.q, &k.q, &o.q);
        Ok(())
    }

    pub fn compute_lcs_with(&mut self, o: &Point, k: &Vector) {
        assert!(nearly_equal(k.length(), 1.));

        if k.nearly_equal(&J) {
            self.set_lcs(o, &-I, &K, &J).unwrap()
        } else if k.nearly_equal(&(-1. * &J)) {
            self.set_lcs(o, &-K, &I, &-J).unwrap()
        } else {
            let j = (&J - (k.q.y * k)).unit();
            let i = &j ^ k;
            self.set_lcs(o, &i, &j, k).unwrap()
        }
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
        let mut j = &J - i.q.y * &i;
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
        let mut j = &J - i.q.y * &i;
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
        let mut j = &J - i.q.y * &i;
        j.normalize();
        let k = &i ^ &j;

        let mut cs = Cs::new();
        if let Err(e) = cs.set_lcs(&p, &i, &j, &k) {
            panic!("{e}");
        }
    }
}
