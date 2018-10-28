use std::sync::Arc;
use std::sync::RwLock;
use group::*;
use visitable::*;
use visitor::*;

pub struct Scene {
    pub root: Arc<RwLock<Group>>
}

impl Scene {
    pub fn new() -> Scene {
        return Scene{
            root: Arc::new(RwLock::new(Group::new()))
        }
    }

    pub fn add_child(&mut self, child: Arc<RwLock<Visitable>>) {
        self.root.write().unwrap().add_child(child);
    }
}

impl Visitable for Scene {
    fn accept(&self, visitor: &Visitor) {
        // Visit the root of the scene.
        self.root.read().unwrap().accept(visitor)
    }
}
