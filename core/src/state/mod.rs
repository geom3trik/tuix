#![allow(dead_code)]

pub mod entity;
pub use entity::*;

pub mod tree;
pub use tree::*;

pub mod storage;
pub use storage::*;

pub mod style;
pub use style::*;

pub mod data;
pub use data::*;

pub mod animation;
pub use animation::*;

pub mod mouse;
pub use mouse::*;

pub mod resource;
pub use resource::*;


use crate::Message;
pub use crate::events::{Builder, Event, Propagation, Widget, EventHandler};
pub use crate::window_event::WindowEvent;

use femtovg::{FontId, TextContext};

use std::collections::VecDeque;

use fnv::FnvHashMap;

// TODO - Move this somewhere more appropriate
const STYLE: &str = r#"
    textbox>.caret {
        background-color: red;
    }

    textbox>.selection {
        background-color: #40000000;
    }
"#;

#[derive(Clone)]
pub struct Fonts {
    pub regular: Option<FontId>,
    pub bold: Option<FontId>,
    pub icons: Option<FontId>,
    pub emoji: Option<FontId>,
    pub arabic: Option<FontId>,
}

pub struct State {
    // Creates and destroys entities
    pub(crate) entity_manager: EntityManager, 
    // The widget tree
    pub tree: Tree,
    // The style properties for every widget
    pub style: Style,
    // Computed data for every widget
    pub data: CachedData,
    // Mouse state
    pub mouse: MouseState,
    // Modifiers state
    pub modifiers: ModifiersState,

    // Hovered entity
    pub hovered: Entity,
    // Active entity
    pub active: Entity,
    // Captured entity
    pub captured: Entity,
    // Focused entity
    pub focused: Entity,

    pub dragged: Entity,

    pub drag_message: Option<Box<dyn Message>>,


    pub(crate) callbacks: FnvHashMap<Entity, Box<dyn FnMut(&mut Box<dyn EventHandler>, &mut Self, Entity)>>,

    // Map of widgets
    pub event_handlers: FnvHashMap<Entity, Box<dyn EventHandler>>,

    // List of removed entities
    pub(crate) removed_entities: Vec<Entity>,

    // Queue of events
    pub event_queue: VecDeque<Event>,

    // pub fonts: Fonts, //TODO - Replace with resource manager

    pub resource_manager: ResourceManager, //TODO

    // Flag which signifies that a restyle is required
    pub needs_restyle: bool,
    pub needs_relayout: bool,
    pub needs_redraw: bool,

    pub text_context: TextContext,

    pub listeners: FnvHashMap<Entity, Box<dyn Fn(&mut dyn EventHandler, &mut State, Entity, &mut Event)>>,
}

impl State {
    pub fn new() -> Self {
        let mut entity_manager = EntityManager::new();
        let _root = entity_manager.create_entity();
        let tree = Tree::new();
        let mut style = Style::default();
        let mut data = CachedData::default();
        let mouse = MouseState::default();
        let modifiers = ModifiersState::default();

        let root = Entity::root();

        data.add(root);
        style.add(root);

        style.clip_widget.set(root, root);

        style.background_color.insert(root, Color::rgb(80, 80, 80)).expect("");

        style.default_font = "roboto".to_string();

        let mut resource_manager =ResourceManager::new();
        resource_manager.themes.push(STYLE.to_string());

        State {
            entity_manager,
            tree,
            style,
            data,
            mouse,
            modifiers,
            hovered: Entity::root(),
            active: Entity::null(),
            captured: Entity::null(),
            focused: Entity::root(),
            dragged: Entity::null(),
            drag_message: None,
            callbacks: FnvHashMap::default(),
            event_handlers: FnvHashMap::default(),
            event_queue: VecDeque::new(),
            removed_entities: Vec::new(),
            // fonts: Fonts {
            //     regular: None,
            //     bold: None,
            //     icons: None,
            //     emoji: None,
            //     arabic: None,
            // },
            resource_manager,
            needs_restyle: false,
            needs_relayout: false,
            needs_redraw: false,

            text_context: TextContext::default(),

            listeners: FnvHashMap::default(),
        }
    }

