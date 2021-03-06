use crate::{Interpolator, entity::Entity};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Column
    }
}

impl Interpolator for FlexDirection {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return *end;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexWrap {
    NoWrap,
    //Wrap,        //TODO
    //WrapReverse, //TODO
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::NoWrap
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::FlexStart
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    //Baseline, //TODO
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Stretch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignSelf {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> Self {
        AlignSelf::Stretch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> AlignContent {
        AlignContent::Stretch
    }
}
