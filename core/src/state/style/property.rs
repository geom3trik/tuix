use crate::style;
use crate::style::Display;

use crate::style::color::Color;
use crate::style::LinearGradient;

use crate::state::style::*;

use crate::state::animation::Transition;

#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    None,

    Animation(String),
    AnimationDuration(std::time::Duration),

    // General
    Display(Display),
    Visibility(Visibility),
    Overflow(Overflow),
    Opacity(f32),

    // Positioning
    Position(Position),
    Left(Length),
    Right(Length),
    Top(Length),
    Bottom(Length),

    // Size
    Width(Length),
    Height(Length),

    // Size Constraints
    MinWidth(Length),
    MinHeight(Length),
    MaxWidth(Length),
    MaxHeight(Length),

    // Margin
    Margin(Length),
    MarginLeft(Length),
    MarginRight(Length),
    MarginTop(Length),
    MarginBottom(Length),

    // Padding
    Padding(Length),
    PaddingLeft(Length),
    PaddingRight(Length),
    PaddingTop(Length),
    PaddingBottom(Length),

    // Flex Container
    FlexDirection(FlexDirection),
    JustifyContent(JustifyContent),
    AlignItems(AlignItems),
    AlignContent(AlignContent),

    // Flex Item
    FlexBasis(Length),
    FlexGrow(f32),
    FlexShrink(f32),
    AlignSelf(AlignSelf),

    // Border
    BorderRadius(Length),
    BorderTopLeftRadius(Length),
    BorderTopRightRadius(Length),
    BorderBottomLeftRadius(Length),
    BorderBottomRightRadius(Length),
    BorderWidth(Length),
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
}
