use std::fmt::Display;

// TODO: check focale value (angle in [0,180])

use crate::{deg_to_rad, rad_to_deg};

pub enum Focale {
    Focale(f64),
    AngleDeg(f64),
    AngleRad(f64),
}

pub const DEFAULT: Focale = Focale::AngleDeg(60.);
pub const DEFAULT_FOCALE: f64 = DEFAULT.get_focale();

impl Default for Focale {
    fn default() -> Focale {
        DEFAULT
    }
}

impl Display for Focale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Focale::Focale(d) => write!(f, "focale   : {d}"),
            Focale::AngleDeg(d) => write!(f, "focale   : {:.2} degrees", d),
            Focale::AngleRad(d) => write!(f, "focale   : {:.2} degrees", rad_to_deg(*d)),
        }
    }
}

impl Focale {
    pub const fn get_focale(&self) -> f64 {
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
