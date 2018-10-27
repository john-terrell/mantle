use drawable::*;
use intersectable::*;
use visitable::*;

pub trait Node : Drawable + Intersectable + Visitable {
}
