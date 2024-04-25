use crate::deg_to_rad;

pub enum Focale {
    Focale(f64),
    AngleDeg(f64),
    AngleRad(f64),
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
