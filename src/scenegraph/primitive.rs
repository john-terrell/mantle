use super::geometry::{Matrix44, Sphere};

#[derive(Copy,Clone)]
pub enum Primitive {
    Transformation(Matrix44),
    Sphere(Sphere),
}
