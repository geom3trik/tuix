use crate::entity::Entity;

use crate::state::style::Visibility;
use crate::state::style::Transform2D;

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
pub struct CachedData {
    pub bounds: Vec<BoundingBox>,
    pub visibility: Vec<Visibility>,
    pub opacity: Vec<f32>,
    // TODO - combine hoverable and focusable with a bitflag
    pub hoverable: Vec<bool>,
    pub focusable: Vec<bool>,

    pub z_order: Vec<i32>,

    pub(crate) child_sum: Vec<(f32, f32)>, // Sum of child (widths, heights)
    pub(crate) child_max: Vec<(f32, f32)>, // Max child (widths, heights)

    pub(crate) prev_size: Vec<Pos>,
    pub clip_region: Vec<BoundingBox>,

    rotate: Vec<f32>,
    scale: Vec<(f32, f32)>,
    transform: Vec<Transform2D>,

    origin: Vec<(f32, f32)>,

    margins: Vec<Margins>,
    cross_stretch_sum: Vec<f32>,
    cross_free_space: Vec<f32>,

    horizontal_free_space: Vec<f32>,
    horizontal_stretch_sum: Vec<f32>,
    vertical_free_space: Vec<f32>,
    vertical_stretch_sum: Vec<f32>,

    grid_row_max: Vec<f32>,
    grid_col_max: Vec<f32>,

    // is_first_child, is_last_child
    stack_child: Vec<(bool, bool)>,
}

impl CachedData {
    pub fn add(&mut self, entity: Entity) {
        let key = entity.index_unchecked();

        if (key + 1) > self.bounds.len() {
            self.bounds.resize(key + 1, Default::default());
            self.visibility.resize(key + 1, Default::default());
            self.hoverable.resize(key + 1, true);
            self.focusable.resize(key + 1, true);
            self.child_sum.resize(key + 1, (0.0, 0.0));
            self.child_max.resize(key + 1, (0.0, 0.0));
            self.prev_size.resize(key + 1, Default::default());

            self.opacity.resize(key + 1, 0.0);

            self.rotate.resize(key + 1, 0.0);
            self.scale.resize(key + 1, (1.0, 1.0));
            self.transform.resize(key + 1, Transform2D::identity());
            self.origin.resize(key + 1, (0.0, 0.0));

            self.z_order.resize(key + 1, 0);

            self.clip_region.resize(key + 1, Default::default());
            self.margins.resize(key + 1, Default::default());
            self.cross_stretch_sum.resize(key + 1, Default::default());
            self.cross_free_space.resize(key + 1, Default::default());

            self.horizontal_free_space
                .resize(key + 1, Default::default());
            self.horizontal_stretch_sum
                .resize(key + 1, Default::default());
            self.vertical_free_space.resize(key + 1, Default::default());
            self.vertical_stretch_sum
                .resize(key + 1, Default::default());
            self.stack_child.resize(key + 1, (false, false));

            self.grid_row_max.resize(key + 1, 0.0);
            self.grid_col_max.resize(key + 1, 0.0);
        }
    }

