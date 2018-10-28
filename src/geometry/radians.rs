use super::degrees::Degrees;

pub struct Radians {
    value: f64
}

impl Radians {
    fn from_degrees(&self, value: f64) -> Radians {
        return Radians{value: value.to_radians()}
    }

    fn from_radians(radians: f64) -> Radians {
        return Radians{value: radians}
    }

    fn in_radians(&self) -> f64 {
        return self.value
    }

    fn in_degrees(&self) -> f64 {
        return self.value.to_degrees()
    }
}
