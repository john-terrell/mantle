#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn add(lhs: Vector, rhs: Vector) -> Vector {
        return Vector{
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        }
    }

    pub fn subtract(lhs: Vector, rhs: Vector) -> Vector {
        return Vector{
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
        }
    }

    pub fn squared_magnitude(&self) -> f64 {
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }

    pub fn magnitude(&self) -> f64 {
        return self.squared_magnitude().sqrt();
    }

    pub fn inverse(&self) -> Vector {
        return Vector{x: -self.x, y: -self.y, z: -self.z};
    }

    pub fn normalized(&self) -> Vector {
        let scale = 1.0 / self.magnitude();
        return Vector{
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    pub fn of_magnitude(&self, magnitude: f64) -> Vector {
        let scale = magnitude / self.magnitude();
        return Vector{
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    pub fn set_length(&mut self, length: f64) {
        let mag = self.magnitude();
        let scaler = length / mag;

        self.x /= scaler;
        self.y /= scaler;
        self.z /= scaler;
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();

        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }
}
