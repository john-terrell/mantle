pub struct Intersection {
    p: Vector,      // surface position
    n: Vector,      // surface normal
    t: f64,         // intersection parameter
    intersectable: &Intersectable // the object that was intersected
}