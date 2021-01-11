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
pub use crate::window_event::WindowEvent;

use femtovg::FontId;

use std::collections::{HashMap, VecDeque};

use fnv::FnvHashMap;

pub struct Fonts {
    pub regular: Option<FontId>,
    pub bold: Option<FontId>,
    pub icons: Option<FontId>,
}

pub struct State {
    entity_manager: EntityManager, // Creates and destroys entities
    pub hierarchy: Hierarchy,      // The widget tree
    pub style: Style,              // The style properties for every widget
    pub transform: Transform,      // Transform properties for all widgets
    pub root: Entity,
    pub mouse: MouseState,
    pub modifiers: ModifiersState,
    pub hovered: Entity,
    pub active: Entity,
    pub captured: Entity,
    pub focused: Entity,

    pub event_handlers: FnvHashMap<Entity, Box<dyn EventHandler>>,
    pub event_queue: VecDeque<Event>,

    pub fonts: Fonts, //TODO - Replace with resource manager

    pub resource_manager: ResourceManager, //TODO
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
        style.add(root);

        style.clip_widget.set(root, root);

        style.background_color.insert(root, Color::rgb(80, 80, 80));

        State {
            entity_manager,
            hierarchy,
            style,
            transform,
            root,
            mouse,
            modifiers,
            hovered: Entity::new(0, 0),
            active: Entity::null(),
            captured: Entity::null(),
            focused: Entity::new(0, 0),
            event_handlers: FnvHashMap::default(),
            event_queue: VecDeque::new(),
            fonts: Fonts {
                regular: None,
                bold: None,
                icons: None,
            },
            resource_manager: ResourceManager::new(),
        }
    }

    pub fn build<'a, T>(&'a mut self, entity: Entity, event_handler: T) -> Builder<'a>
    where
        T: EventHandler + 'static,
    {
        self.event_handlers.insert(entity, Box::new(event_handler));

        Builder::new(self, entity)
    }

    
    pub fn insert_stylesheet(&mut self, path: &str) -> Result<(), std::io::Error> {

        let style_string = std::fs::read_to_string(path.clone())?;
        self.resource_manager.stylesheets.push(path.to_owned());

        // Parse the theme stylesheet
        self.style.parse_theme(&style_string);
        // self.resource_manager.themes.push(style_string);

        Ok(())
    }

    pub fn insert_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        self.reload_styles();
        // self.style.parse_theme(&overall_theme);
    }

    // Removes all style data and then reloads the stylesheets
    // TODO change the error type to allow for parsing errors 
    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {

        if self.resource_manager.themes.is_empty() && self.resource_manager.stylesheets.is_empty() {
            return Ok(())
        }

        // Remove all non-inline style data
        self.style.background_color.remove_styles();
        self.style.font_color.remove_styles();
        
        // Position
        self.style.left.remove_styles();
        self.style.right.remove_styles();
        self.style.top.remove_styles();
        self.style.bottom.remove_styles();
        // Size
        self.style.width.remove_styles();
        self.style.height.remove_styles();
        // Margins
        self.style.margin_left.remove_styles();
        self.style.margin_right.remove_styles();
        self.style.margin_top.remove_styles();
        self.style.margin_bottom.remove_styles();
        // Padding
        self.style.padding_left.remove_styles();
        self.style.padding_right.remove_styles();
        self.style.padding_top.remove_styles();
        self.style.padding_bottom.remove_styles();
        // Border
        self.style.border_width.remove_styles();
        self.style.border_color.remove_styles();
        // Border Radius
        self.style.border_radius_top_left.remove_styles();
        self.style.border_radius_top_right.remove_styles();
        self.style.border_radius_bottom_left.remove_styles();
        self.style.border_radius_bottom_right.remove_styles();
        // Flexbox
        self.style.flex_grow.remove_styles();
        self.style.flex_shrink.remove_styles();
        self.style.flex_basis.remove_styles();
        self.style.align_self.remove_styles();
        self.style.align_content.remove_styles();
        // Flex Container
        self.style.align_items.remove_styles();
        self.style.justify_content.remove_styles();
        self.style.flex_direction.remove_styles();
        // Display
        self.style.display.remove_styles();
        self.style.visibility.remove_styles();
        self.style.opacity.remove_styles();
        // Text Alignment
        self.style.text_align.remove_styles();
        self.style.text_justify.remove_styles();

        let mut overall_theme = String::new();

        // Reload the stored themes
        for theme in self.resource_manager.themes.iter() {
            //self.style.parse_theme(theme);
            overall_theme += theme;
        }

        // Reload the stored stylesheets
        for stylesheet in self.resource_manager.stylesheets.iter() {
            let theme = std::fs::read_to_string(stylesheet)?;
            overall_theme += &theme;
            
        }

        self.style.parse_theme(&overall_theme);

        self.insert_event(Event::new(WindowEvent::Restyle).target(Entity::null()));
        self.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
        self.insert_event(Event::new(WindowEvent::Redraw).target(Entity::null()));

        Ok(())
    }

    pub fn insert_event(&mut self, mut event: Event) {
        if event.unique {
            self.event_queue.retain(|e| e != &event);
        }

        self.event_queue.push_back(event);
    }


    pub fn id2entity(&self, id: &str) -> Option<Entity> {
        self.style.ids.get_by_left(&id.to_string()).cloned()
    }

    // This should probably be moved to state.mouse
    pub fn capture(&mut self, id: Entity) {
        if id != Entity::null() {
            self.insert_event(
                Event::new(WindowEvent::MouseCaptureEvent)
                    .target(id)
                    .propagate(Propagation::Direct),
            );
        }

        if self.captured != Entity::null() {
            self.insert_event(
                Event::new(WindowEvent::MouseCaptureOutEvent)
                    .target(self.captured)
                    .propagate(Propagation::Direct),
            );
        }

        self.captured = id;
        self.active = id;
    }

    // This should probably be moved to state.mouse
    pub fn release(&mut self, id: Entity) {
        if self.captured == id {
            self.insert_event(
                Event::new(WindowEvent::MouseCaptureOutEvent)
                    .target(self.captured)
                    .propagate(Propagation::Direct),
            );
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
        self.style.background_color.animate(std::time::Instant::now());
        self.style.font_color.animate(std::time::Instant::now());
        self.style.border_color.animate(std::time::Instant::now());

        self.style.left.animate(std::time::Instant::now());
        self.style.right.animate(std::time::Instant::now());
        self.style.top.animate(std::time::Instant::now());
        self.style.bottom.animate(std::time::Instant::now());
        self.style.width.animate(std::time::Instant::now());
        self.style.height.animate(std::time::Instant::now());
        self.style.opacity.animate(std::time::Instant::now());
        self.style.rotate.animate(std::time::Instant::now());
        self.style.flex_grow.animate(std::time::Instant::now());
        self.style.flex_shrink.animate(std::time::Instant::now());
        self.style.flex_basis.animate(std::time::Instant::now());
        self.style.margin_left.animate(std::time::Instant::now());
        self.style.margin_right.animate(std::time::Instant::now());
        self.style.margin_top.animate(std::time::Instant::now());
        self.style.margin_bottom.animate(std::time::Instant::now());
        self.style.padding_left.animate(std::time::Instant::now());
        self.style.padding_right.animate(std::time::Instant::now());
        self.style.padding_top.animate(std::time::Instant::now());
        self.style.padding_bottom.animate(std::time::Instant::now());
        self.style.border_radius_top_left.animate(std::time::Instant::now());
        self.style.border_radius_top_right.animate(std::time::Instant::now());
        self.style.border_radius_bottom_left.animate(std::time::Instant::now());
        self.style.border_radius_bottom_right.animate(std::time::Instant::now());
        self.style.border_width.animate(std::time::Instant::now());

        self.style.background_color.has_animations()
            || self.style.font_color.has_animations()
            || self.style.border_color.has_animations()
            || self.style.left.has_animations()
            || self.style.right.has_animations()
            || self.style.top.has_animations()
            || self.style.bottom.has_animations()
            || self.style.width.has_animations()
            || self.style.height.has_animations()
            || self.style.opacity.has_animations()
            || self.style.rotate.has_animations()
            || self.style.flex_grow.has_animations()
            || self.style.flex_shrink.has_animations()
            || self.style.flex_basis.has_animations()
            || self.style.margin_left.has_animations()
            || self.style.margin_right.has_animations()
            || self.style.margin_top.has_animations()
            || self.style.margin_bottom.has_animations()
            || self.style.padding_left.has_animations()
            || self.style.padding_right.has_animations()
            || self.style.padding_top.has_animations()
            || self.style.padding_bottom.has_animations()
            || self.style.border_radius_top_left.has_animations()
            || self.style.border_radius_top_right.has_animations()
            || self.style.border_radius_bottom_left.has_animations()
            || self.style.border_radius_bottom_right.has_animations()
            || self.style.border_width.has_animations()
    }

    pub fn get_root(&self) -> Entity {
        self.root
    }
}