    pub fn reset(&mut self) {
        for (width, height) in self.child_sum.iter_mut() {
            *width = Default::default();
            *height = Default::default();
        }

        for (width, height) in self.child_max.iter_mut() {
            *width = Default::default();
            *height = Default::default();
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

    pub fn get_grid_row_max(&self, entity: Entity) -> f32 {
        self.grid_row_max.get(entity.index_unchecked()).cloned().unwrap_or_default()
    }

    pub fn get_grid_col_max(&self, entity: Entity) -> f32 {
        self.grid_col_max.get(entity.index_unchecked()).cloned().unwrap_or_default()
    }

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

    pub fn get_child_width_sum(&self, entity: Entity) -> f32 {
        self.child_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap().0
    }

    pub fn get_child_height_sum(&self, entity: Entity) -> f32 {
        self.child_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap().1
    }

    pub fn get_child_width_max(&self, entity: Entity) -> f32 {
        self.child_max
            .get(entity.index_unchecked())
            .cloned()
            .unwrap().0
    }

    pub fn get_child_height_max(&self, entity: Entity) -> f32 {
        self.child_max
            .get(entity.index_unchecked())
            .cloned()
            .unwrap().1
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

    pub fn get_horizontal_free_space(&self, entity: Entity) -> f32 {
        self.horizontal_free_space
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

    pub fn get_vertical_free_space(&self, entity: Entity) -> f32 {
        self.vertical_free_space
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

    pub fn get_rotate(&self, entity: Entity) -> f32 {
        self.transform
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()[0]
            .acos()
    }

    pub fn get_translate(&self, entity: Entity) -> (f32, f32) {
        let transform = self.transform
            .get(entity.index_unchecked())
            .cloned()
            .unwrap();

        (transform[4], transform[5])
    }

    pub fn get_scale(&self, entity: Entity) -> f32 {
        let scale = self.scale
            .get(entity.index_unchecked())
            .cloned()
            .unwrap();

        scale.0
    }

    pub fn get_origin(&self, entity: Entity) -> (f32, f32) {
        self.origin
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_transform(&self, entity: Entity) -> Transform2D {
        self.transform
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_transform_mut(&mut self, entity: Entity) -> &mut Transform2D {
        self.transform
            .get_mut(entity.index_unchecked())
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

    pub fn set_horizontal_free_space(&mut self, entity: Entity, value: f32) {
        if let Some(horizontal_free_space) =
            self.horizontal_free_space.get_mut(entity.index_unchecked())
        {
            *horizontal_free_space = value;
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

    pub fn set_vertical_free_space(&mut self, entity: Entity, value: f32) {
        if let Some(vertical_free_space) =
            self.vertical_free_space.get_mut(entity.index_unchecked())
        {
            *vertical_free_space = value;
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

    pub fn set_child_width_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_sum) = self.child_sum.get_mut(entity.index_unchecked()) {
            child_sum.0 = val;
        }
    }

    pub fn set_child_height_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_sum) = self.child_sum.get_mut(entity.index_unchecked()) {
            child_sum.1 = val;
        }
    }

    pub fn set_child_width_max(&mut self, entity: Entity, val: f32) {
        if let Some(child_max) = self.child_max.get_mut(entity.index_unchecked()) {
            child_max.0 = val;
        }
    }

    pub fn set_child_height_max(&mut self, entity: Entity, val: f32) {
        if let Some(child_max) = self.child_max.get_mut(entity.index_unchecked()) {
            child_max.1 = val;
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

    pub fn get_hoverable(&self, entity: Entity) -> bool {
        self.hoverable
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_focusable(&self, entity: Entity) -> bool {
        self.focusable
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn set_hoverable(&mut self, entity: Entity, val: bool) {
        if let Some(hoverable) = self.hoverable.get_mut(entity.index_unchecked()) {
            *hoverable = val;
        }
    }

    pub fn set_focusable(&mut self, entity: Entity, val: bool) {
        if let Some(focusable) = self.focusable.get_mut(entity.index_unchecked()) {
            *focusable = val;
        }
    }

    pub fn set_opacity(&mut self, entity: Entity, val: f32) {
        if let Some(opacity) = self.opacity.get_mut(entity.index_unchecked()) {
            *opacity = val;
        }
    }

    pub fn set_rotate(&mut self, entity: Entity, val: f32) {
        if let Some(transform) = self.transform.get_mut(entity.index_unchecked()) {
            let mut t = Transform2D::identity();
            t.rotate(val);
            transform.premultiply(&t);
        }
    }

    pub fn set_translate(&mut self, entity: Entity, val: (f32, f32)) {
        if let Some(transform) = self.transform.get_mut(entity.index_unchecked()) {
            let mut t = Transform2D::identity();
            t.translate(val.0, val.1);
            transform.premultiply(&t);
        }
    }

    pub fn set_scale(&mut self, entity: Entity, val: f32) {
        if let Some(transform) = self.transform.get_mut(entity.index_unchecked()) {
            let mut t = Transform2D::identity();
            t.scale(val, val);
            transform.premultiply(&t);
        }
    }

    pub fn set_origin(&mut self, entity: Entity, val: (f32, f32)) {
        if let Some(origin) = self.origin.get_mut(entity.index_unchecked()) {
            *origin = val;
        }
    }

    pub fn set_transform(&mut self, entity: Entity, val: Transform2D) {
        if let Some(transform) = self.transform.get_mut(entity.index_unchecked()) {
            *transform = val;
        }
    }
}
