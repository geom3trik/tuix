use crate::style;
use crate::style::Display;

use crate::style::color::Color;
use crate::style::LinearGradient;

use crate::state::style::*;

use crate::state::animation::Transition;

#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    None,

    // General
    Display(Display),
    Visibility(Visibility),
    Overflow(Overflow),
    Opacity(f32),

    // Positioning
    LayoutType(LayoutType),
    PositionType(PositionType),

    // Position and Size
    Space(Units),
    Left(Units),
    Width(Units),
    Right(Units),
    Top(Units),
    Height(Units),
    Bottom(Units),

    // Constraints
    MinLeft(Units),
    MaxLeft(Units),
    MinWidth(Units),
    MaxWidth(Units),
    MinRight(Units),
    MaxRight(Units),

    MinTop(Units),
    MaxTop(Units),
    MinHeight(Units),
    MaxHeight(Units),
    MinBottom(Units),
    MaxBottom(Units),

    // Child Spacing
    ChildSpace(Units),
    ChildLeft(Units),
    ChildRight(Units),
    ChildTop(Units),
    ChildBottom(Units),
    ChildBetween(Units),

    // Border
    BorderRadius(Units),
    BorderTopLeftRadius(Units),
    BorderTopRightRadius(Units),
    BorderBottomLeftRadius(Units),
    BorderBottomRightRadius(Units),
    BorderWidth(Units),
    BorderColor(Color),

    // Background
    BackgroundColor(Color),
    BackgroundImage(String),
    BackgroundGradient(LinearGradient),

    FontSize(f32),
    FontColor(Color),

    OuterShadow(BoxShadow),
    InnerShadow(BoxShadow),

    Transition(Vec<Transition>),

    ZIndex(i32),
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::None => write!(f, ""),
            // General
            Property::Display(val) => write!(f, "display: {};", 1),
            Property::Visibility(val) => write!(f, "visibility: {};", 2),
            Property::Overflow(val) => write!(f, "overflow: {};", 3),
            Property::Opacity(val) => write!(f, "opacity: {};", val),

            // Positioning
            Property::LayoutType(val) => write!(f, "layout-type: {};", val),
            Property::PositionType(val) => write!(f, "position-type: {};", val),

            // Position and Size
            Property::Space(val) => write!(f, "space: {};", val),
            Property::Left(val) => write!(f, "left: {};", val),
            Property::Width(val) => write!(f, "width: {};", val),
            Property::Right(val) => write!(f, "right: {};", val),
            Property::Top(val) => write!(f, "top: {};", val),
            Property::Height(val) => write!(f, "height: {};", val),
            Property::Bottom(val) => write!(f, "bottom: {};", val),

            // Constraints
            Property::MinLeft(val) => write!(f, "min-left: {};", val),
            Property::MaxLeft(val) => write!(f, "max-left: {};", val),
            Property::MinWidth(val) => write!(f, "min-width: {};", val),
            Property::MaxWidth(val) => write!(f, "max-width: {};", val),
            Property::MinRight(val) => write!(f, "min-right: {};", val),
            Property::MaxRight(val) => write!(f, "max-right: {};", val),

            Property::MinTop(val) => write!(f, "min-top: {};", val),
            Property::MaxTop(val) => write!(f, "max-top: {};", val),
            Property::MinHeight(val) => write!(f, "min-height: {};", val),
            Property::MaxHeight(val) => write!(f, "max-height: {};", val),
            Property::MinBottom(val) => write!(f, "min-bottom: {};", val),
            Property::MaxBottom(val) => write!(f, "max-bottom: {};", val),

            // Child Spacing
            Property::ChildSpace(val) => write!(f, "child-space: {};", val),
            Property::ChildLeft(val) => write!(f, "child-left: {};", val),
            Property::ChildRight(val) => write!(f, "child-right: {};", val),
            Property::ChildTop(val) => write!(f, "child-top: {};", val),
            Property::ChildBottom(val) => write!(f, "child-bottom: {};", val),
            Property::ChildBetween(val) => write!(f, "child-between: {};", val),

            // Border
            Property::BorderRadius(val) => write!(f, "border-radius: {};", val),
            Property::BorderTopLeftRadius(val) => write!(f, "border-top-left-radius: {};", val),
            Property::BorderTopRightRadius(val) => write!(f, "border-top-right-radius: {};", val),
            Property::BorderBottomLeftRadius(val) => {
                write!(f, "border-bottom-left-radius: {};", val)
            }
            Property::BorderBottomRightRadius(val) => {
                write!(f, "border-bottom-right-radius: {};", val)
            }
            Property::BorderWidth(val) => write!(f, "border-width: {};", val),
            Property::BorderColor(val) => write!(f, "border-color: {:?};", val),

            // Background
            Property::BackgroundColor(val) => write!(f, "background-color: {:?};", val),
            Property::BackgroundImage(val) => write!(f, "background-image: {};", val),
            Property::BackgroundGradient(val) => write!(f, "background-gradient: {};", 4),

            Property::FontSize(val) => write!(f, "font-size: {};", val),
            Property::FontColor(val) => write!(f, "color: {:?};", val),

            Property::OuterShadow(val) => write!(f, "outer-shadow: {};", 5),
            Property::InnerShadow(val) => write!(f, "inner-shadow: {};", 6),

            Property::Transition(val) => write!(f, "transition: {:?};", val),

            Property::ZIndex(val) => write!(f, "z-index: {};", val),
        }
    }
}
