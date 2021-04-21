use crate::entity::Entity;

use crate::state::style::Visibility;

/// Computed properties used for layout and drawing

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new() -> Self {
        Pos { x: 0.0, y: 0.0 }
    }

    pub fn with(a: f32, b: f32) -> Self {
        Pos { x: a, y: b }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0.0, y: 0.0 }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Self {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Margins {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: std::f32::MAX,
            h: std::f32::MAX,
        }
    }
}

#[derive(Clone, Default)]
pub struct Data {
    pub bounds: Vec<BoundingBox>,
    pub visibility: Vec<Visibility>,
    pub opacity: Vec<f32>,
    // TODO - combine hoverability and focusability with a bitflag
    pub hoverability: Vec<bool>,
    pub focusability: Vec<bool>,

    pub z_order: Vec<i32>,

    pub(crate) child_sum: Vec<f32>, // Sum of child widths
    pub(crate) child_max: Vec<f32>, // Max child width

    pub(crate) prev_size: Vec<Pos>,
    pub clip_region: Vec<BoundingBox>,

    margins: Vec<Margins>,
    cross_stretch_sum: Vec<f32>,
    cross_free_space: Vec<f32>,

    horizontal_used_space: Vec<f32>,
    horizontal_stretch_sum: Vec<f32>,
    vertical_used_space: Vec<f32>,
    vertical_stretch_sum: Vec<f32>,

    // is_first_child, is_last_child
    stack_child: Vec<(bool, bool)>,
}

impl Data {
    pub fn add(&mut self, entity: Entity) {
        let key = entity.index_unchecked();

        if (key + 1) > self.bounds.len() {
            self.bounds.resize(key + 1, Default::default());
            self.visibility.resize(key + 1, Default::default());
            self.hoverability.resize(key + 1, true);
            self.focusability.resize(key + 1, true);
            self.child_sum.resize(key + 1, 0.0);
            self.child_max.resize(key + 1, 0.0);
            self.prev_size.resize(key + 1, Default::default());

            self.opacity.resize(key + 1, 0.0);
            self.z_order.resize(key + 1, 0);

            self.clip_region.resize(key + 1, Default::default());
            self.margins.resize(key + 1, Default::default());
            self.cross_stretch_sum.resize(key + 1, Default::default());
            self.cross_free_space.resize(key + 1, Default::default());

            self.horizontal_used_space
                .resize(key + 1, Default::default());
            self.horizontal_stretch_sum
                .resize(key + 1, Default::default());
            self.vertical_used_space.resize(key + 1, Default::default());
            self.vertical_stretch_sum
                .resize(key + 1, Default::default());
            self.stack_child.resize(key + 1, (false, false));
        }
    }

    pub fn remove(&mut self, _entity: Entity) {}

    // For getters and setters it's safe to use unwrap because every entity must have a position and size.
    // Event if the position and size are 0.0, or the entity is invisible.

    // pub fn get_clip_widget(&self, entity: Entity) -> Entity {
    //     self.clip_widget
    //         .get(entity.index_unchecked())
    //         .cloned()
    //         .unwrap()
    // }

    pub fn get_stack_child(&self, entity: Entity) -> (bool, bool) {
        self.stack_child
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or((false, false))
    }

    pub fn get_bounds(&self, entity: Entity) -> BoundingBox {
        BoundingBox {
            x: self.get_posx(entity),
            y: self.get_posy(entity),
            w: self.get_width(entity),
            h: self.get_height(entity),
        }
    }

