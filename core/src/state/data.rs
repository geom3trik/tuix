use crate::entity::Entity;

use crate::state::style::Visibility;

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

#[derive(Clone)]
pub struct Data {
    pub position: Vec<Pos>,
    pub size: Vec<Pos>,
    pub visibility: Vec<Visibility>,
    pub opacity: Vec<f32>,
    pub hoverability: Vec<bool>,
    pub z_order: Vec<i32>,
    pub clip_widget: Vec<Entity>,
    // Holds the child_width_sum and then the free_width_space
    pub(crate) child_sum: Vec<f32>, // Sum of child widths
    pub(crate) child_max: Vec<f32>, // Max child width
    pub(crate) child_pos: Vec<f32>,
    pub(crate) child_grow_sum: Vec<f32>,
    pub(crate) child_shrink_sum: Vec<f32>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            position: Vec::new(),
            size: Vec::new(),
            visibility: Vec::new(),
            hoverability: Vec::new(),
            child_sum: Vec::new(),
            child_max: Vec::new(),
            child_pos: Vec::new(),
            child_grow_sum: Vec::new(),
            child_shrink_sum: Vec::new(),
            opacity: Vec::new(),
            z_order: Vec::new(),
            clip_widget: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) {
        let key = entity.index_unchecked();

        if (key + 1) > self.position.len() {
            self.position.resize(key + 1, Default::default());
            self.size.resize(key + 1, Default::default());
            self.visibility.resize(key + 1, Default::default());
            self.hoverability.resize(key + 1, true);
            self.child_sum.resize(key + 1, 0.0);
            self.child_max.resize(key + 1, 0.0);
            self.child_pos.resize(key + 1, 0.0);
            self.child_grow_sum.resize(key + 1, 0.0);
            self.child_shrink_sum.resize(key + 1, 0.0);
            self.opacity.resize(key + 1, 0.0);
            self.z_order.resize(key + 1, 0);
            self.clip_widget.resize(key + 1, Entity::new(0));
        }

        // Are these needed?
        if let Some(stored) = self.size.get_mut(key) {
            *stored = Default::default();
        }

        if let Some(stored) = self.position.get_mut(key) {
            *stored = Default::default();
        }

        if let Some(stored) = self.visibility.get_mut(key) {
            *stored = Default::default();
        }
    }

    pub fn remove(&mut self, _entity: Entity) {}

    // For getters and setters it's safe to use unwrap because every entity must have a position and size.
    // Event if the position and size are 0.0, or the entity is invisible.

    pub fn get_clip_widget(&self, entity: Entity) -> Entity {
        self.clip_widget
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

    pub fn get_child_pos(&self, entity: Entity) -> f32 {
        self.child_pos
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_child_grow_sum(&self, entity: Entity) -> f32 {
        self.child_grow_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_child_shrink_sum(&self, entity: Entity) -> f32 {
        self.child_shrink_sum
            .get(entity.index_unchecked())
            .cloned()
            .unwrap()
    }

    pub fn get_posx(&self, entity: Entity) -> f32 {
        self.position
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .x
    }

    pub fn get_posy(&self, entity: Entity) -> f32 {
        self.position
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .y
    }

    pub fn get_width(&self, entity: Entity) -> f32 {
        self.size
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .x
    }

    pub fn get_height(&self, entity: Entity) -> f32 {
        self.size
            .get(entity.index_unchecked())
            .cloned()
            .unwrap_or_default()
            .y
    }

    pub fn get_opacity(&self, entity: Entity) -> f32 {
        self.opacity.get(entity.index_unchecked()).cloned().unwrap()
    }

    // SETTERS

    pub fn set_clip_widget(&mut self, entity: Entity, val: Entity) {
        if let Some(clip_widget) = self.clip_widget.get_mut(entity.index_unchecked()) {
            *clip_widget = val;
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

    pub fn set_child_pos(&mut self, entity: Entity, val: f32) {
        if let Some(child_pos) = self.child_pos.get_mut(entity.index_unchecked()) {
            *child_pos = val;
        }
    }

    pub fn set_child_grow_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_grow_sum) = self.child_grow_sum.get_mut(entity.index_unchecked()) {
            *child_grow_sum = val;
        }
    }

    pub fn set_child_shrink_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_shrink_sum) = self.child_shrink_sum.get_mut(entity.index_unchecked()) {
            *child_shrink_sum = val;
        }
    }

    pub fn set_posx(&mut self, entity: Entity, val: f32) {
        if let Some(position) = self.position.get_mut(entity.index_unchecked()) {
            position.x = val;
        }
    }

    pub fn set_posy(&mut self, entity: Entity, val: f32) {
        if let Some(position) = self.position.get_mut(entity.index_unchecked()) {
            position.y = val;
        }
    }

    pub fn set_width(&mut self, entity: Entity, val: f32) {
        if let Some(size) = self.size.get_mut(entity.index_unchecked()) {
            size.x = val;
        }
    }

    pub fn set_height(&mut self, entity: Entity, val: f32) {
        if let Some(size) = self.size.get_mut(entity.index_unchecked()) {
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

    pub fn set_hoverability(&mut self, entity: Entity, val: bool) {
        if let Some(hoverability) = self.hoverability.get_mut(entity.index_unchecked()) {
            *hoverability = val;
        }
    }

    pub fn set_opacity(&mut self, entity: Entity, val: f32) {
        if let Some(opacity) = self.opacity.get_mut(entity.index_unchecked()) {
            *opacity = val;
        }
    }
}
