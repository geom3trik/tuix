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
    Position(Position),
    Left(Units),
    Right(Units),
    Top(Units),
    Bottom(Units),

    // Size
    Width(Units),
    Height(Units),

    // Size Constraints
    MinWidth(Units),
    MinHeight(Units),
    MaxWidth(Units),
    MaxHeight(Units),

    // Margin
    Margin(Units),
    MarginLeft(Units),
    MarginRight(Units),
    MarginTop(Units),
    MarginBottom(Units),

    // Padding
    Padding(Units),
    PaddingLeft(Units),
    PaddingRight(Units),
    PaddingTop(Units),
    PaddingBottom(Units),

    // Flex Container
    FlexDirection(FlexDirection),
    JustifyContent(JustifyContent),
    AlignItems(AlignItems),
    AlignContent(AlignContent),

    // Flex Item
    FlexBasis(Units),
    FlexGrow(f32),
    FlexShrink(f32),
    AlignSelf(AlignSelf),

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

    TextJustify(Justify),
    TextAlign(Align),

    FontSize(f32),
    FontColor(Color),

    OuterShadow(BoxShadow),
    InnerShadow(BoxShadow),

    Transition(Vec<Transition>),

    ZIndex(i32),

    // Main Axis
    MainBefore(Units),
    MainSize(Units),
    MainAfter(Units),

    // Cross Axis
    CrossBefore(Units),
    CrossSize(Units),
    CrossAfter(Units),

    // Main Axis Align
    ChildSpace(Units),
    ChildLeft(Units),
    ChildRight(Units),
    ChildTop(Units),
    ChildBottom(Units),
    ChildBetween(Units),
}
