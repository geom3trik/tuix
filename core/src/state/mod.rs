pub mod entity;
use bimap::btree::Iter;
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

pub mod handle;
pub use handle::*;

pub use crate::events::{Event, EventHandler, Propagation};
pub use crate::window_event::WindowEvent;
use crate::Message;

use std::{any::TypeId, collections::{HashMap, VecDeque}};

use fnv::FnvHashMap;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

//use hecs::{World,Query};

use femtovg::{
    renderer::OpenGl, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

#[derive(Clone)]
pub struct Fonts {
    pub regular: Option<FontId>,
    pub bold: Option<FontId>,
    pub icons: Option<FontId>,
}

pub trait DrawHandler {
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        // Skip window
        if entity == Entity::new(0, 0) {
            return;
        }

        // Skip invisible widgets
        if state.transform.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        if state.transform.get_opacity(entity) == 0.0 {
            return;
        }

        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        let padding_left = match state
            .style
            .borrow_mut()
            .padding_left
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => *val,
            _ => 0.0,
        };

        let padding_right = match state
            .style
            .borrow_mut()
            .padding_right
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => *val,
            _ => 0.0,
        };

        let padding_top = match state
            .style
            .borrow_mut()
            .padding_top
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => *val,
            _ => 0.0,
        };

        let padding_bottom = match state
            .style
            .borrow_mut()
            .padding_bottom
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => *val,
            _ => 0.0,
        };

        let background_color = state
            .style
            .borrow_mut()
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let font_color = state
            .style
            .borrow_mut()
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(crate::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .borrow_mut()
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let shadow_color = state
            .style
            .borrow_mut()
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.transform.get_width(parent);
        let parent_height = state.transform.get_height(parent);

        let border_radius_top_left = match state
            .style
            .borrow_mut()
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .borrow_mut()
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .borrow_mut()
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .borrow_mut()
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.transform.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let mut shadow_color: femtovg::Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        let border_width = match state
            .style
            .borrow_mut()
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        // Skip widgets with no width or no height
        if width + 2.0 * border_width + padding_left + padding_right == 0.0
            || height + 2.0 * border_width + padding_top + padding_bottom == 0.0
        {
            return;
        }

        // Apply transformations
        let rotate = *state.style.borrow_mut().rotate.get(entity).unwrap_or(&0.0);
        let scaley = state
            .style
            .borrow_mut()
            .scaley
            .get(entity)
            .cloned()
            .unwrap_or_default();

        canvas.save();
        canvas.translate(posx + width / 2.0, posy + height / 2.0);
        canvas.rotate(rotate.to_radians());
        canvas.translate(-(posx + width / 2.0), -(posy + height / 2.0));

        //let pt = canvas.transform().inversed().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        // canvas.translate(pt.0, pt.1);
        // canvas.scale(1.0, scaley.0);
        // canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        let clip_entity = state.transform.get_clip_widget(entity);

        let clip_posx = state.transform.get_posx(clip_entity);

        let clip_posy = state.transform.get_posy(clip_entity);

        let clip_width = state.transform.get_width(clip_entity);

        let clip_height = state.transform.get_height(clip_entity);

        canvas.scissor(clip_posx, clip_posy, clip_width, clip_height);
        //canvas.scissor(0.0, 0.0, 100.0, 100.0);

        let shadow_h_offset = match state
            .style
            .borrow_mut()
            .shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let shadow_v_offset = match state
            .style
            .borrow_mut()
            .shadow_v_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let shadow_blur = match state
            .style
            .borrow_mut()
            .shadow_blur
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let shadow_color = state
            .style
            .borrow_mut()
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let mut shadow_color: femtovg::Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        // Draw shadow (TODO)
        let mut path = Path::new();
        path.rect(
            posx + (border_width / 2.0) - shadow_blur + shadow_h_offset,
            posy + (border_width / 2.0) - shadow_blur + shadow_v_offset,
            width - border_width + 2.0 * shadow_blur,
            height - border_width + 2.0 * shadow_blur,
        );
        // path.rounded_rect_varying(
        //     posx + (border_width / 2.0),
        //     posy + (border_width / 2.0),
        //     width - border_width,
        //     height - border_width,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.solidity(Solidity::Hole);
        //let mut paint = Paint::color(shadow_color);

        let mut paint = Paint::box_gradient(
            posx + (border_width / 2.0) + shadow_h_offset,
            posy + (border_width / 2.0) + shadow_v_offset,
            width - border_width,
            height - border_width,
            border_radius_top_left,
            shadow_blur,
            shadow_color,
            femtovg::Color::rgba(0, 0, 0, 0),
        );

        canvas.fill_path(&mut path, paint);

        // Draw rounded rect
        let mut path = Path::new();
        path.rounded_rect_varying(
            posx + (border_width / 2.0),
            posy + (border_width / 2.0),
            width - border_width,
            height - border_width,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        let mut paint = Paint::color(background_color);
        canvas.fill_path(&mut path, paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        let text_align = state
            .style
            .borrow_mut()
            .text_align
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let text_justify = state
            .style
            .borrow_mut()
            .text_justify
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let font_size = state
            .style
            .borrow_mut()
            .font_size
            .get(entity)
            .cloned()
            .unwrap_or(16.0);

        // Draw text
        if let Some(text) = state.style.borrow_mut().text.get_mut(entity) {
            let font_id = match text.font.as_ref() {
                "Sans" => state.fonts.regular.unwrap(),
                "Icons" => state.fonts.icons.unwrap(),
                _ => state.fonts.regular.unwrap(),
            };

            let mut x = posx + (border_width / 2.0);
            let mut y = posy + (border_width / 2.0);

            let text_string = text.text.to_owned();

            let align = match text_justify {
                Justify::Start => {
                    x += padding_left;
                    femtovg::Align::Left
                }
                Justify::Center => {
                    x += 0.5 * width;
                    femtovg::Align::Center
                }
                Justify::End => {
                    x += width - padding_right;
                    femtovg::Align::Right
                }
            };

            let baseline = match text_align {
                crate::style::Align::Start => {
                    y += padding_top;
                    Baseline::Top
                }
                crate::style::Align::Center => {
                    y += 0.5 * height;
                    Baseline::Middle
                }
                crate::style::Align::End => {
                    y += height - padding_bottom;
                    Baseline::Bottom
                }
            };

            let mut font_color: femtovg::Color = font_color.into();
            font_color.set_alphaf(font_color.a * opacity);

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);
            paint.set_anti_alias(false);

            canvas.fill_text(x, y, &text_string, paint);
        }

        canvas.restore();
    }
}

