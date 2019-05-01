use super::geometry::sphere::Sphere;

pub trait Visitor {
    fn visit_sphere(&self, sphere: &Sphere);
}
