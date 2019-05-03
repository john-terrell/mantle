/*
pub struct DumpNameVisitor {

}

impl Visitor for DumpNameVisitor {
    fn visit_sphere(&self, sphere: &nodes::Sphere) {
        println!("Sphere(radius: {})", sphere.radius);
    }

    fn visit_group(&self, _group: &nodes::Group) {
        println!("Group!");
    }
}
*/
use slotmap::{SlotMap, DefaultKey};

mod scenegraph;
use scenegraph::node::{Node, NodeKey};

struct Scene {
    nodes: SlotMap<DefaultKey, Node>,
    root: NodeKey
}

fn main() {

}

