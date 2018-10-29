use super::vector::Vector;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o: Vector,
    pub d: Vector
}

impl Ray {
    pub fn parameterize(&self, t: f64) -> Vector {
        return Vector{ 
            x: self.o.x + (self.d.x * t), 
            y: self.o.y + (self.d.y * t), 
            z: self.o.z + (self.d.z * t), 
            };
    }
}