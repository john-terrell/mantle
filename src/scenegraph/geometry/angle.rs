use num_traits::*;

pub struct Degrees<T: Float> {
    value: T
}

impl<T: Float> Degrees<T> {
    fn to_radians(&self) -> T {
        return self.value.to_radians();
    }

    fn to_degrees(&self) -> T {
        return self.value;
    }    
}

pub struct Radians<T: Float> {
    value: T
}

impl<T: Float> Radians<T> {
    fn to_radians(&self) -> T {
        return self.value;
    }

    fn to_degrees(&self) -> T {
        return self.value.to_degrees();
    }    
}
