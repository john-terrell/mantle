use super::ray::Ray;

pub struct Sphere {
    pub radius: f64,
}

pub struct SphereIntersection {
    intersections: (f64, f64),
}

impl Sphere {
    pub fn intersect(ray: Ray, sphere: Sphere) -> Option<SphereIntersection> {
        return None;
    }
}
