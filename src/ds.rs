pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64, // TODO this could literally be a bool
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }
}