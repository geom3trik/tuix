use crate::state::animation::Interpolator;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    None,
    Flexbox,
}

impl Default for Display {
    fn default() -> Self {
        Display::Flexbox
    }
}

impl Interpolator for Display {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return *end;
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

impl Interpolator for Visibility {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return *end;
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