#[derive(Default)]
pub struct DefaultDrawHandler {}

impl DrawHandler for DefaultDrawHandler {}

pub struct EventData {
    pub target: Entity,
}

#[derive(Default)]
pub struct Handlers {
    pub draw_handlers: FnvHashMap<Entity, Box<dyn DrawHandler>>,
    pub event_handlers: FnvHashMap<Entity, Vec<Box<dyn FnMut(&mut State, &Handle, &mut Event) -> bool + 'static>>>,
    pub components: FnvHashMap<(Entity, TypeId), Rc<dyn Any + 'static>>,
}

pub struct State {
    entity_manager: EntityManager, // Creates and destroys entities
    pub hierarchy: Hierarchy,      // The widget tree
    pub style: Rc<RefCell<Style>>, // The style properties for every widget
    pub transform: Transform,      // Transform properties for all widgets
    pub root: Entity,
    pub mouse: MouseState,
    pub modifiers: ModifiersState,
    pub hovered: Entity,
    pub active: Entity,
    pub captured: Entity,
    pub focused: Entity,

    pub event_handlers: FnvHashMap<Entity, Box<dyn EventHandler>>,
    
    pub handlers: Rc<RefCell<Handlers>>,

    pub event_queue: VecDeque<Event>,

    pub fonts: Fonts, //TODO - Replace with resource manager

