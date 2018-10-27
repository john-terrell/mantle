use visitable::*;
use visitor::*;

pub struct Sphere {
    pub radius: f64,  
}

impl Visitable for Sphere {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit_sphere(self)
    }
}
