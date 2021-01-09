// TODO - Rename to paint maybe?

use crate::entity::Entity;

use crate::state::animator::Interpolator;
use crate::style::color::Color;

use crate::style::Length;

// TODO
/*
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BorderStyle {
    Solid,
    None,
    Hidden,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::Solid
    }
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorderRadius {
    pub top_left: Length,
    pub top_right: Length,
    pub bottom_left: Length,
    pub bottom_right: Length,
}

impl Default for BorderRadius {
    fn default() -> Self {
        BorderRadius {
            top_left: Length::default(),
            top_right: Length::default(),
            bottom_left: Length::default(),
            bottom_right: Length::default(),
        }
    }
}

impl Interpolator for BorderRadius {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        BorderRadius {
            top_left: Length::interpolate(&start.top_left, &end.top_left, t),
            top_right: Length::interpolate(&start.top_right, &end.top_right, t),
            bottom_left: Length::interpolate(&start.bottom_left, &end.bottom_left, t),
            bottom_right: Length::interpolate(&start.bottom_right, &end.bottom_right, t),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BoxShadow {
    pub horizontal_offset: Length,
    pub vertical_offset: Length,
    pub blur_radius: Length,
    pub color: Color,
}

impl Default for BoxShadow {
    fn default() -> Self {
        BoxShadow {
            horizontal_offset: Length::Auto,
            vertical_offset: Length::Auto,
            blur_radius: Length::Auto,
            color: Color::rgba(0, 0, 0, 128),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ClipArea {
    clip_posx: f32,
    clip_posy: f32,
    clip_width: f32,
    clip_height: f32,
}

impl Default for ClipArea {
    fn default() -> Self {
        ClipArea {
            clip_posx: 0.0,
            clip_posy: 0.0,
            clip_width: 100.0,
            clip_height: 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FocusOrder {
    pub next: Entity,
    pub prev: Entity,
}

impl Default for FocusOrder {
    fn default() -> Self {
        FocusOrder {
            next: Entity::null(),
            prev: Entity::null(),
        }
    }
}
