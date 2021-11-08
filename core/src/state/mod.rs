//! # UI State
//!
//! [State] is where all of the UI data is stored. In ECS terms, [State] is the world and manages
//! the creation of entities (see [Widget]), and the storage of components (see [Style]).

/// Entity ID
pub mod entity;
pub use entity::{Entity, AsEntity};

/// Cached UI Data
pub mod data;
pub use data::*;

/// Mouse Data
pub mod mouse;
pub use mouse::*;

mod resource;
pub use resource::*;

mod layer;
pub use layer::*;


use crate::storage::shared_set::SharedSet;
use crate::{AnimationBuilder, BindEvent, Builder, Color, Event, EventHandler, PropSet, Propagation, Rule, Style};
use crate::{WindowEvent, Tree, TreeExt};

use crate::IdManager;

use femtovg::{TextContext};

use std::collections::{HashMap, VecDeque};

use fnv::FnvHashMap;

// TODO - Move this somewhere more appropriate
const STYLE: &str = r#"
    textbox>.caret {
        background-color: red;
    }

    textbox>.selection {
        background-color: #40000000;
    }

    scroll_container>.scrollbar {
        background-color: #606060;
        width: 15px;
    }

    checkbox {
        width: 20px;
        height: 20px;
        border-color: black;
        border-width: 1px;
    }

    /*
    button {
        background-color: #CCCCCC;
        width: 100px;
        height: 30px;
        child-space: 1s;
        border-radius: 3px;
    }

    button:hover {
        background-color: #E6E6E6;
    }

    button:active {
        background-color: #737373;
    }

    button:disabled {
        color: #737373;
    }
    */
"#;

// #[derive(Clone)]
// pub struct Fonts {
//     pub regular: Option<FontId>,
//     pub bold: Option<FontId>,
//     pub icons: Option<FontId>,
//     pub emoji: Option<FontId>,
//     pub arabic: Option<FontId>,
// }


/// Stores the gloabal state of the UI application.
pub struct State {
    /// Creates and destroys entities
    pub(crate) entity_manager: IdManager<Entity>,
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

    pub scale_factor: f32,


    // pub(crate) callbacks: FnvHashMap<Entity, Box<dyn FnMut(&mut Box<dyn EventHandler>, &mut Self, Entity)>>,

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

    pub layers: HashMap<i32, Layer>,

    pub listeners: FnvHashMap<Entity, Box<dyn Fn(&mut dyn EventHandler, &mut State, Entity, &mut Event)>>,
}

impl State {
    pub fn new() -> Self {
        let mut entity_manager = IdManager::new();
        let _root = entity_manager.create();
        let tree = Tree::new();
        let mut style = Style::default();
        let mut data = CachedData::default();
        let mouse = MouseState::default();
        let modifiers = ModifiersState::default();

        let root = Entity::root();

        data.add(root).expect("Failed to add root entity to data cache");
        style.add(root);

        style.clip_widget.insert(root, root).expect("msg");

        style.background_color.insert(root, Color::rgb(255, 255, 255));

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
            scale_factor: 1.0,
            //callbacks: FnvHashMap::default(),
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

            layers: HashMap::default(),

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

    /// Adds a style rule to the application (TODO)
    ///
    /// This function adds a style rule to the application allowing for multiple entites to share the same style properties based on the rule selector.
    ///
    /// # Examples
    /// Adds a style rule which sets the flex-grow properties of all 'button' elements to 1.0:
    /// ```
    /// state.add_style_rule(StyleRule::new(Selector::element("button")).property(Property::FlexGrow(1.0)))
    /// ```
    pub fn add_style_rule(&mut self) -> Rule {
        self.style.rule_manager.create()
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

        for rule in self.style.rules.iter() {
            self.style.rule_manager.destroy(rule.id);
        }

        self.style.rules.clear();
        
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
        // if event.unique {
        //     self.event_queue.retain(|e| e != &event);
        // }

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

    // Adds a new entity with a specified parent
    pub fn add(&mut self, parent: Entity) -> Entity {
        let entity = self
            .entity_manager
            .create();
        self.tree.add(entity, parent).expect("");
        self.data.add(entity).expect("Failed to add entity to data cache");
        self.style.add(entity);

        Entity::root().restyle(self);
        Entity::root().relayout(self);
        Entity::root().redraw(self);

        entity
    }

    //  TODO
    pub fn remove(&mut self, entity: Entity) {
        //println!("Request Remove: {}", entity);
        // Collect all entities below the removed entity on the same branch of the tree
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        for entity in delete_list.iter().rev() {
            //println!("Removing: {}", entity);
            //self.tree.remove(*entity);
            self.removed_entities.push(*entity);
        }

        Entity::root().restyle(self);
        Entity::root().relayout(self);
        Entity::root().redraw(self);
    }

    pub fn create_animation(&mut self, duration: std::time::Duration) -> AnimationBuilder {
        let id = self.style.animation_manager.create();
        AnimationBuilder::new(id, self, duration)
    }

    // Run all pending animations
    // TODO - This should probably be moved to style or an animation handling system
    pub fn apply_animations(&mut self) -> bool {

        let time = std::time::Instant::now();

        self.style.background_color.tick(time);
        
        // Spacing
        self.style.left.tick(time);
        self.style.right.tick(time);
        self.style.top.tick(time);
        self.style.bottom.tick(time);

        // Spacing Constraints
        self.style.min_left.tick(time);
        self.style.max_left.tick(time);
        self.style.min_right.tick(time);
        self.style.max_right.tick(time);
        self.style.min_top.tick(time);
        self.style.max_top.tick(time);
        self.style.min_bottom.tick(time);
        self.style.max_bottom.tick(time);

        // Size
        self.style.width.tick(time);
        self.style.height.tick(time);

        // Size Constraints
        self.style.min_width.tick(time);
        self.style.max_width.tick(time);
        self.style.min_height.tick(time);
        self.style.max_height.tick(time);

        // Child Spacing
        self.style.child_left.tick(time);
        self.style.child_right.tick(time);
        self.style.child_top.tick(time);
        self.style.child_bottom.tick(time);
        self.style.row_between.tick(time);
        self.style.col_between.tick(time);

        self.style.opacity.tick(time);
        self.style.rotate.tick(time);

        // Border Radius
        self.style.border_radius_top_left.tick(time);
        self.style.border_radius_top_right.tick(time);
        self.style.border_radius_bottom_left.tick(time);
        self.style.border_radius_bottom_right.tick(time);
        
        // Border
        self.style.border_width.tick(time);
        self.style.border_color.tick(time);

        // Font
        self.style.font_size.tick(time);
        self.style.font_color.tick(time);
        

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
