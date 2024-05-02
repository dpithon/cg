use auto_ops::impl_op_ex;
use std::fmt;

use crate::{deg_to_rad, Point, Vector};

pub struct Matrix {
    m: [[f64; 4]; 4],
}

impl Matrix {
    pub fn from_lines(q1: [f64; 4], q2: [f64; 4], q3: [f64; 4], q4: [f64; 4]) -> Matrix {
        Matrix {
            m: [q1, q2, q3, q4],
        }
    }

    pub fn from_columns(q1: [f64; 4], q2: [f64; 4], q3: [f64; 4], q4: [f64; 4]) -> Matrix {
        Matrix {
            m: [
                [q1[0], q2[0], q3[0], q4[0]],
                [q1[1], q2[1], q3[1], q4[1]],
                [q1[2], q2[2], q3[2], q4[2]],
                [q1[3], q2[3], q3[3], q4[3]],
            ],
        }
    }
    pub fn get_o(&self) -> Point {
        Point::new(self.m[0][3], self.m[1][3], self.m[2][3])
    }

    pub fn get_i(&self) -> Vector {
        Vector::new(self.m[0][0], self.m[1][0], self.m[2][0])
    }

    pub fn get_j(&self) -> Vector {
        Vector::new(self.m[0][1], self.m[1][1], self.m[2][1])
    }

    pub fn get_k(&self) -> Vector {
        Vector::new(self.m[0][2], self.m[1][2], self.m[2][2])
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
                [1., 0., 0., v.x],
                [0., 1., 0., v.y],
                [0., 0., 1., v.z],
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

impl_op_ex!(*|lhs: &Matrix, rhs: &Vector| -> Vector {
    Vector::new(
        lhs.m[0][0] * rhs.x + lhs.m[0][1] * rhs.y + lhs.m[0][2] * rhs.z,
        lhs.m[1][0] * rhs.x + lhs.m[1][1] * rhs.y + lhs.m[1][2] * rhs.z,
        lhs.m[2][0] * rhs.x + lhs.m[2][1] * rhs.y + lhs.m[2][2] * rhs.z,
    )
});

impl_op_ex!(*|lhs: &Matrix, rhs: &Point| -> Point {
    Point::new(
        lhs.m[0][0] * rhs.x + lhs.m[0][1] * rhs.y + lhs.m[0][2] * rhs.z + lhs.m[0][3],
        lhs.m[1][0] * rhs.x + lhs.m[1][1] * rhs.y + lhs.m[1][2] * rhs.z + lhs.m[1][3],
        lhs.m[2][0] * rhs.x + lhs.m[2][1] * rhs.y + lhs.m[2][2] * rhs.z + lhs.m[2][3],
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
