use super::ImageSize;
use crate::{Point, Ray, Vector};

pub struct Sampler {
    focale: f64,
    max_x: u32,
    max_y: u32,
    fac_x: f64,
    fac_y: f64,
    hlf_h: f64,
    x: u32,
    y: u32,
}

impl Sampler {
    pub fn new(size: &ImageSize, focale: f64) -> Sampler {
        let h = (size.height as f64) / (size.width as f64);
        let max_x = size.width - 1;
        let max_y = size.height - 1;

        Sampler {
            focale,
            max_x,
            max_y,
            fac_x: -1. / ((size.width - 1) as f64),
            fac_y: -h / ((size.height - 1) as f64),
            hlf_h: h / 2.,
            x: 0,
            y: 0,
        }
    }

    fn convert(&self) -> Ray {
        let x = self.fac_x * (self.x as f64) + 0.5;
        let y = self.fac_y * (self.y as f64) + self.hlf_h;

        Ray::new(Point::new(x, y, 0.), Vector::new(x, y, self.focale).unit()) // TODO: document FM
                                                                              // ray
    }
}

impl Iterator for Sampler {
    type Item = (u32, u32, Ray);

    fn next(&mut self) -> Option<Self::Item> {
        let ret;

        if self.x == self.max_x && self.y == self.max_y {
            ret = None;
        } else if self.x == self.max_x {
            ret = Some((self.x, self.y, self.convert()));
            self.x = 0;
            self.y += 1;
        } else {
            ret = Some((self.x, self.y, self.convert()));
            self.x += 1;
        }
        ret
    }
}
