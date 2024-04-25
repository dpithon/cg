use std::fmt::Display;

use crate::{deg_to_rad, rad_to_deg};

pub enum Focale {
    Focale(f64),
    AngleDeg(f64),
    AngleRad(f64),
}

pub const DEFAULT_ANGLE: f64 = 90.; // angle = 90.

impl Default for Focale {
    fn default() -> Focale {
        Focale::Focale(DEFAULT_ANGLE)
    }
}

impl Display for Focale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Focale::Focale(d) => write!(f, "focale   : {d}"),
            Focale::AngleDeg(d) => write!(f, "focale   : {:.2}", d),
            Focale::AngleRad(d) => write!(f, "focale   : {:.2}", rad_to_deg(*d)),
        }
    }
}

impl Focale {
    pub fn get_focale(&self) -> f64 {
        match self {
            Focale::AngleDeg(deg) => {
                let rad = deg_to_rad(*deg);
                1. / (2. * (rad / 2.).tan())
            }
            Focale::AngleRad(rad) => 1. / (2. * (rad / 2.).tan()),
            Focale::Focale(focale) => *focale,
        }
    }
}
