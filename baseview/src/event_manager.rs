use std::collections::HashMap;
use tuix_core::{
    Color, CursorIcon, Entity, Event, EventHandler, Hierarchy, IntoHierarchyIterator,
    IntoParentIterator, State, WindowEvent,
};

use crate::Renderer;
use femtovg::Canvas;

pub struct EventManager {
    pub event_handlers: HashMap<Entity, Box<dyn EventHandler>>,
    pub event_queue: Vec<Event>,
    _needs_redraw: bool,
    _total_frames: usize,
    pub start_time: std::time::Instant,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_handlers: HashMap::new(),
            event_queue: Vec::new(),
            _needs_redraw: false,
            _total_frames: 0,
            start_time: std::time::Instant::now(),
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

        //let mut draw_hierarchy: Vec<Entity> = state.hierarchy.into_iter().collect();

        //draw_hierarchy.sort_by_cached_key(|entity| state.transform.get_z_order(*entity));

        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Move event handlers from state to event manager
        self.event_handlers.extend(state.event_handlers.drain());

        // Move events from state into event manager
        let event_queue = state.event_queue.clone();
        //self.event_queue.append(&mut state.event_queue);
        self.event_queue = event_queue.into_iter().collect::<Vec<Event>>();

        self.event_queue.sort_by_cached_key(|event| event.order);

        // Clear the event queue in state
        state.event_queue.clear();

        // Loop over the events in the event manager queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);
            // If a redraw is needed then set the flag to return true

            if let Some(window_event) = event.message.downcast::<WindowEvent>() {
                match window_event {
                    WindowEvent::Redraw => {
                        needs_redraw = true;
                    }

                    WindowEvent::SetCursor(cursor_icon) => {
                        match cursor_icon {
                            CursorIcon::Arrow => {
                                //window.handle.window().set_cursor_icon(glutin::window::CursorIcon::Arrow);
                            }

                            CursorIcon::NResize => {
                                //window.handle.window().set_cursor_icon(glutin::window::CursorIcon::NResize);
                            }

                            CursorIcon::EResize => {
                                //window.handle.window().set_cursor_icon(glutin::window::CursorIcon::EResize);
                            }
                        }
                    }

                    _ => {}
                }
            }

            let target = event.target;

            // A null entity as target means send event to all entities
            if event.target == Entity::null() {
                for entity in hierarchy.entities.iter() {
                    //println!("Entity: {}", entity);
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        if event_handler.on_event(state, *entity, event) {
                            break;
                        }
                    }
                }
                continue 'events;
            }

            // Propagate down from root to target (not including target)
            if event.get_propagate_down() {
                // Walk down the hierarchy
                for entity in hierarchy.into_iter() {
                    // Stop before the target entity
                    if entity == event.target {
                        break;
                    }

                    // Send event to all entities before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        if event_handler.on_event(state, entity, event) {
                            continue 'events;
                        }
                    }
                }
            }

            // Send event to target
            if let Some(event_handler) = self.event_handlers.get_mut(&event.target) {
                if event_handler.on_event(state, event.target, event) {
                    continue 'events;
                }
            }

            // Propagate up from target to root (not including target)
            if event.get_propagate_up() {
                // Walk up the hierarchy from parent to parent
                for entity in target.parent_iter(&hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        if event_handler.on_event(state, entity, event) {
                            continue 'events;
                        }
                    }
                }
            }

            // Propagate down from target to leaf
            if event.get_propagate_fall() {
                // Walk hierarchy from the target down the branch
                for widget in target.into_iter(&hierarchy) {
                    // Skip the target entity
                    if widget == event.target {
                        continue;
                    }

                    if let Some(event_handler) = self.event_handlers.get_mut(&widget) {
                        if event_handler.on_event(state, widget, event) {
                            continue 'events;
                        }
                    }
                }
            }
        }

        return needs_redraw;
    }

    pub fn draw(
        &mut self,
        state: &mut State,
        hierarchy: &Hierarchy,
        canvas: &mut Canvas<Renderer>,
    ) {
        //let dpi_factor = window.handle.window().scale_factor();
        //let size = window.handle.window().inner_size();

        //window.canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        //window.canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.3, 0.3, 0.32));

        let clear_color = state.style.background_color.get(state.root);
        let clear_color = if let Some(clear_color) = clear_color {
            *clear_color
        } else {
            Color::rgb(77, 77, 82)
        };

        canvas.clear_rect(
            0,
            0,
            state.transform.get_width(state.root).round() as u32,
            state.transform.get_height(state.root).round() as u32,
            clear_color.into(),
        );

        let mut draw_hierarchy: Vec<Entity> = hierarchy.into_iter().collect();
        draw_hierarchy.sort_by_cached_key(|entity| state.transform.get_z_order(*entity));

        for widget in draw_hierarchy.into_iter() {
            if let Some(event_handler) = self.event_handlers.get_mut(&widget) {
                event_handler.on_draw(state, widget, canvas);
            }
        }

        canvas.flush();

        // window
        //     .handle
        //     .swap_buffers()
        //     .expect("Failed to swap buffers");
    }
}
