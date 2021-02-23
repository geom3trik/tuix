use crate::animation::Interpolator;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Length {
    Initial(f32),    // Default Value
    Auto,            // Let Layout System Decide
    Pixels(f32),     // Value in pixels
    Percentage(f32), // Percentage of parent
}

impl Length {
    pub fn is_auto(&self) -> bool {
        match self {
            Length::Auto => true,
            _ => false,
        }
    }

    pub fn get_value(&self, parent_length: f32) -> f32 {
        match self {
            Length::Auto => 0.0,
            Length::Initial(value) => 0.0,
            Length::Pixels(value) => *value,
            Length::Percentage(value) => *value * parent_length,
        }
    }

    pub fn get_value_or(&self, parent_length: f32, default: f32) -> f32 {
        match self {
            Length::Auto => default,
            Length::Initial(value) => default,
            Length::Pixels(value) => *value,
            Length::Percentage(value) => *value * parent_length,
        }
    }

    // pub fn set_value(&mut self, value: f32) {
    //     match self {
    //         Length::Pixels(val) => {
    //             *val = value;
    //         }

    //         Length::Percentage(val) => {
    //             *val = value;
    //         }

    //         _ => {}
    //     }
    // }
}

impl Default for Length {
    fn default() -> Self {
        Length::Auto
    }
}

impl Interpolator for Length {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let s = match start {
            Length::Pixels(val) => val,
            Length::Percentage(val) => val,
            Length::Auto => return end.clone(),
            Length::Initial(val) => val,
        };

        match end {
            Length::Pixels(e) => Length::Pixels(f32::interpolate(s, e, t)),
            Length::Percentage(e) => Length::Percentage(f32::interpolate(s, e, t)),
            Length::Auto => return end.clone(),
            Length::Initial(e) => Length::Pixels(f32::interpolate(s, e, t)),
        }
    }
}