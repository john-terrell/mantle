pub struct Angle<T> {
    value: T
}

impl<T> Angle<T> {
    fn in_radians(&self) -> f64 {
        return self.value.in_radians();
    }

    fn in_degrees() -> f64 {
        return self.value.in_degrees();
    }
}

