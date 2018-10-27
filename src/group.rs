use std::sync::Arc;
use std::sync::RwLock;

use visitable::*;
use visitor::*;

pub struct Group {
    children: Vec<Arc<RwLock<Visitable>>>,
}

impl Group {
    pub fn new() -> Group {
        let children = Vec::new();
        Group { children: children }
    }

    pub fn add_child(&mut self, child: Arc<RwLock<Visitable>>) {
        self.children.push(child);
    }
}

impl Visitable for Group {
    fn accept(&self, visitor: &Visitor) {
        visitor.visit_group(self);

        for child in &self.children {
            child.read().unwrap().accept(visitor);
        }    
    }
}