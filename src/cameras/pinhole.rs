use crate::{Cs, Point, Vector, O, STD_CS};

pub struct Pinhole {
    location: Point,
    target: Point,
    angle: f64,

    width: u32,
    height: u32,
    pub cs: Cs,
}

pub struct PinholeIterator {
    max_x: u32,
    max_y: u32,
    xfac: f64,
    yfac: f64,
    h_2: f64,
    focale: f64,
    x: u32,
    y: u32,
}

pub struct Ray {
    o: Point,
    v: Vector,
}

pub const DEFAULT_ANGLE: f64 = 100.;

impl Pinhole {
    pub fn new(width: u32, height: u32) -> Pinhole {
        Pinhole {
            location: O,
            target: Point::new(0., 0., 1.),
            angle: DEFAULT_ANGLE,
            width,
            height,
            cs: STD_CS,
        }
    }

    pub fn move_to(&mut self, location: Point) -> &mut Pinhole {
        self.location = location;
        self
    }

    pub fn point_to(&mut self, target: Point) -> &mut Pinhole {
        self.target = target;
        self
    }

    pub fn set_view_angle(&mut self, deg: f64) -> &mut Pinhole {
        self.angle = (std::f64::consts::PI / 180.) * deg;
        self
    }

    pub fn setup(&mut self) -> &mut Pinhole {
        let direction = (&self.target - &self.location).unit();
        self.cs = Cs::build_from_k(&self.location, &direction);
        self
    }

    pub fn iter(&mut self) -> PinholeIterator {
        self.setup();

        PinholeIterator {
            max_x: self.width - 1,
            max_y: self.height - 1,
            xfac: -1. / ((self.width - 1) as f64),
            yfac: -((self.height as f64) / (self.width as f64)) / ((self.height - 1) as f64),
            h_2: (self.height as f64) / (2. * (self.width as f64)),
            focale: 1. / (2. * (self.angle / 2.).tan()),
            x: 0,
            y: 0,
        }
    }
}

impl PinholeIterator {
    fn compute(&self) -> Ray {
        let x = self.xfac * (self.x as f64) + 0.5;
        let y = self.yfac * (self.y as f64) + self.h_2;

        let o = Point::new(x, y, 0.);
        let v = (&o - Point::new(0., 0., -self.focale)).unit();

        Ray { o, v }
    }
}

impl Iterator for PinholeIterator {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.max_x && self.y == self.max_y {
            None
        } else if self.x == self.max_x {
            self.x = 0;
            self.y += 1;
            Some(self.compute())
        } else {
            self.x += 1;
            Some(self.compute())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SphCoord;

    #[test]
    fn cam_4() {
        let mut theta: f64 = 0.;
        let step = 0.01;

        while theta < std::f64::consts::PI {
            let mut phy: f64 = 0.;
            while phy < 2. * std::f64::consts::PI {
                let mut cam = Pinhole::new(320, 200);

                cam.move_to(Point::new(1., 2., 3.))
                    .point_to(SphCoord::build(12., theta, phy).into_point())
                    .set_view_angle(95.)
                    .setup();
                if let Err(e) = Cs::check_base(&cam.cs.i, &cam.cs.j, &cam.cs.k) {
                    panic!("{e}");
                }
                phy += step;
            }
            theta += step;
        }
    }

    #[test]
    fn cam_5() {
        let mut cam = Pinhole::new(320, 240);
        for ray in cam.iter() {
            println!("{} {}", ray.o, ray.v);
        }
    }
}