    pub(crate) fn build<'a, T>(&'a mut self, entity: Entity, event_handler: T) -> Builder<'a,T>
    where
        T: EventHandler + 'static,
    {
        self.event_handlers.insert(entity, Box::new(event_handler));

        Builder::new(self, entity)
    }

    pub fn query<E: EventHandler>(&mut self, entity: Entity) -> Option<&mut E> {
        if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
            event_handler.downcast::<E>()
        } else {
            None
        }
    }

    /// Adds a stylesheet to the application
    ///
    /// This function adds the stylesheet path to the application allowing for hot reloading of syles
    /// while the application is running.
    ///
    /// # Examples
    ///
    /// ```
    /// state.add_stylesheet("path_to_stylesheet.css");
    /// ```
    pub fn add_stylesheet(&mut self, path: &str) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path.clone())?;
        self.resource_manager.stylesheets.push(path.to_owned());
        self.style.parse_theme(&style_string);

        Ok(())
    }

    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        self.reload_styles().expect("Failed to reload styles");
    }

    /// Adds a style rule to the application
    ///
    /// This function adds a style rule to the application allowing for multiple entites to share the same style properties based on the rule selector.
    ///
    /// # Examples
    /// Adds a style rule which sets the flex-grow properties of all 'button' elements to 1.0:
    /// ```
    /// state.add_style_rule(StyleRule::new(Selector::element("button")).property(Property::FlexGrow(1.0)))
    /// ```
    pub fn add_style_rule(&mut self, style_rule: StyleRule) {
        self.style.add_rule(style_rule);
        Entity::root().restyle(self);
        Entity::root().relayout(self);
        Entity::root().redraw(self);
    }

    //TODO
    // pub fn add_image(&mut self, image: image::DynamicImage) -> Rc<()> {
    //     self.resource_manager.add_image(image)
    // }

    /// Add a font from memory to the application
    pub fn add_font_mem(&mut self, name: &str, data: &[u8]) {
        // TODO - return error
        if self.resource_manager.fonts.contains_key(name) {
            println!("Font already exists");
            return;
        }
        //let id = self.text_context.add_font_mem(&data.clone()).expect("failed");
        //println!("{} {:?}", name, id);
        self.resource_manager.fonts.insert(name.to_owned(), FontOrId::Font(data.to_vec()));
    }

    /// Sets the global default font for the application
    pub fn set_default_font(&mut self, name: &str) {
        self.style.default_font = name.to_string();
    }

    // Removes all style data and then reloads the stylesheets
    // TODO change the error type to allow for parsing errors
    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {
        if self.resource_manager.themes.is_empty() && self.resource_manager.stylesheets.is_empty() {
            return Ok(());
        }

        self.style.remove_all();

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

        Entity::root().restyle(self);
        Entity::root().relayout(self);
        Entity::root().redraw(self);

        Ok(())
    }

    /// Insert a new event into the application event queue
    ///
    /// Inserts a new event into the application event queue that will be processed on the next event loop.
    /// If the event unique flag is set to true, only the most recent event of the same type will exist in the queue.
    ///
    /// # Examples
    /// ```
    /// state.insert_event(Event::new(WindowEvent::WindowClose));
    /// ```
    pub fn insert_event(&mut self, event: Event) {
        if event.unique {
            self.event_queue.retain(|e| e != &event);
        }

        self.event_queue.push_back(event);
    }

    // This should probably be moved to state.mouse
    pub fn capture(&mut self, entity: Entity) {
        //println!("CAPTURE: {}", entity);
        if entity != Entity::null() && self.captured != entity {
            self.insert_event(
                Event::new(WindowEvent::MouseCaptureEvent)
                    .target(entity)
                    .propagate(Propagation::Direct),
            );
        }

        if self.captured != Entity::null() && self.captured != entity {
            self.insert_event(
                Event::new(WindowEvent::MouseCaptureOutEvent)
                    .target(self.captured)
                    .propagate(Propagation::Direct),
            );
        }

        self.captured = entity;
        self.active = entity;
    }

    // This should probably be moved to state.mouse
    pub fn release(&mut self, id: Entity) {
        
        if self.captured == id {
            self.insert_event(
                Event::new(WindowEvent::MouseCaptureOutEvent)
                    .target(self.captured)
                    .propagate(Propagation::Direct),
            );

            //println!("RELEASE: {}", id);
            
            self.captured = Entity::null();
            self.active = Entity::null();
        }
        
    }

    pub fn set_focus(&mut self, entity: Entity) {
        if self.focused != entity {
            if self.focused != Entity::null() {
                self.focused.set_focus(self, false);
                self.insert_event(Event::new(WindowEvent::FocusOut).target(self.focused));
            }
            
            if entity != Entity::null() {
                self.focused = entity;
                entity.set_focus(self, true);
                self.insert_event(Event::new(WindowEvent::FocusIn).target(self.focused));
            }
        }  
    }

    pub fn drag<M>(&mut self, entity: Entity, message: Option<M>) 
    where M: Message,
    {
        self.dragged = entity;
        if let Some(m) = message {
            self.drag_message = Some(Box::new(m));
        }
        
    }

    // Adds a new entity with a specified parent
    pub(crate) fn add(&mut self, parent: Entity) -> Entity {
        let entity = self
            .entity_manager
            .create_entity()
            .expect("Failed to create entity");
        self.tree.add(entity, parent).expect("");
        self.data.add(entity);
        self.style.add(entity);

        Entity::root().restyle(self);
        Entity::root().relayout(self);
        Entity::root().redraw(self);

        entity
    }

    //  TODO
    pub fn remove(&mut self, entity: Entity) {
        // Collect all entities below the removed entity on the same branch of the tree
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        for entity in delete_list.iter().rev() {
            self.tree.remove(*entity).expect("");
            //self.tree.remove(*entity);
            self.data.remove(*entity);
            self.style.remove(*entity);
            self.removed_entities.push(*entity);
            self.entity_manager.destroy_entity(*entity);
        }

        Entity::root().restyle(self);
        Entity::root().relayout(self);
        Entity::root().redraw(self);
    }

    // Run all pending animations
    // TODO - This should probably be moved to style or an animation handling system
    pub fn apply_animations(&mut self) -> bool {

        let time = std::time::Instant::now();

        self.style.background_color.animate(time);
        
        // Spacing
        self.style.left.animate(time);
        self.style.right.animate(time);
        self.style.top.animate(time);
        self.style.bottom.animate(time);

        // Spacing Constraints
        self.style.min_left.animate(time);
        self.style.max_left.animate(time);
        self.style.min_right.animate(time);
        self.style.max_right.animate(time);
        self.style.min_top.animate(time);
        self.style.max_top.animate(time);
        self.style.min_bottom.animate(time);
        self.style.max_bottom.animate(time);

        // Size
        self.style.width.animate(time);
        self.style.height.animate(time);

        // Size Constraints
        self.style.min_width.animate(time);
        self.style.max_width.animate(time);
        self.style.min_height.animate(time);
        self.style.max_height.animate(time);

        // Child Spacing
        self.style.child_left.animate(time);
        self.style.child_right.animate(time);
        self.style.child_top.animate(time);
        self.style.child_bottom.animate(time);
        self.style.row_between.animate(time);
        self.style.col_between.animate(time);

        self.style.opacity.animate(time);
        self.style.rotate.animate(time);

        // Border Radius
        self.style.border_radius_top_left.animate(time);
        self.style.border_radius_top_right.animate(time);
        self.style.border_radius_bottom_left.animate(time);
        self.style.border_radius_bottom_right.animate(time);
        
        // Border
        self.style.border_width.animate(time);
        self.style.border_color.animate(time);

        // Font
        self.style.font_size.animate(time);
        self.style.font_color.animate(time);
        

        self.style.background_color.has_animations()
            || self.style.font_color.has_animations()
            // Spacing
            || self.style.left.has_animations()
            || self.style.right.has_animations()
            || self.style.top.has_animations()
            || self.style.bottom.has_animations()
            // Spacing Constraints
            || self.style.min_left.has_animations()
            || self.style.max_left.has_animations()
            || self.style.min_right.has_animations()
            || self.style.max_right.has_animations()
            || self.style.min_top.has_animations()
            || self.style.max_top.has_animations()
            || self.style.min_bottom.has_animations()
            || self.style.max_bottom.has_animations()
            // Size
            || self.style.width.has_animations()
            || self.style.height.has_animations()
            // Size Constraints
            || self.style.min_width.has_animations()
            || self.style.max_width.has_animations()
            || self.style.min_height.has_animations()
            || self.style.max_height.has_animations()
            // Child Spacing
            || self.style.child_left.has_animations()
            || self.style.child_right.has_animations()
            || self.style.child_top.has_animations()
            || self.style.child_bottom.has_animations()
            || self.style.row_between.has_animations()
            || self.style.col_between.has_animations()
            //
            || self.style.opacity.has_animations()
            || self.style.rotate.has_animations()
            // Border Radius
            || self.style.border_radius_top_left.has_animations()
            || self.style.border_radius_top_right.has_animations()
            || self.style.border_radius_bottom_left.has_animations()
            || self.style.border_radius_bottom_right.has_animations()
            // Border
            || self.style.border_width.has_animations()
            || self.style.border_color.has_animations()

    }
}
