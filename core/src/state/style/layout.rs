use crate::animation::Interpolator;
use crate::entity::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaxWidth(pub f32);

impl Default for MaxWidth {
    fn default() -> Self {
        MaxWidth(std::f32::INFINITY)
    }
}

impl Interpolator for MaxWidth {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        MaxWidth(start.0 + (end.0 - start.0) * t)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaxHeight(pub f32);

impl Default for MaxHeight {
    fn default() -> Self {
        MaxHeight(std::f32::INFINITY)
    }
}

impl Interpolator for MaxHeight {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        MaxHeight(start.0 + (end.0 - start.0) * t)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    Relative,
    Absolute,
}

impl Default for Position {
    fn default() -> Self {
        Position::Relative
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Justify {
    Start,
    Center,
    End,
}

impl Default for Justify {
    fn default() -> Self {
        Justify::Start
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Default for Align {
    fn default() -> Self {
        Align::Center
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifySelf {
    Start,
    End,
    Center,
    Stretch,
}

impl Default for JustifySelf {
    fn default() -> Self {
        JustifySelf::Start
    }
}

// Not currently used

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Hidden
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Scroll {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Default for Scroll {
    fn default() -> Self {
        Scroll {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }
}

// Experimental new layout system
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Units {
    Auto,
    Pixels(f32),
    Percentage(f32),
    Stretch(f32),
}

impl Units {

    pub fn is_auto(&self) -> bool {
        match self {
            Units::Auto => true,
            _ => false,
        }
    }

    pub fn get_value(&self, parent_length: f32) -> f32 {
        match self {
            Units::Pixels(value) => *value,
            Units::Percentage(value) => *value * parent_length,
            _ => 0.0,
        }
    }

    pub fn get_value_or(&self, parent_length: f32, default: f32) -> f32 {
        match self {
            Units::Pixels(value) => *value,
            Units::Percentage(value) => *value * parent_length,
            _ => default,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for Units {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Auto => {
                write!(fmt, "Auto")
            }

            Self::Pixels(val) => {
                write!(fmt, "{}px", val)
            }

            Self::Percentage(val) => {
                write!(fmt, "{}%", val)
            }

            Self::Stretch(val) => {
                write!(fmt, "{}s", val)
            }
        }
    }
}

impl Interpolator for Units {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let s = match start {
            Units::Pixels(val) => val,
            Units::Percentage(val) => val,
            Units::Stretch(_) => return *end,
            Units::Auto => return *end,
        };

        match end {
            Units::Pixels(e) => Units::Pixels(f32::interpolate(s, e, t)),
            Units::Percentage(e) => Units::Percentage(f32::interpolate(s, e, t)),
            Units::Stretch(_) => return *end,
            Units::Auto => return *end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    pub space_before: Units,
    pub size: Units,
    pub space_after: Units,
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            space_before: Units::Auto,
            size: Units::Stretch(1.0),
            space_after: Units::Auto,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxisAlign {
    pub space_before_first: Units,
    pub space_between: Units,
    pub space_after_last: Units,
}

impl Default for AxisAlign {
    fn default() -> Self {
        Self {
            space_before_first: Units::Stretch(1.0),
            space_between: Units::Stretch(1.0),
            space_after_last: Units::Stretch(1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    None,
    Horizontal,
    Vertical,
    Grid,
}

impl Default for LayoutType {
    fn default() -> Self {
        Self::Vertical
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum PositioningType {
    SelfDirected,
    ParentDirected,
}

impl Default for PositioningType {
    fn default() -> Self {
        Self::ParentDirected
    }
}


#[derive(Default, Debug, Clone, PartialEq)]
pub struct GridAxis {
    pub items: Vec<Units>,
    pub align: AxisAlign,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GridItem {
    pub row_index: u32,
    pub row_span: u32,
    pub col_index: u32,
    pub col_span: u32,
}
