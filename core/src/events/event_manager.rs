use crate::{Builder, CursorIcon, Entity, Event, Hierarchy, HierarchyTree, ImageOrId, IntoBranchIterator, IntoHierarchyIterator, IntoParentIterator, PropSet, Propagation, State, WindowEvent, hierarchy};

use crate::EventHandler;

use std::{collections::{HashMap, VecDeque, hash_map::DefaultHasher}, convert::TryInto, println};

use std::time::{Duration, Instant};

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId,
    LineCap, LineJoin, Paint, Path, Renderer, Solidity, 
};

use fnv::FnvHashMap;

pub struct EventManager {
    event_handlers: FnvHashMap<Entity, Box<dyn EventHandler>>,

    // Queue of events to be processed
    pub event_queue: Vec<Event>,

    // A copy of the hierarchy for iteration
    pub hierarchy: Hierarchy,

    prev_width: f32,
    prev_height: f32,
    prev_dpi_factor: f64,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_handlers: FnvHashMap::default(),
            event_queue: Vec::new(),

            hierarchy: Hierarchy::new(),

            prev_width: 0.0,
            prev_height: 0.0,
            prev_dpi_factor: 1.0,
        }
    }

    pub fn flush_events(&mut self, state: &mut State) -> bool {
        let mut needs_redraw = false;

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

            // Skip events with no target unless they are set to propagate to all entities
            if event.target == Entity::null() && event.propagation != Propagation::All {
                continue 'events;
            }

            if let Some(window_event) = event.message.downcast::<WindowEvent>() {
                match window_event {
                    WindowEvent::Redraw => {
                        needs_redraw = true;
                    }

                    _ => {}
                }
            }

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // A null entity as target means send event to all entities
            if event.propagation == Propagation::All {
                for entity in self.hierarchy.into_iter() {
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event_(state, entity, event);

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
                    .parent_iter(&self.hierarchy)
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
                        event_handler.on_event_(state, *entity, event);

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
                    event_handler.on_event_(state, event.target, event);

                    if event.consumed {
                        continue 'events;
                    }
                }
            }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up || event.propagation == Propagation::DownUp {
                // Walk up the hierarchy from parent to parent
                for entity in target.parent_iter(&self.hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event_(state, entity, event);

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
                for entity in target.branch_iter(&self.hierarchy) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities after the target on the same branch
                    if let Some(event_handler) = self.event_handlers.get_mut(&entity) {
                        event_handler.on_event_(state, entity, event);

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

    pub fn draw(&mut self, state: &mut State, canvas: &mut Canvas<OpenGl>) {
        //let dpi_factor = window.handle.window().scale_factor();
        //let size = window.handle.window().inner_size();

        // for (resource, image_or_id) in state.resource_manager.image_ids.iter_mut() {
        //     match image_or_id {
        //         ImageOrId::Image(data, width, height) => {
        //             image_or_id = 
        //         }
        //     }
        // }

        state.resource_manager.image_ids.iter_mut().for_each(|(_, image_or_id)| {
            match image_or_id {
                ImageOrId::Image(image) => {
                    //let img = image.clone();
                    //let image: femtovg::ImageSource = (&img).try_into().unwrap();
                    let image: femtovg::ImageSource = (&*image).try_into().unwrap();
                    *image_or_id = ImageOrId::Id(canvas.create_image(image, ImageFlags::empty()).unwrap())
                }
                _=> {}
            }
        });

        let width = state.data.get_width(Entity::root());
        let height = state.data.get_height(Entity::root());
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
            .get(Entity::root())
            .cloned()
            .unwrap_or_default()
            .into();

        // Clear the canvas
        canvas.clear_rect(0, 0, width as u32, height as u32, background_color);

        // Reset any canvas transforms
        canvas.reset();

        // Sort the hierarchy by z order
        let mut draw_hierarchy: Vec<Entity> = self.hierarchy.into_iter().collect();
        draw_hierarchy.sort_by_cached_key(|entity| state.data.get_z_order(*entity));

        // Call the on_draw() method for each widget
        for widget in draw_hierarchy.into_iter() {
            if let Some(event_handler) = self.event_handlers.get_mut(&widget) {
                event_handler.on_draw_(state, widget, canvas);
            }
        }

        // Send the canvas to the GPU to draw
        canvas.flush();
    }
}
