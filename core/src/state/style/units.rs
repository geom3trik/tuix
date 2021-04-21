use crate::animation::Interpolator;

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
            Units::Stretch(val) => val,
            Units::Auto => return *end,
        };

        match end {
            Units::Pixels(e) => Units::Pixels(f32::interpolate(s, e, t)),
            Units::Percentage(e) => Units::Percentage(f32::interpolate(s, e, t)),
            Units::Stretch(e) => Units::Stretch(f32::interpolate(s, e, t)),
            Units::Auto => return *end,
        }
    }
}