    pub fn get_cross_stretch_sum(&self, entity: Entity) -> f32 {
        self.cross_stretch_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn set_cross_stretch_sum(&mut self, entity: Entity, val: f32) {
        if let Some(cross_stretch_sum) = self.cross_stretch_sum.get_mut(entity.index_unchecked()) {
            *cross_stretch_sum = val;
        }
    }

    pub fn get_cross_free_space(&self, entity: Entity) -> f32 {
        self.cross_free_space
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn set_cross_free_space(&mut self, entity: Entity, val: f32) {
        if let Some(cross_free_space) = self.cross_free_space.get_mut(entity.index_unchecked()) {
            *cross_free_space = val;
        }
    }

    pub fn get_space_left(&self, entity: Entity) -> f32 {
        self.margins
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
            .left
    }

    pub fn get_space_right(&self, entity: Entity) -> f32 {
        self.margins
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
            .right
    }

    pub fn get_space_top(&self, entity: Entity) -> f32 {
        self.margins
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
            .top
    }

    pub fn get_space_bottom(&self, entity: Entity) -> f32 {
        self.margins
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
            .bottom
    }

    pub fn get_space(&self, entity: Entity) -> Margins {
        self.margins.get(entity.index_unchecked()).cloned().unwrap()
    }

    pub fn get_clip_region(&self, entity: Entity) -> BoundingBox {
        self.clip_region
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_z_order(&self, entity: Entity) -> i32 {
        self.z_order.get(entity.index_unchecked()).cloned().unwrap()
    }

    pub fn get_child_sum(&self, entity: Entity) -> f32 {
        self.child_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_child_max(&self, entity: Entity) -> f32 {
        self.child_max
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_posx(&self, entity: Entity) -> f32 {
        self.bounds
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .x
    }

    pub fn get_posy(&self, entity: Entity) -> f32 {
        self.bounds
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .y
    }

    pub fn get_width(&self, entity: Entity) -> f32 {
        self.bounds
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .w
    }

    pub fn get_height(&self, entity: Entity) -> f32 {
        self.bounds
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .h
    }

    pub(crate) fn get_prev_width(&self, entity: Entity) -> f32 {
        self.prev_size
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .x
    }

    pub(crate) fn get_prev_height(&self, entity: Entity) -> f32 {
        self.prev_size
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .y
    }

    pub fn get_opacity(&self, entity: Entity) -> f32 {
        self.opacity.get(entity.index_unchecked()).cloned().unwrap()
    }

    pub fn get_horizontal_used_space(&self, entity: Entity) -> f32 {
        self.horizontal_used_space
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_horizontal_stretch_sum(&self, entity: Entity) -> f32 {
        self.horizontal_stretch_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_vertical_used_space(&self, entity: Entity) -> f32 {
        self.vertical_used_space
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_vertical_stretch_sum(&self, entity: Entity) -> f32 {
        self.vertical_stretch_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    // SETTERS

    // pub fn set_clip_widget(&mut self, entity: Entity, val: Entity) {
    //     if let Some(clip_widget) = self.clip_widget.get_mut(entity.index_unchecked()) {
    //         *clip_widget = val;
    //     }
    // }

    pub fn set_stack_first_child(&mut self, entity: Entity, value: bool) {
        if let Some(stack_child) = self.stack_child.get_mut(entity.index_unchecked()) {
            stack_child.0 = value;
        }
    }

    pub fn set_stack_last_child(&mut self, entity: Entity, value: bool) {
        if let Some(stack_child) = self.stack_child.get_mut(entity.index_unchecked()) {
            stack_child.1 = value;
        }
    }

    pub fn set_horizontal_used_space(&mut self, entity: Entity, value: f32) {
        if let Some(horizontal_used_space) =
            self.horizontal_used_space.get_mut(entity.index_unchecked())
        {
            *horizontal_used_space = value;
        }
    }

    pub fn set_horizontal_stretch_sum(&mut self, entity: Entity, value: f32) {
        if let Some(horizontal_stretch_sum) = self
            .horizontal_stretch_sum
            .get_mut(entity.index_unchecked())
        {
            *horizontal_stretch_sum = value;
        }
    }

    pub fn set_vertical_used_space(&mut self, entity: Entity, value: f32) {
        if let Some(vertical_used_space) =
            self.vertical_used_space.get_mut(entity.index_unchecked())
        {
            *vertical_used_space = value;
        }
    }

    pub fn set_vertical_stretch_sum(&mut self, entity: Entity, value: f32) {
        if let Some(vertical_stretch_sum) =
            self.vertical_stretch_sum.get_mut(entity.index_unchecked())
        {
            *vertical_stretch_sum = value;
        }
    }

    pub fn set_margins(&mut self, entity: Entity, val: Margins) {
        if let Some(margins) = self.margins.get_mut(entity.index_unchecked()) {
            *margins = val;
        }
    }

    pub fn set_space_left(&mut self, entity: Entity, val: f32) {
        if let Some(margins) = self.margins.get_mut(entity.index_unchecked()) {
            margins.left = val;
        }
    }

    pub fn set_space_right(&mut self, entity: Entity, val: f32) {
        if let Some(margins) = self.margins.get_mut(entity.index_unchecked()) {
            margins.right = val;
        }
    }

    pub fn set_space_top(&mut self, entity: Entity, val: f32) {
        if let Some(margins) = self.margins.get_mut(entity.index_unchecked()) {
            margins.top = val;
        }
    }

    pub fn set_space_bottom(&mut self, entity: Entity, val: f32) {
        if let Some(margins) = self.margins.get_mut(entity.index_unchecked()) {
            margins.bottom = val;
        }
    }

    pub fn set_clip_region(&mut self, entity: Entity, val: BoundingBox) {
        if let Some(clip_region) = self.clip_region.get_mut(entity.index_unchecked()) {
            *clip_region = val;
        }
    }

    pub fn set_z_order(&mut self, entity: Entity, val: i32) {
        if let Some(z_order) = self.z_order.get_mut(entity.index_unchecked()) {
            *z_order = val;
        }
    }

    pub fn set_child_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_sum) = self.child_sum.get_mut(entity.index_unchecked()) {
            *child_sum = val;
        }
    }

    pub fn set_child_max(&mut self, entity: Entity, val: f32) {
        if let Some(child_max) = self.child_max.get_mut(entity.index_unchecked()) {
            *child_max = val;
        }
    }

    pub fn set_posx(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity.index_unchecked()) {
            bounds.x = val;
        }
    }

    pub fn set_posy(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity.index_unchecked()) {
            bounds.y = val;
        }
    }

    pub fn set_width(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity.index_unchecked()) {
            bounds.w = val;
        }
    }

    pub fn set_height(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity.index_unchecked()) {
            bounds.h = val;
        }
    }

    pub(crate) fn set_prev_width(&mut self, entity: Entity, val: f32) {
        if let Some(size) = self.prev_size.get_mut(entity.index_unchecked()) {
            size.x = val;
        }
    }

    pub(crate) fn set_prev_height(&mut self, entity: Entity, val: f32) {
        if let Some(size) = self.prev_size.get_mut(entity.index_unchecked()) {
            size.y = val;
        }
    }

    pub fn get_visibility(&self, entity: Entity) -> Visibility {
        self.visibility
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn set_visibility(&mut self, entity: Entity, val: Visibility) {
        if let Some(visibility) = self.visibility.get_mut(entity.index_unchecked()) {
            *visibility = val;
        }
    }

    pub fn get_hoverability(&self, entity: Entity) -> bool {
        self.hoverability
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_focusability(&self, entity: Entity) -> bool {
        self.focusability
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn set_hoverability(&mut self, entity: Entity, val: bool) {
        if let Some(hoverability) = self.hoverability.get_mut(entity.index_unchecked()) {
            *hoverability = val;
        }
    }

    pub fn set_focusability(&mut self, entity: Entity, val: bool) {
        if let Some(focusability) = self.focusability.get_mut(entity.index_unchecked()) {
            *focusability = val;
        }
    }

    pub fn set_opacity(&mut self, entity: Entity, val: f32) {
        if let Some(opacity) = self.opacity.get_mut(entity.index_unchecked()) {
            *opacity = val;
        }
    }
}
