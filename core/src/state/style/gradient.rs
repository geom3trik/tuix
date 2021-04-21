use crate::entity::Entity;

use crate::state::animation::Interpolator;
use crate::style::color::Color;

use crate::style::Units;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GradientStop {
    // Position of the gradient stop
    // TODO - it doesn't make sense for this to be in Units
    pub position: Units,
    // Colour of the gradient stop
    pub color: Color,
}

impl GradientStop {
    pub fn new(position: Units, color: Color) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::LeftToRight
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LinearGradient {
    // Direction of the gradient
    pub direction: Direction,
    // Stops of the gradient
    pub stops: Vec<GradientStop>,
}

impl LinearGradient {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            stops: Vec::new(),
        }
    }

    pub fn add_stop(mut self, stop: GradientStop) -> Self {
        self.stops.push(stop);

        self
    }

    pub fn get_stops(&mut self, parent_Units: f32) -> Vec<(f32, Color)> {
        self.stops
            .iter()
            .map(|stop| (stop.position.get_value(parent_Units), stop.color))
            .collect::<Vec<_>>()
    }
}
