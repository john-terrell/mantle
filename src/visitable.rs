use visitor::Visitor;

pub trait Visitable {
    fn accept(&self, visitor: &Visitor);
}
