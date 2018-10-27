use sphere::Sphere;
use group::Group;

pub trait Visitor {
    fn visit_sphere(&self, sphere: &Sphere);
    fn visit_group(&self, group: &Group);
}
