use crate::{
    BuildHandler, Builder, CursorIcon, Entity, Event, EventHandler, Hierarchy, HierarchyTree,
    IntoBranchIterator, IntoHierarchyIterator, IntoParentIterator, PropSet, Propagation, State,
    WindowEvent, AppEvent,
};
use std::{
    collections::{HashMap, VecDeque},
    println,
};

use std::time::{Duration, Instant};

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId,
    LineCap, LineJoin, Paint, Path, Renderer, Solidity,
};

use fnv::FnvHashMap;

pub struct EventManager {
    pub event_handlers: FnvHashMap<Entity, Box<dyn EventHandler>>,

    // Queue of events to be processed
    pub event_queue: Vec<Event>,

    prev_width: f32,
    prev_height: f32,
    prev_dpi_factor: f64,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_handlers: FnvHashMap::default(),
            event_queue: Vec::new(),

            prev_width: 0.0,
            prev_height: 0.0,
            prev_dpi_factor: 1.0,
        }
    }

    // pub fn insert_event(&mut self, event: Event) {
    //     self.event_queue.push_back(event);
    // }

    pub fn flush_events2(&mut self, state: &mut State) -> bool {
        let mut needs_redraw = false;

        // Clone the hierarchy from state
        let hierarchy = state.hierarchy.clone();

        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Move event handlers from state to event manager
        self.event_handlers.extend(state.event_handlers.drain());

        // Remove widgets that should be removed
        for entity in state.removed_entities.iter() {
            self.event_handlers.remove(entity);            
        }

        state.removed_entities.clear();


        // Clone events from state into event manager
        let event_queue = state.event_queue.clone();

        // Sort the events by order
        self.event_queue = event_queue.into_iter().collect::<Vec<Event>>();
        self.event_queue.sort_by_cached_key(|event| event.order);

        // Clear the event queue in state
        state.event_queue.clear();

        // Loop over the events in the event manager queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);

            if event.target == Entity::null() {
                continue 'events;
            }

            if let Some(window_event) = event.message.downcast::<WindowEvent>() {
                match window_event {
                    WindowEvent::Redraw => {
                        
                        needs_redraw = true;
                    }

                    /*
                    WindowEvent::SetCursor(cursor_icon) => match cursor_icon {
                        CursorIcon::Arrow => {
                            window
                                .handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::Arrow);
                        }

                        CursorIcon::NResize => {
                            window
                                .handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NResize);
                        }

                        CursorIcon::EResize => {
                            window
                                .handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::EResize);
                        }
                    },
                    */
                    _ => {}
                }
            }

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // A null entity as target means send event to all entities
            if event.propagation == Propagation::All {
                for entity in hierarchy.into_iter() {
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        if event.consumed {
                            break;
                        }
                    }
                }
                continue 'events;
            }

            // Propagate down from root to target (not including target)
            if event.propagation == Propagation::Down || event.propagation == Propagation::DownUp {
                // Construct the list of widgets to walk down by going up from the target
                let ancestors: Vec<Entity> = event
                    .target
                    .parent_iter(&hierarchy)
                    .collect::<Vec<Entity>>();

                // Walk down the list of ancestors
                for entity in ancestors.iter().rev() {
                    // Skip the window
                    if *entity == Entity::root() {
                        continue;
                    }

                    // Stop before the target entity
                    if *entity == event.target {
                        break;
                    }

                    // Send event to all ancestors before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, *entity, event);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            if event.propagation != Propagation::Fall {
                // Send event to target
                if let Some(event_handler) = self.event_handlers.get_mut(&event.target) {
                    event_handler.on_event(state, event.target, event);

                    if event.consumed {
                        continue 'events;
                    }
                }
            }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up || event.propagation == Propagation::DownUp {
                // Walk up the hierarchy from parent to parent
                for entity in target.parent_iter(&hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            // Propagate down from target to leaf of current branch
            if event.propagation == Propagation::Fall {
                // Walk hierarchy from the target down the branch
                for entity in target.branch_iter(&hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities after the target on the same branch
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }
        }

        return needs_redraw;
    }


    pub fn flush_events<H: FnOnce(&mut FnvHashMap<Entity, Box<dyn EventHandler>>, &mut AppEvent)>(&mut self, state: &mut State, handler: H) -> bool {
        let mut needs_redraw = false;

        // Clone the hierarchy from state
        let hierarchy = state.hierarchy.clone();

        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Move event handlers from state to event manager
        self.event_handlers.extend(state.event_handlers.drain());

        // Clone events from state into event manager
        let event_queue = state.event_queue.clone();

        // Sort the events by order
        self.event_queue = event_queue.into_iter().collect::<Vec<Event>>();
        self.event_queue.sort_by_cached_key(|event| event.order);

        // Clear the event queue in state
        state.event_queue.clear();

        // Loop over the events in the event manager queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);

            if event.target == Entity::null() {
                continue 'events;
            }

            if let Some(app_event) = event.message.downcast::<AppEvent>() {
                handler(&mut self.event_handlers, app_event);
                return true;
            }  

            if let Some(window_event) = event.message.downcast::<WindowEvent>() {
                match window_event {
                    WindowEvent::Redraw => {
                        needs_redraw = true;
                    }

                    /*
                    WindowEvent::SetCursor(cursor_icon) => match cursor_icon {
                        CursorIcon::Arrow => {
                            window
                                .handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::Arrow);
                        }

                        CursorIcon::NResize => {
                            window
                                .handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NResize);
                        }

                        CursorIcon::EResize => {
                            window
                                .handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::EResize);
                        }
                    },
                    */
                    _ => {}
                }
            }

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // A null entity as target means send event to all entities
            if event.propagation == Propagation::All {
                for entity in hierarchy.into_iter() {
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        if event.consumed {
                            break;
                        }
                    }
                }
                continue 'events;
            }

            // Propagate down from root to target (not including target)
            if event.propagation == Propagation::Down || event.propagation == Propagation::DownUp {
                // Construct the list of widgets to walk down by going up from the target
                let ancestors: Vec<Entity> = event
                    .target
                    .parent_iter(&hierarchy)
                    .collect::<Vec<Entity>>();

                // Walk down the list of ancestors
                for entity in ancestors.iter().rev() {
                    // Skip the window
                    if *entity == Entity::root() {
                        continue;
                    }

                    // Stop before the target entity
                    if *entity == event.target {
                        break;
                    }

                    // Send event to all ancestors before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, *entity, event);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            if event.propagation != Propagation::Fall {
                // Send event to target
                if let Some(event_handler) = self.event_handlers.get_mut(&event.target) {
                    event_handler.on_event(state, event.target, event);

                    if event.consumed {
                        continue 'events;
                    }
                }
            }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up || event.propagation == Propagation::DownUp {
                // Walk up the hierarchy from parent to parent
                for entity in target.parent_iter(&hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            // Propagate down from target to leaf of current branch
            if event.propagation == Propagation::Fall {
                // Walk hierarchy from the target down the branch
                for entity in target.branch_iter(&hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities after the target on the same branch
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }
        }

        return needs_redraw;
    }

    pub fn draw(&mut self, state: &mut State, hierarchy: &Hierarchy, window: Entity, canvas: &mut Canvas<OpenGl>) {
        //let dpi_factor = window.handle.window().scale_factor();
        //let size = window.handle.window().inner_size();

        let width = state.data.get_width(window);
        let height = state.data.get_height(window);
        // TODO: Move this to the window widget
        let dpi_factor = 1.0;

        // Set the canvas size
        // if (self.prev_width != width
        //     || self.prev_height != height
        //     || self.prev_dpi_factor != dpi_factor)
        // {
            canvas.set_size(width as u32, height as u32, dpi_factor as f32);
        //}

        //println!("width: {}  height: {}", width, height);

        // Get the desired window background color
        let background_color: femtovg::Color = state
            .style
            .background_color
            .get(window)
            .cloned()
            .unwrap_or_default()
            .into();


        // Clear the canvas
        canvas.clear_rect(0, 0, width as u32, height as u32, background_color);

        // Reset any canvas transforms
        canvas.reset();

        let mut draw_hierarchy = Vec::new();
        let mut temp = Some(window);
        let mut iterator = window.into_iter(&hierarchy);
        while temp.is_some() {
            temp = iterator.next();
            if let Some(entity) = temp {
                let parent_window = state.data.get_window(entity);
                if parent_window != window {
                    temp = iterator.next_branch();
                }
                draw_hierarchy.push(entity);
            }
        }

        
        //let mut draw_hierarchy: Vec<Entity> = window.into_iter(&hierarchy).collect();
        // Sort the hierarchy by z order
        //let mut draw_hierarchy: Vec<Entity> = hierarchy.into_iter().collect();
        draw_hierarchy.sort_by_cached_key(|entity| state.data.get_z_order(*entity));

        // Call the on_draw() method for each widget
        for widget in draw_hierarchy.into_iter() {
            if let Some(event_handler) = self.event_handlers.get_mut(&widget) {
                event_handler.on_draw(state, widget, canvas);
            }
        }

        // Send the canvas to the GPU to draw
        canvas.flush();
    }
}
