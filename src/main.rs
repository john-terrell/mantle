use std::sync::Arc;
use std::sync::RwLock;

mod sphere;
use sphere::*;

mod visitable;
use visitable::*;

mod visitor;

mod group;
use group::*;

mod scene;
use scene::*;

mod geometry;
use geometry::angle::*;

mod camera;

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

    let a = Angle::degrees(180.0);

    println!("Radians: {}", a.in_radians());
    println!("Degrees: {}", a.in_degrees());

    let mut scene = Box::new(Scene::new());

    scene.add_child(Arc::new(RwLock::new(Sphere { radius: 10.0, })));
    scene.add_child(Arc::new(RwLock::new(Sphere { radius: 20.0, })));
    scene.add_child(Arc::new(RwLock::new(Sphere { radius: 30.0, })));

    let grp = Arc::new(RwLock::new(Group::new()));

    grp.write().unwrap().add_child(Arc::new(RwLock::new(Sphere { radius: 101.0, })));

    scene.add_child(grp);

    let dv = DumpNameVisitor {};

    scene.accept(&dv);
}
