use super::primitive::*;
use slotmap::*;

new_key_type! { pub struct NodeKey; }

#[derive(Copy,Clone)]
pub struct ListItem {
    prev: NodeKey,
    next: NodeKey,
}

impl ListItem {
    pub fn new() -> ListItem {
        return ListItem {
            prev: NodeKey::null(),
            next: NodeKey::null(),
        }
    }
}

#[derive(Copy,Clone)]
pub struct ChildList {
    first: NodeKey,
    last: NodeKey,
}

impl ChildList {
    pub fn new() -> ChildList {
        return ChildList {
            first: NodeKey::null(),
            last: NodeKey::null(),
        }
    }

    pub fn append_child(&mut self, old_tail: &mut Node, child: NodeKey) {
        old_tail.li.next = child;
        self.last = child
    }
}

#[derive(Copy,Clone)]
pub struct Node {
    li: ListItem,
    children: ChildList,
    primitive: Primitive,
}

impl Node {
    pub fn new(primitive: Primitive) -> Node {
        return Node {
            li: ListItem::new(),
            children: ChildList::new(),
            primitive: primitive,
        }
    }

//    fn add_child(&mut self, new_child: NodeKey) {
//        self.children.push(new_child)
//    }
}
