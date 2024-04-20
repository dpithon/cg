use crate::geometry::{Cs, Matrix, Point, Quad, Vector, J};

pub struct Pinhole {
    pub cs: Cs,
    pub m1: Matrix,
    pub m2: Matrix,
}

impl Pinhole {
    pub fn build(location: Point, w: Vector) -> Result<Pinhole, &'static str> {
        assert!(w.is_normalized());
        assert!(!w.nearly_equal(&J));

        let n = &J - (w.y * &w);
        let inv_len_n = 1. / n.length();

        let v = inv_len_n * n;
        let u = &v ^ &w;

        let m1 = Matrix::from_columns(&u, &v, &w, &location);
        let m2 = Matrix::from_lines(
            &Quad::new(
                u.x,
                u.y,
                u.z,
                -u.x * location.x - u.y * location.y - u.z * location.z,
            ),
            &Quad::new(
                v.x,
                v.y,
                v.z,
                -v.x * location.x - v.y * location.y - v.z * location.z,
            ),
            &Quad::new(
                w.x,
                w.y,
                w.z,
                -w.x * location.x - w.y * location.y - w.z * location.z,
            ),
            &Quad::new(0., 0., 0., 1.),
        );

        Ok(Pinhole {
            cs: Cs::build(location, u, v, w)?,
            m1,
            m2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Point, SphCoord, Vector, O};

    #[test]
    fn cam_1() {
        let _ = Pinhole::build(Point::new(1., 2., 3.), Vector::new(2., 2., 2.).unit());
        let _ = Pinhole::build(Point::new(1., 2., 3.), Vector::new(-2., 1.2, 25.).unit());
        let _ = Pinhole::build(Point::new(1., 2., 3.), Vector::new(12., -23., 0.2).unit());
        let _ = Pinhole::build(Point::new(1., 2., 3.), Vector::new(2., 0.5, 22.).unit());
    }

    #[test]
    fn cam_2() {
        let mut theta: f64 = 0.;
        let step = 0.01;

        while theta < std::f64::consts::PI {
            let mut phy: f64 = 0.;
            while phy < 2. * std::f64::consts::PI {
                let w = SphCoord::build(12., theta, phy).into_vector().unit();
                Pinhole::build(O, w).expect("failed to build pinhole");
                phy += step;
            }
            theta += step;
        }
    }
}
