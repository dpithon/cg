pub trait AsQuad {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
    fn get_w(&self) -> f64;
}

pub struct Quad {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Quad {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quad {
        Quad { x, y, z, w }
    }
}

impl AsQuad for Quad {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_z(&self) -> f64 {
        self.z
    }

    fn get_w(&self) -> f64 {
        self.w
    }
}
