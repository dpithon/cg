use auto_ops::impl_op_ex;
use std::fmt;

use crate::{deg_to_rad, Point, Quad, Vector};

pub struct Matrix {
    pub m: [[f64; 4]; 4],
}

impl Matrix {
    pub fn from_lines(q1: &Quad, q2: &Quad, q3: &Quad, q4: &Quad) -> Matrix {
        Matrix {
            m: [
                [q1.x, q1.y, q1.z, q1.w],
                [q2.x, q2.y, q2.z, q2.w],
                [q3.x, q3.y, q3.z, q3.w],
                [q4.x, q4.y, q4.z, q4.w],
            ],
        }
    }

    pub fn from_columns(q1: &Quad, q2: &Quad, q3: &Quad, q4: &Quad) -> Matrix {
        Matrix {
            m: [
                [q1.x, q2.x, q3.x, q4.x],
                [q1.y, q2.y, q3.y, q4.y],
                [q1.z, q2.z, q3.z, q4.z],
                [q1.w, q2.w, q3.w, q4.w],
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

    pub fn translation(v: &Vector) -> Matrix {
        Matrix {
            m: [
                [1., 0., 0., v.q.x],
                [0., 1., 0., v.q.y],
                [0., 0., 1., v.q.z],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn scaling(f: f64) -> Matrix {
        Matrix {
            m: [
                [f, 0., 0., 0.],
                [0., f, 0., 0.],
                [0., 0., f, 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn rotation_x(deg: f64) -> Matrix {
        let rad = deg_to_rad(deg);

        Matrix {
            m: [
                [1., 0., 0., 0.],
                [0., rad.cos(), -rad.sin(), 0.],
                [0., rad.sin(), rad.cos(), 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn rotation_y(deg: f64) -> Matrix {
        let rad = deg_to_rad(deg);

        Matrix {
            m: [
                [rad.cos(), 0., rad.sin(), 0.],
                [0., 1., 0., 0.],
                [-rad.sin(), 0., rad.cos(), 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn rotation_z(deg: f64) -> Matrix {
        let rad = deg_to_rad(deg);

        Matrix {
            m: [
                [rad.cos(), -rad.sin(), 0., 0.],
                [rad.sin(), rad.cos(), 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}

impl_op_ex!(*|lhs: &Matrix, rhs: &Vector| -> Vector {
    Vector::new(
        lhs.m[0][0] * rhs.q.x + lhs.m[0][1] * rhs.q.y + lhs.m[0][2] * rhs.q.z,
        lhs.m[1][0] * rhs.q.x + lhs.m[1][1] * rhs.q.y + lhs.m[1][2] * rhs.q.z,
        lhs.m[2][0] * rhs.q.x + lhs.m[2][1] * rhs.q.y + lhs.m[2][2] * rhs.q.z,
    )
});

impl_op_ex!(*|lhs: &Matrix, rhs: &Point| -> Point {
    Point::new(
        lhs.m[0][0] * rhs.q.x
            + lhs.m[0][1] * rhs.q.y
            + lhs.m[0][2] * rhs.q.z
            + lhs.m[0][3] * rhs.q.w,
        lhs.m[1][0] * rhs.q.x
            + lhs.m[1][1] * rhs.q.y
            + lhs.m[1][2] * rhs.q.z
            + lhs.m[1][3] * rhs.q.w,
        lhs.m[2][0] * rhs.q.x
            + lhs.m[2][1] * rhs.q.y
            + lhs.m[2][2] * rhs.q.z
            + lhs.m[2][3] * rhs.q.w,
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
