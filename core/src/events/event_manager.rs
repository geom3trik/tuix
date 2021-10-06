use crate::{Entity, Event, FontOrId, Propagation, State, Tree, TreeExt, Visibility, WindowEvent};


use femtovg::{
    renderer::OpenGl, Canvas,
};

pub struct EventManager {

    // Queue of events to be processed
    pub event_queue: Vec<Event>,

    // A copy of the tree for iteration
    pub tree: Tree,

    prev_width: f32,
    prev_height: f32,
    prev_dpi_factor: f64,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_queue: Vec::new(),

            tree: Tree::new(),

            prev_width: 0.0,
            prev_height: 0.0,
            prev_dpi_factor: 1.0,
        }
    }

    pub fn flush_events(&mut self, state: &mut State) -> bool {
        let mut needs_redraw = false;
        let mut needs_restyle = false;
        let mut needs_relayout = false;

        if state.tree.changed {
            self.tree = state.tree.clone();
            state.tree.changed = false;
        }

        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Remove widgets that should be removed
        // for entity in state.removed_entities.iter() {
        //     self.event_handlers.remove(entity);
        // }

        //state.removed_entities.clear();

        // Move events from state to event manager
        self.event_queue.extend(state.event_queue.drain(0..));

        // Sort the events by order
        //self.event_queue.sort_by_cached_key(|event| event.order);

        // Loop over the events in the event queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);

            // Send events to any listeners
            let listeners = state.listeners.iter().map(|(entity, _)| *entity).collect::<Vec<Entity>>();
            for entity in listeners {
                if let Some(listener) = state.listeners.remove(&entity) {
                    if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                        (listener)(event_handler.as_mut(), state, entity, event);

                        state.event_handlers.insert(entity, event_handler);
                    }
                    

                    state.listeners.insert(entity, listener);
                }

                if event.consumed {
                    continue 'events;
                }
            }

            // Skip events with no target unless they are set to propagate to all entities
            if event.target == Entity::null() && event.propagation != Propagation::All {
                continue 'events;
            }

            if let Some(window_event) = event.message.downcast::<WindowEvent>() {
                match window_event {
                    WindowEvent::Redraw => {
                        needs_redraw = true;
                        continue 'events;
                    }

                    WindowEvent::Relayout => {
                        needs_relayout = true;
                        continue 'events;
                    }

                    WindowEvent::Restyle => {
                        needs_restyle = true;
                        continue 'events;
                    }

                    _ => {}
                }
            }

            // if let Some(redraw_event) = event.message.downcast::<RedrawEvent>() {
            //     needs_redraw = true;
            // }

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // A null entity as target means send event to all entities
            if event.propagation == Propagation::All {
                for entity in self.tree.into_iter() {
                    if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                        event_handler.on_event_(state, entity, event);

                        state.event_handlers.insert(entity, event_handler);

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
                    .parent_iter(&self.tree)
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
                    if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                        event_handler.on_event_(state, *entity, event);

                        state.event_handlers.insert(*entity, event_handler);

                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            // Direct
            if event.propagation != Propagation::Fall {
                // Send event to target
                if let Some(mut event_handler) = state.event_handlers.remove(&event.target) {
                    event_handler.on_event_(state, event.target, event);

                    state.event_handlers.insert(event.target, event_handler);
                    // if let Some(test) = self.callbacks.get_mut(&event.target) {
                    //     (test)(event_handler, state, event.target);
                    // }

                    if event.consumed {
                        continue 'events;
                    }
                }
            }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up || event.propagation == Propagation::DownUp {
                // Walk up the tree from parent to parent
                for entity in target.parent_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                        event_handler.on_event_(state, entity, event);

                        state.event_handlers.insert(entity, event_handler);
                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }

            // Propagate down from target to leaf of current branch
            if event.propagation == Propagation::Fall {
                // Walk tree from the target down the branch
                for entity in target.branch_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities after the target on the same branch
                    if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                        event_handler.on_event_(state, entity, event);

                        state.event_handlers.insert(entity, event_handler);
                        // Skip to the next event if the current event is consumed
                        if event.consumed {
                            continue 'events;
                        }
                    }
                }
            }
        }

        if needs_restyle {
            if let Some(mut event_handler) = state.event_handlers.remove(&Entity::root()) {
                event_handler.on_event_(state, Entity::root(), &mut Event::new(WindowEvent::Restyle));

                state.event_handlers.insert(Entity::root(), event_handler);
            }
        }

        if needs_relayout {
            if let Some(mut event_handler) = state.event_handlers.remove(&Entity::root()) {
                event_handler.on_event_(state, Entity::root(), &mut Event::new(WindowEvent::Relayout));

                state.event_handlers.insert(Entity::root(), event_handler);
            }
        }

        if needs_redraw {
            if let Some(mut event_handler) = state.event_handlers.remove(&Entity::root()) {
                event_handler.on_event_(state, Entity::root(), &mut Event::new(WindowEvent::Redraw));

                state.event_handlers.insert(Entity::root(), event_handler);
            }
        }

        return needs_redraw;
    }

    pub fn load_resources(&mut self, state: &mut State, canvas: &mut Canvas<OpenGl>) {
        for (name, font) in state.resource_manager.fonts.iter_mut() {
            
            match font {
                FontOrId::Font(data) => {
                    let id1 = canvas.add_font_mem(&data.clone()).expect(&format!("Failed to load font file for: {}", name));
                    let id2 = state.text_context.add_font_mem(&data.clone()).expect("failed");
                    if id1 != id2 {
                        panic!("Fonts in canvas must have the same id as fonts in the text context");
                    }
                    *font = FontOrId::Id(id1);
                }

                _=> {}
            }
        }
    }

    pub fn draw(&mut self, state: &mut State, window: Entity, canvas: &mut Canvas<OpenGl>) {
        //let dpi_factor = window.handle.window().scale_factor();
        //let size = window.handle.window().inner_size();

        self.load_resources(state, canvas);

        // for (resource, image_or_id) in state.resource_manager.image_ids.iter_mut() {
        //     match image_or_id {
        //         ImageOrId::Image(data, width, height) => {
        //             image_or_id =
        //         }
        //     }
        // }

        // state
        //     .resource_manager
        //     .image_ids
        //     .iter_mut()
        //     .for_each(|(_, image_or_id)| {
        //         match image_or_id {
        //             ImageOrId::Image(image) => {
        //                 //let img = image.clone();
        //                 //let image: femtovg::ImageSource = (&img).try_into().unwrap();
        //                 let image: femtovg::ImageSource = (&*image).try_into().unwrap();
        //                 *image_or_id =
        //                     ImageOrId::Id(canvas.create_image(image, ImageFlags::empty()).unwrap())
        //             }
        //             _ => {}
        //         }
        //     });

        let width = state.data.get_width(Entity::root());
        let height = state.data.get_height(Entity::root());
        // TODO: Move this to the window widget
        let dpi_factor = 1.0;

        // Set the canvas size
        if self.prev_width != width
            || self.prev_height != height
            || self.prev_dpi_factor != dpi_factor
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

        //canvas.save();
        // Reset any canvas transforms
        canvas.reset();

        // Sort the tree by z order
        //let mut draw_tree: Vec<Entity> = window.window_iter(&state.tree).collect();
        //draw_tree.sort_by_cached_key(|entity| state.data.get_z_index(*entity));

        // Call the on_draw() method for each widget
        for entity in window.window_iter(&state.tree.clone()) {
            println!("Draw: {:?}", entity);

            // Skip window
            if entity == Entity::root() {
                continue;
            }

            // Skip invisible widgets
            if state.data.get_visibility(entity) == Visibility::Invisible {
                continue;
            }

            // Skip widgets that have 0 opacity
            if state.data.get_opacity(entity) == 0.0 {
                continue;
            }

            //let bounds = state.data.get_bounds(entity);

            // Skip widgets with no width or no height
            // if bounds.w == 0.0 || bounds.h == 0.0 {
            //     continue;
            // }

            // let clip_region = state.data.get_clip_region(entity);
            // canvas.scissor(
            //     clip_region.x,
            //     clip_region.y,
            //     clip_region.w,
            //     clip_region.h,
            // );
    
            // Apply transformations
            let transform = state.data.get_transform(entity);
    
    
            canvas.save();
            // canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);
            

            if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                //let start = std::time::Instant::now();
                event_handler.on_draw_(state, entity, canvas);
                //println!("{:.2?} seconds for whatever you did.", start.elapsed());
                state.event_handlers.insert(entity, event_handler);
            }

            canvas.restore();
        }

        //canvas.restore();

        // Send the canvas to the GPU to draw
        canvas.flush();
    }
}
