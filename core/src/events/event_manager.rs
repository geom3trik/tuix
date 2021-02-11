use crate::{
    BuildHandler, Builder, CursorIcon, Entity, Event, EventHandler, Hierarchy, HierarchyTree,
    IntoBranchIterator, IntoHierarchyIterator, IntoParentIterator, PropSet, Propagation, State,
    WindowEvent,
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
    // List of event handlers
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

    pub fn flush_events(&mut self, state: &mut State) -> bool {
        //println!("FLUSH");
        let mut needs_redraw = false;

        // Copy the hierarchy from state
        let hierarchy = state.hierarchy.clone();

        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Move event handlers from state to event manager
        self.event_handlers.extend(state.event_handlers.drain());

        // Move events from state into event manager
        let event_queue = state.event_queue.clone();

        // Sort the events by order
        self.event_queue = event_queue.into_iter().collect::<Vec<Event>>();
        self.event_queue.sort_by_cached_key(|event| event.order);

        // Clear the event queue in state
        state.event_queue.clear();

        // Loop over the events in the event manager queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);

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

            let target = event.target;

            // A null entity as target means send event to all entities
            if event.target == Entity::null() {
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
                let ancestors: Vec<Entity> =
                    target.parent_iter(&hierarchy).collect::<Vec<Entity>>();

                for entity in ancestors.iter().rev() {
                    // Skip the window
                    if *entity == Entity::new(0, 0) {
                        continue;
                    }

                    // Stop before the target entity
                    if *entity == event.target {
                        break;
                    }

                    //println!("Ancestor: {:?}", entity);

                    // Send event to all ancestors before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, *entity, event);

                        if event.consumed {
                            continue 'events;
                        }
                    }
                }

                //Walk down the hierarchy
                // for entity in hierarchy.into_iter() {
                //     // Stop before the target entity
                //     if entity == event.target {
                //         break;
                //     }

                //     // Send event to all entities before the target
                //     if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                //         if event_handler.on_event(state, entity, event) {
                //             continue 'events;
                //         }
                //     }
                // }
            }

            // Send event to target
            if let Some(event_handler) = self.event_handlers.get_mut(&event.target) {
                event_handler.on_event(state, event.target, event);

                if event.consumed {
                    continue 'events;
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

                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            // Propagate down from target to leaf
            if event.propagation == Propagation::Fall {
                // Walk hierarchy from the target down the branch
                for entity in target.branch_iter(&hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event(state, entity, event);

                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }
        }

        return needs_redraw;
    }

    pub fn draw(&mut self, state: &mut State, hierarchy: &Hierarchy, canvas: &mut Canvas<OpenGl>) {
        //let dpi_factor = window.handle.window().scale_factor();
        //let size = window.handle.window().inner_size();

        let width = state.data.get_width(state.root);
        let height = state.data.get_height(state.root);
        // TODO: Move this to the window widget
        let dpi_factor = 1.0;

        // Set the canvas size
        if (self.prev_width != width
            || self.prev_height != height
            || self.prev_dpi_factor != dpi_factor)
        {
            canvas.set_size(width as u32, height as u32, dpi_factor as f32);
        }

        // Get the desired window background color
        let background_color: femtovg::Color = state
            .style
            .background_color
            .get(state.root)
            .cloned()
            .unwrap_or_default()
            .into();

        // Clear the canvas
        canvas.clear_rect(0, 0, width as u32, height as u32, background_color);

        // Reset any canvas transforms
        canvas.reset();

        // Sort the hierarchy by z order
        let mut draw_hierarchy: Vec<Entity> = hierarchy.into_iter().collect();
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
