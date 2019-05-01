#[derive(Copy, Clone)]
pub struct Angle {
    value: f64
}

impl Angle {
    pub fn degrees(d: f64) -> Angle {
        return Angle{value: d.to_radians()};
    }

    pub fn radians(r: f64) -> Angle {
        return Angle{value: r};
    }

    pub fn in_degrees(&self) -> f64 {
        return self.value.to_degrees();
    }

    pub fn in_radians(&self) -> f64 {
        return self.value;
    }
}
