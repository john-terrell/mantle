use std::sync::Arc;
use std::sync::RwLock;

mod sphere;
use sphere::*;

mod visitable;
use visitable::*;

mod visitor;

mod group;
use group::*;

pub struct DumpNameVisitor {

}

impl visitor::Visitor for DumpNameVisitor {
    fn visit_sphere(&self, sphere: &Sphere) {
        println!("Sphere(radius: {})", sphere.radius);
    }

    fn visit_group(&self, _group: &Group) {
        println!("Group!");
    }
}

fn main() {
    let mut root = Box::new(Group::new());

    root.add_child(Arc::new(RwLock::new(Sphere { radius: 10.0, })));
    root.add_child(Arc::new(RwLock::new(Sphere { radius: 20.0, })));
    root.add_child(Arc::new(RwLock::new(Sphere { radius: 30.0, })));

    let grp = Arc::new(RwLock::new(Group::new()));

    grp.write().unwrap().add_child(Arc::new(RwLock::new(Sphere { radius: 101.0, })));

    root.add_child(grp);

    let dv = DumpNameVisitor {};

    root.accept(&dv);
}
