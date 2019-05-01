use super::visitor::Visitor;
use slotmap::*;

new_key_type! { pub struct PrimitiveKey; }

pub trait Primitive {
    fn accept(&self, visitor: &Visitor);
}
