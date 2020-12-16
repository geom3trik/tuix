pub mod entity;
pub use entity::*;

pub mod hierarchy;
pub use hierarchy::*;

pub mod storage;
pub use storage::*;

pub mod style;
pub use style::*;

pub mod transform;
pub use transform::*;

pub mod animator;
pub use animator::*;

pub mod mouse;
pub use mouse::*;

pub mod resource;
pub use resource::*;

pub use crate::events::{Builder, Event, EventHandler, Propagation};
pub use crate::window::WindowEvent;

use femtovg::FontId;


use std::collections::{HashMap, VecDeque};

pub struct Fonts {
    pub regular: Option<FontId>,
    pub bold: Option<FontId>,
    pub icons: Option<FontId>,
}

pub struct State {
    entity_manager: EntityManager,  // Creates and destroys entities
    pub hierarchy: Hierarchy,       // The widget tree
    pub style: Style,               // The style properties for every widget
    pub transform: Transform,       // Transform properties for all widgets
    pub root: Entity,               
    pub mouse: MouseState,
    pub modifiers: ModifiersState,
    pub hovered: Entity,
    pub active: Entity,
    pub captured: Entity,
    pub focused: Entity,

    pub event_handlers: HashMap<Entity, Box<dyn EventHandler>>,
    pub event_queue: VecDeque<Event>,

    pub fonts: Fonts, //TODO - Replace with resource manager

    //pub resource_manager: ResourceManager, //TODO
}

impl State {
    pub fn new() -> Self {
        let mut entity_manager = EntityManager::new();
        let hierarchy = Hierarchy::new();
        let mut style = Style::new();
        let mut transform = Transform::new();
        let mouse = MouseState::default();
        let modifiers = ModifiersState::default();

        let root = entity_manager
            .create_entity()
            .expect("Failed to create root");

        transform.add(root);

        style.clip_widget.set(root, root);

        State {
            entity_manager,
            hierarchy,
            style,
            transform,
            root,
            mouse,
            modifiers,
            hovered: Entity::new(0,0),
            active: Entity::null(),
            captured: Entity::null(),
            focused: Entity::new(0,0),
            event_handlers: HashMap::new(),
            event_queue: VecDeque::new(),
            fonts: Fonts{regular: None, bold: None, icons: None},
            //resource_manager: ResourceManager::new(),
        }
    }

    pub fn build<'a, T>(&'a mut self, entity: Entity, event_handler: T) -> Builder<'a>
    where
        T: EventHandler + 'static,
    {
        self.event_handlers.insert(entity, Box::new(event_handler));

        Builder::new(self, entity)
    }

    // This should return an error type
    pub fn insert_style(&mut self, stylesheet: &str) {
        // Parse the theme stylesheet
        self.style.parse_theme(stylesheet);
    }

    pub fn insert_event(&mut self, mut event: Event) {
        if event.unique {
            self.event_queue.retain(|e| e != &event);
        }

        self.event_queue.push_back(event);
    }

    pub fn capture(&mut self, id: Entity) {
        //println!("Capture: {}", id);
        if id != Entity::null() {
            self.insert_event(Event::new(WindowEvent::MouseCaptureEvent).target(id).propagate(Propagation::Direct));
        }
        
        if self.captured != Entity::null() {
            self.insert_event(Event::new(WindowEvent::MouseCaptureOutEvent).target(self.captured).propagate(Propagation::Direct));
        }
        
        
        self.captured = id;
        self.active = id;

    }

    pub fn release(&mut self, id: Entity) {
        if self.captured == id {
            self.insert_event(Event::new(WindowEvent::MouseCaptureOutEvent).target(self.captured).propagate(Propagation::Direct));
            self.captured = Entity::null();
            self.active = Entity::null();
        }
        
    }

    pub fn add(&mut self, parent: Entity) -> Entity {
        let entity = self
            .entity_manager
            .create_entity()
            .expect("Failed to create entity");
        self.hierarchy.add(entity, Some(parent));

        self.transform.add(entity);
        self.style.add(entity);

        entity
    }

    // TODO
    // pub fn add_with_sibling(&mut self, sibling: Entity) -> Entity {
    //     let entity = self
    //         .entity_manager
    //         .create_entity()
    //         .expect("Failed to create entity");
    //     //self.hierarchy.add_with_sibling(entity, sibling);
    //     self.transform.add(entity);
    //     self.style.add(entity);

    //     entity
    // }

    //  TODO
    // pub fn remove(&mut self, entity: Entity) {
    //     //self.hierarchy.remove(entity);
    //     //self.transform.remove(entity);
    //     //self.style.remove(entity);
    //     //self.entity_manager.destroy_entity(entity);
    // }

    pub fn apply_animations(&mut self) -> bool {

        self.style
            .background_color
            .animate(std::time::Instant::now());

        self.style.left.animate(std::time::Instant::now());
        self.style.right.animate(std::time::Instant::now());
        self.style.top.animate(std::time::Instant::now());
        self.style.bottom.animate(std::time::Instant::now());
        self.style.width.animate(std::time::Instant::now());
        self.style.height.animate(std::time::Instant::now());
        self.style.opacity.animate(std::time::Instant::now());
        self.style.rotate.animate(std::time::Instant::now());
        self.style.flex_grow.animate(std::time::Instant::now());
        self.style.margin_left.animate(std::time::Instant::now());
        self.style.margin_right.animate(std::time::Instant::now());
        self.style.margin_top.animate(std::time::Instant::now());
        self.style.margin_bottom.animate(std::time::Instant::now());

        self.style.background_color.has_animations()
            || self.style.left.has_animations()
            || self.style.right.has_animations()
            || self.style.top.has_animations()
            || self.style.bottom.has_animations()
            || self.style.width.has_animations()
            || self.style.height.has_animations()
            || self.style.opacity.has_animations()
            || self.style.rotate.has_animations()
            || self.style.flex_grow.has_animations()
            || self.style.margin_left.has_animations()
            || self.style.margin_right.has_animations()
            || self.style.margin_top.has_animations()
            || self.style.margin_bottom.has_animations()
    }

    pub fn get_root(&self) -> Entity {
        self.root
    }
}
