use crate::state::animator::Interpolator;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    None,
    Normal,
    Flexbox,
    Grid,
}

impl Default for Display {
    fn default() -> Self {
        Display::Normal
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Visibility {
    Visible,
    Invisible,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Opacity(pub f32);

impl Default for Opacity {
    fn default() -> Self {
        Opacity(1.0)
    }
}

impl Interpolator for Opacity {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return Opacity(start.0 + (end.0 - start.0) * t);
    }
}