    pub resource_manager: ResourceManager, //TODO

    pub handles: Vec<Handle>,
}

impl State {
    pub fn new() -> Self {
        let mut entity_manager = EntityManager::new();
        let hierarchy = Hierarchy::new();
        let mut style = Rc::new(RefCell::new(Style::new()));
        let mut transform = Transform::new();
        let mouse = MouseState::default();
        let modifiers = ModifiersState::default();

        let root = entity_manager
            .create_entity()
            .expect("Failed to create root");

        transform.add(root);
        style.borrow_mut().add(root);

        style.borrow_mut().clip_widget.set(root, root);

        style
            .borrow_mut()
            .background_color
            .insert(root, Color::rgb(80, 80, 80));

        Self {
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
            //draw_handlers: FnvHashMap::default(),
            // event handlers
            //handlers: FnvHashMap::default(),
            handlers: Rc::new(RefCell::new(Handlers::default())),
            //components: FnvHashMap::default(),
            event_queue: VecDeque::new(),
            fonts: Fonts {
                regular: None,
                bold: None,
                icons: None,
            },
            resource_manager: ResourceManager::new(),
            handles: Vec::new(),
        }
    }

    pub fn build<T>(&mut self, entity: Entity, event_handler: T)
    where
        T: EventHandler + 'static + Send,
    {
        self.event_handlers.insert(entity, Box::new(event_handler));
    }

    pub fn insert_event_handler<E, F>(&mut self, entity: Entity, mut handler: F)
    where
        E: Message,
        F: FnMut(&mut State, &Handle, &EventData, &mut E) -> bool + 'static,
    {
        self.add_erased_handler(
            entity,
            Box::new(move |state, handle, event| {
                if let Some(e) = event.message.downcast::<E>() {
                    (handler)(
                        state,
                        handle,
                        &EventData {
                            target: event.target,
                        },
                        e,
                    )
                } else {
                    false
                }
            }),
        );
    }

    // pub fn insert_event_handler2<C, E, F>(&mut self, entity: Entity, mut handler: F)
    // where
    //     C: Message,
    //     E: Message,
    //     F: FnMut(&mut C, &Handle, &Meta, &mut E) -> bool + 'static
    // {
    //     self.add_erased_handler(entity, Box::new(move |state, handle, event| {
    //         if let Some(boxed_component) = state.components.get_mut(&(entity, TypeId::of::<C>())) {
    //             if let Some(component) = boxed_component.downcast::<C>() {
    //                 if let Some(e) = event.message.downcast::<E>() {
    //                     (handler)(component, handle, &Meta{target: event.target}, e)
    //                 } else {
    //                     false
    //                 }
    //             } else {
    //                 false
    //             }
    //         } else {
    //             false
    //         }

    //     }));
    // }

    pub fn insert_event_handler3<C, E, F>(&mut self, entity: Entity, mut handler: F)
    where
        C: std::any::Any + 'static,
        E: Message,
        F: FnMut(&mut C, &mut State, &Handle, &EventData, &mut E) -> bool + 'static,
    {
        let rc_component: Rc<RefCell<C>> = self
            .handlers
            .borrow_mut()
            .components
            .get(&(entity, TypeId::of::<C>()))
            .and_then(|rc| Rc::downcast(rc.clone()).ok())
            .unwrap(); // !!!
        self.add_erased_handler(
            entity,
            Box::new(move |state, handle, event| {
                if let Some(e) = event.message.downcast::<E>() {
                    (handler)(
                        &mut *rc_component.borrow_mut(),
                        state,
                        handle,
                        &EventData {
                            target: event.target,
                        },
                        e,
                    )
                } else {
                    false
                }
            }),
        );
    }

    fn add_erased_handler(
        &mut self,
        entity: Entity,
        handler: Box<dyn FnMut(&mut State, &Handle, &mut Event) -> bool + 'static>,
    ) {
        if let Some(handlers) = self.handlers.borrow_mut().event_handlers.get_mut(&entity) {
            handlers.push(handler);
            return;
        } 
        
        let mut handlers = Vec::new();
        handlers.push(handler);
        self.handlers.borrow_mut().event_handlers.insert(entity, handlers);
        
    }

