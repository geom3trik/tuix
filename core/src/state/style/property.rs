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
