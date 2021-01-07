
// TODO - Grid is not yet implemented

use super::layout::*;

use super::flexbox::JustifyContent;

// Container Properties
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
}

impl Default for JustifyItems {
    fn default() -> Self {
        JustifyItems::Start
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Stetch,
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Start
    }
}

// Determines the alignment of the grid items in the row dimension when the grid is smaller than the container.
// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum JustifyContent {
//     Start,
//     End,
//     Center,
//     Stretch,
//     SpaceBetween,
//     SpaceAround,
//     SpaceEvenly,
// }

// impl Default for JustifyContent {
//     fn default() -> Self {
//         JustifyContent::Start
//     }
// }

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignContent {
    Start,
    End,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for AlignContent {
    fn default() -> Self {
        AlignContent::Start
    }
}

// Item Properties

#[derive(Clone, PartialEq, Debug)]
pub struct GridContainer {
    pub grid_template_rows: Vec<f32>,
    pub grid_template_columns: Vec<f32>,
    pub grid_template_areas: Vec<f32>,
    pub grid_column_gap: f32,
    pub grid_row_gap: f32,
}

impl Default for GridContainer {
    fn default() -> Self {
        GridContainer {
            grid_template_rows: Vec::new(),
            grid_template_columns: Vec::new(),
            grid_template_areas: Vec::new(),
            grid_column_gap: 0.0,
            grid_row_gap: 0.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GridItem {
    pub grid_row_start: u32,
    pub grid_row_span: u32,
    pub grid_column_start: u32,
    pub grid_column_span: u32,
    pub justify_self: JustifySelf,
    pub align_self: AlignSelf,
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem {
            grid_row_start: 0,
            grid_row_span: 0,
            grid_column_start: 0,
            grid_column_span: 0,
            justify_self: Default::default(),
            align_self: Default::default(),
        }
    }
}
