pub struct Degrees {
    value: f64
}

impl Degrees {
    fn from_degrees(value: f64) -> Degrees {
        return Degrees{value: value}
    }

    fn from_radians(radians: f64) -> Degrees {
        return Degrees{value: radians.to_degrees()}
    }

    fn in_radians(&self) -> f64 {
        return self.value.to_radians()
    }

    fn in_degrees(&self) -> f64 {
        return self.value
    }
}