    // pub fn add_component<C: hecs::Component>(&mut self, handle: &mut Handle, component: C) {
    //     handle.hecs_entity = self.world.spawn((component,));

    // }

    pub fn add_component2<C: 'static + Any + Clone + PartialEq>(
        &mut self,
        handle: &Handle,
        component: C,
    ) {
        self.handlers.borrow_mut().components.insert(
            (handle.entity, component.type_id()),
            Rc::new(RefCell::new(component)),
        );
    }

    // pub fn insert_handler<E: Message, Q: hecs::Query, F: FnMut(&Handle, hecs::QueryItem<'_,Q>, &mut E) -> bool + 'static>(&mut self, handle: &Handle, mut handler: F) {
    //     self.add_erased_handler(handle.entity, Box::new(move |state, handle, event|{

    //         if let Some(e) = event.message.downcast::<E>() {
    //             if let Ok(components) = state.world.query_one_mut::<Q>(handle.hecs_entity) {
    //                 (handler)(handle, components, e)
    //             } else {
    //                 false
    //             }

    //         } else {
    //             false
    //         }
    //     }));
    // }

    // fn add_erased_handler2(&mut self, entity: hecs::Entity, handler: Box<dyn FnMut(&mut hecs::World) -> bool + 'static>) {
    //     println!("Do Something");
    // }

    // pub fn insert_handler2<Q: hecs::Query, F>(&mut self, mut handler: F, e: hecs::Entity)
    // where F: FnMut(<<Q as hecs::Query>::Fetch as hecs::Fetch>::Item) -> bool + 'static {
    //     self.add_erased_handler2(e, Box::new(move |world| {
    //         if let Ok(components) = world.query_one_mut::<Q>(e) {
    //             (handler)(components)
    //         } else {
    //             false
    //         }
    //     }));
    // }

    pub fn insert_draw_handler<D: DrawHandler + 'static>(
        &mut self,
        entity: Entity,
        draw_handler: D,
    ) {
        self.handlers.borrow_mut().draw_handlers.insert(entity, Box::new(draw_handler));
    }

    pub fn add_widget(&mut self, parent: &Handle) -> Handle {
        // Create an entity
        let entity = self.add(parent.entity);

        // Call the build handler?
        //self.handles.push(Handle::new(entity, self.style.clone()));

        //self.handles.last().unwrap()
        Handle::new(entity, self.style.clone(), self.handlers.clone())
    }

    pub fn insert_stylesheet(&mut self, path: &str) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path.clone())?;
        self.resource_manager.stylesheets.push(path.to_owned());

        // Parse the theme stylesheet
        self.style.borrow_mut().parse_theme(&style_string);
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
            return Ok(());
        }

        // Remove all non-inline style data
        self.style.borrow_mut().background_color.remove_styles();
        self.style.borrow_mut().font_color.remove_styles();

        // Position
        self.style.borrow_mut().left.remove_styles();
        self.style.borrow_mut().right.remove_styles();
        self.style.borrow_mut().top.remove_styles();
        self.style.borrow_mut().bottom.remove_styles();
        // Size
        self.style.borrow_mut().width.remove_styles();
        self.style.borrow_mut().height.remove_styles();
        // Margins
        self.style.borrow_mut().margin_left.remove_styles();
        self.style.borrow_mut().margin_right.remove_styles();
        self.style.borrow_mut().margin_top.remove_styles();
        self.style.borrow_mut().margin_bottom.remove_styles();
        // Padding
        self.style.borrow_mut().padding_left.remove_styles();
        self.style.borrow_mut().padding_right.remove_styles();
        self.style.borrow_mut().padding_top.remove_styles();
        self.style.borrow_mut().padding_bottom.remove_styles();
        // Border
        self.style.borrow_mut().border_width.remove_styles();
        self.style.borrow_mut().border_color.remove_styles();
        // Border Radius
        self.style
            .borrow_mut()
            .border_radius_top_left
            .remove_styles();
        self.style
            .borrow_mut()
            .border_radius_top_right
            .remove_styles();
        self.style
            .borrow_mut()
            .border_radius_bottom_left
            .remove_styles();
        self.style
            .borrow_mut()
            .border_radius_bottom_right
            .remove_styles();
        // Flexbox
        self.style.borrow_mut().flex_grow.remove_styles();
        self.style.borrow_mut().flex_shrink.remove_styles();
        self.style.borrow_mut().flex_basis.remove_styles();
        self.style.borrow_mut().align_self.remove_styles();
        self.style.borrow_mut().align_content.remove_styles();
        // Flex Container
        self.style.borrow_mut().align_items.remove_styles();
        self.style.borrow_mut().justify_content.remove_styles();
        self.style.borrow_mut().flex_direction.remove_styles();
        // Display
        self.style.borrow_mut().display.remove_styles();
        self.style.borrow_mut().visibility.remove_styles();
        self.style.borrow_mut().opacity.remove_styles();
        // Text Alignment
        self.style.borrow_mut().text_align.remove_styles();
        self.style.borrow_mut().text_justify.remove_styles();

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

        self.style.borrow_mut().parse_theme(&overall_theme);

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
        self.style
            .borrow_mut()
            .ids
            .get_by_left(&id.to_string())
            .cloned()
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
        self.style.borrow_mut().add(entity);

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
            .borrow_mut()
            .background_color
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .font_color
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .border_color
            .animate(std::time::Instant::now());

        self.style
            .borrow_mut()
            .left
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .right
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .top
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .bottom
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .width
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .height
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .opacity
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .rotate
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .flex_grow
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .flex_shrink
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .flex_basis
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .margin_left
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .margin_right
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .margin_top
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .margin_bottom
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .padding_left
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .padding_right
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .padding_top
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .padding_bottom
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .border_radius_top_left
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .border_radius_top_right
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .border_radius_bottom_left
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .border_radius_bottom_right
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .border_width
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .min_width
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .max_width
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .min_height
            .animate(std::time::Instant::now());
        self.style
            .borrow_mut()
            .max_height
            .animate(std::time::Instant::now());

        self.style.borrow().background_color.has_animations()
            || self.style.borrow().font_color.has_animations()
            || self.style.borrow().border_color.has_animations()
            || self.style.borrow().left.has_animations()
            || self.style.borrow().right.has_animations()
            || self.style.borrow().top.has_animations()
            || self.style.borrow().bottom.has_animations()
            || self.style.borrow().width.has_animations()
            || self.style.borrow().height.has_animations()
            || self.style.borrow().opacity.has_animations()
            || self.style.borrow().rotate.has_animations()
            || self.style.borrow().flex_grow.has_animations()
            || self.style.borrow().flex_shrink.has_animations()
            || self.style.borrow().flex_basis.has_animations()
            || self.style.borrow().margin_left.has_animations()
            || self.style.borrow().margin_right.has_animations()
            || self.style.borrow().margin_top.has_animations()
            || self.style.borrow().margin_bottom.has_animations()
            || self.style.borrow().padding_left.has_animations()
            || self.style.borrow().padding_right.has_animations()
            || self.style.borrow().padding_top.has_animations()
            || self.style.borrow().padding_bottom.has_animations()
            || self.style.borrow().border_radius_top_left.has_animations()
            || self.style.borrow().border_radius_top_right.has_animations()
            || self
                .style
                .borrow()
                .border_radius_bottom_left
                .has_animations()
            || self
                .style
                .borrow()
                .border_radius_bottom_right
                .has_animations()
            || self.style.borrow().border_width.has_animations()
            || self.style.borrow().min_width.has_animations()
            || self.style.borrow().max_width.has_animations()
            || self.style.borrow().min_height.has_animations()
            || self.style.borrow().max_height.has_animations()
    }

    pub fn get_root(&self) -> Entity {
        self.root
    }
}
