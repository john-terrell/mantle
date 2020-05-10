use slotmap::{SlotMap, DefaultKey};
use super::node::{Node, NodeKey};

struct Scenegraph {
    nodes: SlotMap<DefaultKey, Node>,
    root: NodeKey
}

impl Scenegraph {
    pub fn add_node(&mut self) -> NodeKey {
    }

    pub fn children(&self, node: NodeKey) -> Iter {
        
    }
}