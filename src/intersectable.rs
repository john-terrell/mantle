pub trait Intersectable {
    fn find_intersection(&self, _intesector: Intersector) -> Option<Intersection> {
        return None;
    }
}
