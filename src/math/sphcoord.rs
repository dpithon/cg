use std::f64::consts::PI;
use std::fmt;

use crate::{Point, Vector};

const TWO_PI: f64 = 2. * PI;

pub struct SphCoord {
    pub rho: f64,
    pub theta: f64,
    pub phy: f64,
}

impl fmt::Display for SphCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SphCoord ({}, {}, {})", self.rho, self.theta, self.phy)
    }
}

impl fmt::Debug for SphCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.rho)
            .field(&self.theta)
            .field(&self.phy)
            .finish()
    }
}

impl SphCoord {
    pub fn build(rho: f64, theta: f64, phy: f64) -> SphCoord {
        assert!(rho >= 0.);
        assert!((0.0..=PI).contains(&theta));
        assert!((0.0..=TWO_PI).contains(&phy));

        SphCoord { rho, theta, phy }
    }

    // FIXME: as/into rust convention ...

    pub fn into_point(self) -> Point {
        self.as_point()
    }

    pub fn into_vector(self) -> Vector {
        self.as_vector()
    }

    pub fn as_point(&self) -> Point {
        Point::new(
            self.rho * self.theta.sin() * self.phy.sin(),
            self.rho * self.theta.cos(),
            self.rho * self.theta.sin() * self.phy.cos(),
        )
    }

    pub fn as_vector(&self) -> Vector {
        Vector::new(
            self.rho * self.theta.sin() * self.phy.sin(),
            self.rho * self.theta.cos(),
            self.rho * self.theta.sin() * self.phy.cos(),
        )
    }

    //  FIXME: Real convention below ...
    //
    //  pub fn as_point(&self) -> Point {
    //      Point::new(
    //          self.rho * self.theta.sin() * self.phy.cos(),
    //          self.rho * self.theta.sin() * self.phy.sin(),
    //          self.rho * self.theta.cos(),
    //      )
    //  }
    //   pub fn as_vector(&self) -> Vector {
    //       Vector::new(
    //           self.rho * self.theta.sin() * self.phy.cos(),
    //           self.rho * self.theta.sin() * self.phy.sin(),
    //           self.rho * self.theta.cos(),
    //       )
    //   }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_1() {
        let s = SphCoord::build(1., PI, TWO_PI);

        assert!(s.rho == 1.);
        assert!(s.theta == PI);
        assert!(s.phy == TWO_PI);
    }

    #[test]
    #[should_panic]
    fn build_2() {
        let _ = SphCoord::build(-1., PI, TWO_PI);
    }

    #[test]
    #[should_panic]
    fn build_3() {
        let _ = SphCoord::build(1., PI + 0.1, TWO_PI);
    }

    #[test]
    #[should_panic]
    fn build_4() {
        let _ = SphCoord::build(1., PI, TWO_PI + 0.1);
    }

    #[test]
    fn into_point_1() {
        let s = SphCoord::build(1., 0., 0.);
        let p = s.into_point();
        assert!(p.x == 0.);
        assert!(p.y == 1.);
        assert!(p.z == 0.);
    }

    #[test]
    fn into_point_2() {
        let s = SphCoord::build(1., PI / 4., PI / 4.);
        let p = s.into_point();
        assert!((p.y - f64::sqrt(2.0) / 2.).abs() < 0.0001);
        assert!((p.x - 0.5).abs() < 0.0001);
        assert!((p.z - 0.5).abs() < 0.0001);
    }
}
