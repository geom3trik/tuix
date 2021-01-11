use crate::style;
use crate::style::Display;

use crate::style::color::Color;

use crate::state::style::*;

use crate::state::animator::Transition;

#[derive(Clone, Debug)]
pub enum Property {
    None,

    Animation(String),
    AnimationDuration(std::time::Duration),

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
    FlexBasis(f32),
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

    TextJustify(Justify),
    TextAlign(Align),

    FontSize(f32),
    FontColor(Color),

    Transition(Vec<Transition>),

    ZIndex(i32),
}
