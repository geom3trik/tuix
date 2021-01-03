// Rename to Transform when I've come up with a better name for the current transform mod
use crate::state::animator::Interpolator;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Scale(pub f32);

impl Scale {
    pub fn new(scale: f32) -> Self {
        Scale(scale)
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale(1.0)
    }
}

impl Interpolator for Scale {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return Scale(start.0 + (end.0 - start.0) * t);
    }
}
