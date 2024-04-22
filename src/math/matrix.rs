use auto_ops::impl_op_ex;
use std::fmt;

use crate::{AsQuad, Point, Vector};

pub struct Matrix {
    m: [[f64; 4]; 4],
}

pub const ID_MATRIX: Matrix = Matrix {
    m: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ],
};

impl Matrix {
    pub fn from_lines(
        q1: &dyn AsQuad,
        q2: &dyn AsQuad,
        q3: &dyn AsQuad,
        q4: &dyn AsQuad,
    ) -> Matrix {
        Matrix {
            m: [
                [q1.get_x(), q1.get_y(), q1.get_z(), q1.get_w()],
                [q2.get_x(), q2.get_y(), q2.get_z(), q2.get_w()],
                [q3.get_x(), q3.get_y(), q3.get_z(), q3.get_w()],
                [q4.get_x(), q4.get_y(), q4.get_z(), q4.get_w()],
            ],
        }
    }

    pub fn from_columns(
        q1: &dyn AsQuad,
        q2: &dyn AsQuad,
        q3: &dyn AsQuad,
        q4: &dyn AsQuad,
    ) -> Matrix {
        Matrix {
            m: [
                [q1.get_x(), q2.get_x(), q3.get_x(), q4.get_x()],
                [q1.get_y(), q2.get_y(), q3.get_y(), q4.get_y()],
                [q1.get_z(), q2.get_z(), q3.get_z(), q4.get_z()],
                [q1.get_w(), q2.get_w(), q3.get_w(), q4.get_w()],
            ],
        }
    }

    pub fn transpose(&self) -> Matrix {
        Matrix {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ],
        }
    }
}

impl_op_ex!(*|lhs: &Matrix, rhs: &Vector| -> Vector {
    Vector::new(
        lhs.m[0][0] * rhs.x + lhs.m[0][1] * rhs.y + lhs.m[0][2] * rhs.z,
        lhs.m[1][0] * rhs.x + lhs.m[1][1] * rhs.y + lhs.m[1][2] * rhs.z,
        lhs.m[2][0] * rhs.x + lhs.m[2][1] * rhs.y + lhs.m[2][2] * rhs.z,
    )
});

impl_op_ex!(*|lhs: &Matrix, rhs: &Point| -> Point {
    Point::new(
        lhs.m[0][0] * rhs.x + lhs.m[0][1] * rhs.y + lhs.m[0][2] * rhs.z + lhs.m[0][3] * rhs.get_w(),
        lhs.m[1][0] * rhs.x + lhs.m[1][1] * rhs.y + lhs.m[1][2] * rhs.z + lhs.m[1][3] * rhs.get_w(),
        lhs.m[2][0] * rhs.x + lhs.m[2][1] * rhs.y + lhs.m[2][2] * rhs.z + lhs.m[2][3] * rhs.get_w(),
    )
});

impl_op_ex!(*|lhs: &Matrix, rhs: &Matrix| -> Matrix {
    Matrix {
        m: [
            [
                lhs.m[0][0] * rhs.m[0][0]
                    + lhs.m[0][1] * rhs.m[1][0]
                    + lhs.m[0][2] * rhs.m[2][0]
                    + lhs.m[0][3] * rhs.m[3][0],
                lhs.m[0][0] * rhs.m[0][1]
                    + lhs.m[0][1] * rhs.m[1][1]
                    + lhs.m[0][2] * rhs.m[2][1]
                    + lhs.m[0][3] * rhs.m[3][1],
                lhs.m[0][0] * rhs.m[0][2]
                    + lhs.m[0][1] * rhs.m[1][2]
                    + lhs.m[0][2] * rhs.m[2][2]
                    + lhs.m[0][3] * rhs.m[3][2],
                lhs.m[0][0] * rhs.m[0][3]
                    + lhs.m[0][1] * rhs.m[1][3]
                    + lhs.m[0][2] * rhs.m[2][3]
                    + lhs.m[0][3] * rhs.m[3][3],
            ],
            [
                lhs.m[1][0] * rhs.m[0][0]
                    + lhs.m[1][1] * rhs.m[1][0]
                    + lhs.m[1][2] * rhs.m[2][0]
                    + lhs.m[1][3] * rhs.m[3][0],
                lhs.m[1][0] * rhs.m[0][1]
                    + lhs.m[1][1] * rhs.m[1][1]
                    + lhs.m[1][2] * rhs.m[2][1]
                    + lhs.m[1][3] * rhs.m[3][1],
                lhs.m[1][0] * rhs.m[0][2]
                    + lhs.m[1][1] * rhs.m[1][2]
                    + lhs.m[1][2] * rhs.m[2][2]
                    + lhs.m[1][3] * rhs.m[3][2],
                lhs.m[1][0] * rhs.m[0][3]
                    + lhs.m[1][1] * rhs.m[1][3]
                    + lhs.m[1][2] * rhs.m[2][3]
                    + lhs.m[1][3] * rhs.m[3][3],
            ],
            [
                lhs.m[2][0] * rhs.m[0][0]
                    + lhs.m[2][1] * rhs.m[1][0]
                    + lhs.m[2][2] * rhs.m[2][0]
                    + lhs.m[2][3] * rhs.m[3][0],
                lhs.m[2][0] * rhs.m[0][1]
                    + lhs.m[2][1] * rhs.m[1][1]
                    + lhs.m[2][2] * rhs.m[2][1]
                    + lhs.m[2][3] * rhs.m[3][1],
                lhs.m[2][0] * rhs.m[0][2]
                    + lhs.m[2][1] * rhs.m[1][2]
                    + lhs.m[2][2] * rhs.m[2][2]
                    + lhs.m[2][3] * rhs.m[3][2],
                lhs.m[2][0] * rhs.m[0][3]
                    + lhs.m[2][1] * rhs.m[1][3]
                    + lhs.m[2][2] * rhs.m[2][3]
                    + lhs.m[2][3] * rhs.m[3][3],
            ],
            [
                lhs.m[3][0] * rhs.m[0][0]
                    + lhs.m[3][1] * rhs.m[1][0]
                    + lhs.m[3][2] * rhs.m[2][0]
                    + lhs.m[3][3] * rhs.m[3][0],
                lhs.m[3][0] * rhs.m[0][1]
                    + lhs.m[3][1] * rhs.m[1][1]
                    + lhs.m[3][2] * rhs.m[2][1]
                    + lhs.m[3][3] * rhs.m[3][1],
                lhs.m[3][0] * rhs.m[0][2]
                    + lhs.m[3][1] * rhs.m[1][2]
                    + lhs.m[3][2] * rhs.m[2][2]
                    + lhs.m[3][3] * rhs.m[3][2],
                lhs.m[3][0] * rhs.m[0][3]
                    + lhs.m[3][1] * rhs.m[1][3]
                    + lhs.m[3][2] * rhs.m[2][3]
                    + lhs.m[3][3] * rhs.m[3][3],
            ],
        ],
    }
});

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{:5.2} {:5.2} {:5.2} {:5.2}|\n|{:5.2} {:5.2} {:5.2} {:5.2}|\n|{:5.2} {:5.2} {:5.2} {:5.2}|\n|{:5.2} {:5.2} {:5.2} {:5.2}|",
            self.m[0][0],
            self.m[0][1],
            self.m[0][2],
            self.m[0][3],
            self.m[1][0],
            self.m[1][1],
            self.m[1][2],
            self.m[1][3],
            self.m[2][0],
            self.m[2][1],
            self.m[2][2],
            self.m[2][3],
            self.m[3][0],
            self.m[3][1],
            self.m[3][2],
            self.m[3][3]
        )
    }
}
