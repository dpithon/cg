use std::fmt::Display;

use crate::{Point, Vector, K, O};

pub enum Heading {
    Point(Point),
    Vector(Vector),
}

pub struct Orientation {
    location: Point,
    heading: Heading,
}

pub const DEFAULT_LOCATION: Point = O;
pub const DEFAULT_HEADING: Heading = Heading::Vector(K);

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation {
            location: DEFAULT_LOCATION,
            heading: DEFAULT_HEADING,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.heading {
            Heading::Point(p) => write!(f, "location : {}\nlook_at  : {}", self.location, p),
            Heading::Vector(v) => write!(f, "location : {}\nheading  : {}", self.location, v),
        }
    }
}
impl Orientation {
    pub fn move_to(&mut self, p: Point) -> &mut Orientation {
        self.location = p;
        self
    }

    pub fn look_at(&mut self, p: Point) -> &mut Orientation {
        self.heading = Heading::Point(p);
        self
    }

    pub fn set_heading(&mut self, v: Vector) -> &mut Orientation {
        self.heading = Heading::Vector(v);
        self
    }

    pub fn get_location(&self) -> &Point {
        &self.location
    }

    pub fn compute_heading(&self) -> Vector {
        match &self.heading {
            Heading::Point(p) => (p - &self.location).unit(),
            Heading::Vector(v) => v.clone().unit(),
        }
    }
}
