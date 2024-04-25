use std::fmt::Display;

use super::Focale;
use super::ImageSize;

use crate::rad_to_deg;
use crate::{Point, Vector};
use crate::{O, POINT_K};

pub const DEFAULT_LOCATION: Point = O;
pub const DEFAULT_TARGET: Point = POINT_K;
pub const DEFAULT_WIDTH: u32 = 320;
pub const DEFAULT_HEIGHT: u32 = 240;
pub const DEFAULT_ANGLE: f64 = 90.; // angle = 90.

pub enum Heading {
    Point(Point),
    Vector(Vector),
}

pub struct PinholeSettings {
    pub location: Point,
    pub heading: Heading,
    pub image_size: ImageSize,
    pub focale: Focale,
}

impl Default for PinholeSettings {
    fn default() -> Self {
        PinholeSettings {
            location: DEFAULT_LOCATION,
            heading: Heading::Point(DEFAULT_TARGET),
            image_size: ImageSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            focale: Focale::AngleDeg(DEFAULT_ANGLE),
        }
    }
}

impl Display for PinholeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "location : {}\n{}\n{}\n{}",
            self.location, self.heading, self.focale, self.image_size
        )
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
impl Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading::Point(p) => write!(f, "look at  : {}", p),
            Heading::Vector(v) => write!(f, "heading  : {}", v),
        }
    }
}
impl PinholeSettings {
    pub fn new() -> PinholeSettings {
        PinholeSettings::default()
    }

    pub fn move_to(&mut self, location: Point) {
        self.location = location;
    }

    pub fn point_to(&mut self, target: Point) {
        self.heading = Heading::Point(target);
    }

    pub fn set_heading(&mut self, heading: Vector) {
        self.heading = Heading::Vector(heading);
    }

    pub fn set_focale(&mut self, focale: Focale) {
        self.focale = focale;
    }

    pub fn set_image_size(&mut self, width: u32, height: u32) {
        self.image_size = ImageSize::new(width, height);
    }

    pub fn get_heading(&self) -> Vector {
        match &self.heading {
            Heading::Point(p) => (p - &self.location).unit(),
            Heading::Vector(v) => Vector::from(v).unit(),
        }
    }
}
