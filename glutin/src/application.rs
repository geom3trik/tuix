#![allow(deprecated)]

use std::collections::HashMap;

use glutin::event_loop::{self, ControlFlow, EventLoop};
use glutin::window::WindowId;

use crate::keyboard::{scan_to_code, vcode_to_code, vk_to_key};

use crate::window::{self, CurrentContextWrapper, Window};

use tuix_core::{BoundingBox, Units};
use tuix_core::{Entity, State, PropSet};

use tuix_core::{MouseButton, MouseButtonState};

use tuix_core::events::{Event, EventManager, Propagation};

use tuix_core::TreeExt;

use tuix_core::style::{Display, Visibility};

use tuix_core::{WindowDescription, WindowEvent, WindowWidget};

use tuix_core::{apply_hover, apply_clipping};

use glutin::event::VirtualKeyCode;

type GEvent<'a, T> = glutin::event::Event<'a, T>;


#[derive(Debug)]
pub enum AppEvent {
    /// Emitted when a new window should be created by the application
    CreateWindow(Entity),
    /// Emiited when a new window should be destroyed by the application
    Destroy(Entity),
}

pub struct Application {
    //pub window: Window,
    pub state: State,
    event_loop: EventLoop<()>,
    pub event_manager: EventManager,
    pub windows: HashMap<WindowId, Entity>,
    pub root_window: WindowId,
}

impl Application {
    pub fn new<F: FnOnce(&mut State, Entity)>(
        window_description: WindowDescription,
        app: F,
    ) -> Self {
        let event_loop = EventLoop::new();
        let mut state = State::new();
        state.reload_styles().expect("Failed to reload styles");

        let mut event_manager = EventManager::new();
        let root = Entity::root();
        //state.tree.add(Entity::root(), None);

        event_manager.tree = state.tree.clone();
        
        let regular_font = include_bytes!("../fonts/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../fonts/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../fonts/entypo.ttf");
        let emoji_font = include_bytes!("../fonts/OpenSansEmoji.ttf");
        let arabic_font = include_bytes!("../fonts/amiri-regular.ttf");

        state.add_font_mem("roboto", regular_font);
        state.add_font_mem("roboto-bold", bold_font);
        state.add_font_mem("icon", icon_font);
        state.add_font_mem("emoji", emoji_font);
        state.add_font_mem("arabic", arabic_font);

        let mut window = Window::new(window_description.clone());

        let window_id = window.create(&event_loop);
        
        event_manager.load_resources(&mut state, window.canvas.as_mut().unwrap());

        println!("2 Root Win: {}", root);
        state.event_handlers.insert(root, Box::new(window));
        
        app(&mut state, root);

        let mut windows = HashMap::new();

        windows.insert(window_id, root);


        state.style.width.insert(
            Entity::root(),
            Units::Pixels(window_description.inner_size.width as f32),
        );
        state.style.height.insert(
            Entity::root(),
            Units::Pixels(window_description.inner_size.height as f32),
        );

        state
            .data
            .set_width(Entity::root(), window_description.inner_size.width as f32);
        state
            .data
            .set_height(Entity::root(), window_description.inner_size.height as f32);
        state.data.set_opacity(Entity::root(), 1.0);
    
        //state.data.set_focusable(Entity::root(), false);

    
        Entity::root().set_element(&mut state, "window");

        let mut bounding_box = BoundingBox::default();
        bounding_box.w = window_description.inner_size.width as f32;
        bounding_box.h = window_description.inner_size.height as f32;

        state.data.set_clip_region(Entity::root(), bounding_box);

        //WindowWidget::new().build_window(&mut state);

        Application {
            //window: window,
            event_loop: event_loop,
            event_manager: event_manager,
            state: state,
            windows,
            root_window: window_id,
        }
    }

    pub fn run(self) {

        let mut state = self.state;

        let mut event_manager = self.event_manager;
        event_manager.tree = state.tree.clone();
    

        //println!("Event Manager: {:?}", event_manager.tree);

        //let mut window = self.window;
        let mut should_quit = false;

        //let tree = state.tree.clone();

        Entity::root().restyle(&mut state);
        Entity::root().relayout(&mut state);

        let event_loop_proxy = self.event_loop.create_proxy();

        state.needs_redraw = true;

        let mut click_time = std::time::Instant::now();
        let double_click_interval = std::time::Duration::from_millis(500);
        let mut double_click = false;
        let mut click_pos = (0.0, 0.0);

        let mut windows = self.windows;

        let event_loop = self.event_loop;
        let root_window = self.root_window;

        event_loop.run(move |event, event_loop_window_target, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                GEvent::LoopDestroyed => return,

                GEvent::UserEvent(_) => {
                    //window.handle.window().request_redraw();
                    // for (_, window_entity) in windows.iter() {
                    //     if let Some(mut event_handler) = state.event_handlers.remove(window_entity) {
                    //         if let Some(window) = event_handler.downcast::<Window>() {
                    //             window.handle.as_ref().unwrap().window().request_redraw();
                    //         }

                    //         state.event_handlers.insert(*window_entity, event_handler);
                    //     }                            
                    // }
                }

                GEvent::MainEventsCleared => {
                    //println!("MAIN EVENTS CLEARED");
                    //let start = std::time::Instant::now();

                    
                    while !state.event_queue.is_empty() {
                        // Create any new windows
                        'events: for event in state.event_queue.iter_mut() {
                            // Respond to AppEvent::CreateWindow here
                            if let Some(app_event) = event.message.downcast() {
                                match app_event {
                                    AppEvent::CreateWindow(entity) => {
                                        //println!("Create Window: {}", entity);

                                        //Make all the other windows not current
                                        for (_, other) in windows.iter() {
                                            if *other != *entity {
                                                if let Some(mut window_component) = state.event_handlers.remove(other) {
                                                    if let Some(window_widget) = window_component.downcast::<Window>() {
                                                        if let Some(current_context_wrapper) = window_widget.handle.take() {
                                                            let new_windowed_context = match current_context_wrapper {
                                                                CurrentContextWrapper::PossiblyCurrent(windowed_context) => {
                                                                    CurrentContextWrapper::NotCurrent(unsafe { windowed_context.make_not_current().unwrap()})
                                                                }
                
                                                                t => t,
                                                            };
                                                            window_widget.handle = Some(new_windowed_context);
                                                        }
                                                    }
                                                  
                                                    state.event_handlers.insert(*other, window_component);
                                                }
                                            }
                                        } 
                
                
                                        if let Some(mut window_component) = state.event_handlers.remove(&entity) {
                                            //println!("Found it");
                                            if let Some(window_widget) = window_component.downcast::<Window>() {
                                                let window_id = window_widget.create(event_loop_window_target);
                                                windows.insert(window_id, (*entity).set_window());
                                            }
                                            
                                            //println!("Put it back");
                                            state.event_handlers.insert(*entity, window_component);
                                        }

                                        //println!("{:?}", windows);

                                        
                
                                        continue 'events;
                                    }

                                    _=> {}
                                }
                            }
                        }
                        event_manager.flush_events(&mut state);
                    }
                    
                    //println!("{:.2?} seconds for whatever you did.", start.elapsed());

                    if state.apply_animations() {

                        *control_flow = ControlFlow::Poll;

                        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));

                        event_loop_proxy.send_event(()).unwrap();
                        //window.handle.window().request_redraw();
                        // for (_, window_entity) in windows.iter() {
                        //     if let Some(mut event_handler) = state.event_handlers.remove(window_entity) {
                        //         if let Some(window) = event_handler.downcast::<Window>() {
                        //             window.handle.as_ref().unwrap().window().request_redraw();
                        //         }

                        //         state.event_handlers.insert(*window_entity, event_handler);
                        //     }                            
                        // }
                    } else {
                        *control_flow = ControlFlow::Wait;
                    }

                    let tree = state.tree.clone();

                    if state.needs_redraw {
                        // TODO - Move this to EventManager
                        apply_clipping(&mut state, &tree);
                        //window.handle.window().request_redraw();
                        for (_, window_entity) in windows.iter() {
                            println!("WE: {:?}", window_entity);
                            set_not_current(&windows, &mut state);
                            if let Some(mut event_handler) = state.event_handlers.remove(window_entity) {
                                if let Some(window) = event_handler.downcast::<Window>() {
                                    if let Some(current_context_wrapper) = window.handle.take() {
                                        match current_context_wrapper {
                                            CurrentContextWrapper::NotCurrent(windowed_context) => {
                                                let new_context = unsafe { windowed_context.make_current().unwrap()};
        
                                                // Draw the sub-tree which belongs to the window
                                                //event_manager.draw(&mut state, *window_entity, window.canvas.as_mut().unwrap());
        
                                                //new_context.swap_buffers().unwrap();
                                                new_context.window().request_redraw();
                                                window.handle = Some(CurrentContextWrapper::PossiblyCurrent(new_context));
                                            }
        
                                            _=> {}
                                        }
                                    }

                                    //window.handle.as_ref().unwrap().window().request_redraw();
                                }

                                state.event_handlers.insert(*window_entity, event_handler);
                            }                            
                        }

                        // for (id, _) in state.event_handlers.iter() {
                        //     println!("{:?}", id);
                        // }

                        state.needs_redraw = false;
                    }

                    // println!("Event Handlers");
                    // for (id, _) in state.event_handlers.iter() {
                    //     println!("{:?}", id);
                    // }
                    //println!("Get Here");
                }

                // REDRAW

                GEvent::RedrawRequested(window_id) => {
                    println!("REDRAW REQUESTED: {:?}", window_id);
                    if let Some(window_entity) = windows.get(&window_id) {
                        //println!("window entity: {:?}", window_entity);
                        set_not_current(&windows, &mut state);
                        if let Some(mut window_event_handler) = state.event_handlers.remove(&window_entity) {
                            if let Some(window) = window_event_handler.downcast::<Window>() {
                                if let Some(current_context_wrapper) = window.handle.take() {
                                    match current_context_wrapper {
                                        CurrentContextWrapper::NotCurrent(windowed_context) => {
                                            let new_context = unsafe { windowed_context.make_current().unwrap()};
    
                                            // Draw the sub-tree which belongs to the window
                                            event_manager.draw(&mut state, *window_entity, window.canvas.as_mut().unwrap());
    
                                            new_context.swap_buffers().unwrap();
    
                                            window.handle = Some(CurrentContextWrapper::PossiblyCurrent(new_context));
                                        }
    
                                        _=> {}
                                    }
                                }
                            }

                            state.event_handlers.insert(*window_entity, window_event_handler);
                        }
                    }
                    // //let start = std::time::Instant::now();
                    // event_manager.draw(&mut state, &mut window.canvas);
                    // //println!("{:.2?} seconds for whatever you did.", start.elapsed());
                    // // Swap buffers
                    // window
                    //     .handle
                    //     .swap_buffers()
                    //     .expect("Failed to swap buffers");
                    //println!("Then here");
                }

                GEvent::WindowEvent {
                    event,
                    window_id,
                } => {

                    match event {
                        //////////////////
                        // Close Window //
                        //////////////////
                        glutin::event::WindowEvent::CloseRequested => {

                            if let Some(entity) = windows.get(&window_id) {
                                state.remove(*entity);
                                // Remove from tree and state
                                //state.components.remove(entity);
                            }

                            windows.remove(&window_id);

                            if windows.is_empty() || root_window == window_id {
                                *control_flow = ControlFlow::Exit;
                            }
                        }

                        //TODO
                        ///////////////////////
                        // Modifiers Changed //
                        ///////////////////////
                        glutin::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                            state.modifiers.shift = modifiers_state.shift();
                            state.modifiers.ctrl = modifiers_state.ctrl();
                            state.modifiers.alt = modifiers_state.alt();
                            state.modifiers.logo = modifiers_state.logo();
                        }

                        ////////////////////
                        // Focused Window //
                        ////////////////////
                        glutin::event::WindowEvent::Focused(_) => {
                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle)
                            //         .target(Entity::root())
                            //         .origin(Entity::root()),
                            // );
                            // state.insert_event(
                            //     Event::new(WindowEvent::Relayout)
                            //         .target(Entity::root())
                            //         .origin(Entity::root()),
                            // );

                            Entity::root().restyle(&mut state);
                            Entity::root().relayout(&mut state);
                            Entity::root().redraw(&mut state);
                        }

                        ////////////////////
                        // Focused Window //
                        ////////////////////
                        glutin::event::WindowEvent::ReceivedCharacter(input) => {
                            state.insert_event(
                                Event::new(WindowEvent::CharInput(input))
                                    .target(state.focused)
                                    .propagate(Propagation::Down),
                            );
                        }

                        glutin::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            let s = match input.state {
                                glutin::event::ElementState::Pressed => MouseButtonState::Pressed,
                                glutin::event::ElementState::Released => MouseButtonState::Released,
                            };

	                        // Prefer virtual keycodes to scancodes, as scancodes aren't uniform between platforms
	                        let code = if let Some(vkey) = input.virtual_keycode {
		                        vcode_to_code(vkey)
	                        } else {
		                        scan_to_code(input.scancode)
	                        };

                            let key = vk_to_key(
                                input.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert),
                            );

                            if let Some(virtual_keycode) = input.virtual_keycode {
                                if virtual_keycode == VirtualKeyCode::F5
                                    && s == MouseButtonState::Pressed
                                {
                                    state.reload_styles().unwrap();
                                }

                                if virtual_keycode == VirtualKeyCode::H && s == MouseButtonState::Pressed {
                                    //println!("Focused Widget: {}", state.focused);
                                    
                                    println!("Tree");
                                    for entity in state.tree.clone().into_iter() {
                                        println!("Entity: {} Parent: {:?} posx: {} posy: {} width: {} height: {}", entity, entity.get_parent(&mut state), state.data.get_posx(entity), state.data.get_posy(entity), state.data.get_width(entity), state.data.get_height(entity));
                                    }
                                    for eh in state.event_handlers.iter() {
                                        println!("EH: {:?}", eh.0);
                                    }
                                }

                                if virtual_keycode == VirtualKeyCode::Tab
                                    && s == MouseButtonState::Pressed
                                {
                                    let next_focus = state
                                        .style
                                        .focus_order
                                        .get(state.focused)
                                        .cloned()
                                        .unwrap_or_default()
                                        .next;
                                    let prev_focus = state
                                        .style
                                        .focus_order
                                        .get(state.focused)
                                        .cloned()
                                        .unwrap_or_default()
                                        .prev;

                                    if state.modifiers.shift {
                                        if prev_focus != Entity::null() {
                                            // state.focused.set_focus(&mut state, false);
                                            // state.focused = prev_focus;
                                            // state.focused.set_focus(&mut state, true);
                                            state.set_focus(prev_focus);
                                        } else {
                                            // TODO impliment reverse iterator for tree
                                            // state.focused = match state.focused.into_iter(&state.tree).next() {
                                            //     Some(val) => val,
                                            //     None => Entity::root(),
                                            // };
                                        }
                                    } else {
                                        let tree = state.tree.clone();


                                        //let next = iter.next();

                                        println!("Focused: {}", state.focused);




                                        if next_focus != Entity::null() {
                                            // state.focused.set_focus(&mut state, false);
                                            // state.focused = next_focus;
                                            // state.focused.set_focus(&mut state, true);
                                            state.set_focus(next_focus);
                                        } else {

                                            //state.focused.set_focus(&mut state, false);

                                            let mut iter =  state.focused.tree_iter(&tree);
                                            iter.next();


                                            if let Some(mut temp) = iter.next() {
                                                while !state.data.get_focusable(temp)
                                                    || state.data.get_visibility(temp) == Visibility::Invisible
                                                    || state.data.get_opacity(temp) == 0.0
                                                    || state.style.display.get(temp) == Some(&Display::None)
                                                {
                                                    temp = match iter.next() {
                                                        Some(e) => e,
                                                        None => {
                                                            Entity::root()
                                                        }
                                                    };

                                                    if temp == Entity::root() {
                                                        break;
                                                    }
                                                }

                                                state.set_focus(temp);
                                            } else {
                                                state.set_focus(Entity::root());
                                            }

                                            //state.focused.set_focus(&mut state, true);
                                        }
                                    }

                                    Entity::root().restyle(&mut state);

                                }
                            }

                            match s {
                                MouseButtonState::Pressed => {
                                    if state.focused != Entity::null() {
                                        state.insert_event(
                                            Event::new(WindowEvent::KeyDown(code, key))
                                                .target(state.focused)
                                                .propagate(Propagation::DownUp),
                                        );
                                    } else {
                                        state.insert_event(
                                            Event::new(WindowEvent::KeyDown(code, key))
                                                .target(state.hovered)
                                                .propagate(Propagation::DownUp),
                                        );
                                    }
                                }

                                MouseButtonState::Released => {
                                    if state.focused != Entity::null() {
                                        state.insert_event(
                                            Event::new(WindowEvent::KeyUp(code, key))
                                                .target(state.focused)
                                                .propagate(Propagation::DownUp),
                                        );
                                    } else {
                                        state.insert_event(
                                            Event::new(WindowEvent::KeyUp(code, key))
                                                .target(state.hovered)
                                                .propagate(Propagation::DownUp),
                                        );
                                    }
                                }
                            }
                        }


                        // Window Resize Event
                        glutin::event::WindowEvent::Resized(physical_size) => {
                            //window.handle.resize(physical_size);

                            state
                                .style
                                .width
                                .insert(Entity::root(), Units::Pixels(physical_size.width as f32));
                            state
                                .style
                                .height
                                .insert(Entity::root(), Units::Pixels(physical_size.height as f32));

                            state
                                .data
                                .set_width(Entity::root(), physical_size.width as f32);
                            state
                                .data
                                .set_height(Entity::root(), physical_size.height as f32);

                            let mut bounding_box = BoundingBox::default();
                            bounding_box.w = physical_size.width as f32;
                            bounding_box.h = physical_size.height as f32;

                            state.data.set_clip_region(Entity::root(), bounding_box);

                            // state.insert_event(Event::new(WindowEvent::Restyle).origin(Entity::root()).target(Entity::root()));
                            // state.insert_event(
                            //     Event::new(WindowEvent::Relayout).target(Entity::root()),
                            // );

                            Entity::root().restyle(&mut state);
                            Entity::root().relayout(&mut state);
                            Entity::root().redraw(&mut state);

                        }

                        // Cursor Moved Event 
                        glutin::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            modifiers: _,
                        } => {
                            let cursorx = (position.x) as f32;
                            let cursory = (position.y) as f32;

                            state.mouse.cursorx = cursorx as f32;
                            state.mouse.cursory = cursory as f32;

                            //println!("Cursor Moved: {:?} {} {}", window_id, cursorx, cursory);

                            if let Some(window_entity) = windows.get(&window_id) {
                                apply_hover(&mut state, *window_entity);
                            }

                            if state.captured != Entity::null() {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                        .target(state.captured)
                                        .propagate(Propagation::Direct),
                                );
                            } else if state.hovered != Entity::root() {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                        .target(state.hovered),
                                );
                            }
                        }

                        // Mouse Input Event
                        glutin::event::WindowEvent::MouseInput {
                            device_id: _,
                            state: s,
                            button,
                            modifiers: _,
                        } => {
                            let s = match s {
                                glutin::event::ElementState::Pressed => MouseButtonState::Pressed,
                                glutin::event::ElementState::Released => MouseButtonState::Released,
                            };

                            let b = match button {
                                glutin::event::MouseButton::Left => MouseButton::Left,
                                glutin::event::MouseButton::Right => MouseButton::Right,
                                glutin::event::MouseButton::Middle => MouseButton::Middle,
                                glutin::event::MouseButton::Other(id) => MouseButton::Other(id),
                            };

                            match b {
                                MouseButton::Left => {
                                    state.mouse.left.state = s;
                                }

                                MouseButton::Right => {
                                    state.mouse.right.state = s;
                                }

                                MouseButton::Middle => {
                                    state.mouse.middle.state = s;
                                }

                                _ => {}
                            }

                            match s {
                                MouseButtonState::Pressed => {
                                    // if state.hovered != Entity::null()
                                    //     && state.active != state.hovered
                                    // {
                                    //     state.active = state.hovered;
                                    //     state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
                                    //     state.needs_restyle = true;
                                    // }

                                    let new_click_time = std::time::Instant::now();
                                    let click_duration = new_click_time - click_time;
                                    let new_click_pos = (state.mouse.cursorx, state.mouse.cursory);

                                    if click_duration <= double_click_interval && new_click_pos == click_pos{
                                        if !double_click {
                                            let _target = if state.captured != Entity::null() {
                                                state.insert_event(
                                                    Event::new(WindowEvent::MouseDoubleClick(b))
                                                        .target(state.captured)
                                                        .propagate(Propagation::Direct),
                                                );
                                                state.captured
                                            } else {
                                                state.insert_event(
                                                    Event::new(WindowEvent::MouseDoubleClick(b))
                                                        .target(state.hovered),
                                                );
                                                state.hovered
                                            };
                                            double_click = true;
                                        }
                                        
                                    } else {
                                        double_click = false;
                                    }
                                    
                                    click_time = new_click_time;
                                    click_pos = new_click_pos;

                                    let _target = if state.captured != Entity::null() {
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseDown(b))
                                                .target(state.captured)
                                                .propagate(Propagation::Direct),
                                        );
                                        state.captured
                                    } else {
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseDown(b))
                                                .target(state.hovered),
                                        );
                                        state.hovered
                                    };

                                    // if let Some(event_handler) = state.event_handlers.get_mut(&target) {
                                    //     if let Some(callback) = event_manager.callbacks.get_mut(&target) {
                                    //         (callback)(event_handler, &mut state, target);
                                    //     }
                                    // }

                                    match b {
                                        MouseButton::Left => {
                                            state.mouse.left.pos_down =
                                                (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.pressed = state.hovered;
                                        }

                                        MouseButton::Middle => {
                                            state.mouse.middle.pos_down =
                                                (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.pressed = state.hovered;
                                        }

                                        MouseButton::Right => {
                                            state.mouse.right.pos_down =
                                                (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.pressed = state.hovered;
                                        }

                                        _ => {}
                                    }
                                }

                                MouseButtonState::Released => {
                                    //state.active = Entity::null();
                                    //state.insert_event(Event::new(WindowEvent::Restyle));
                                    //state.needs_restyle = true;

                                    if state.captured != Entity::null() {
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseUp(b))
                                                .target(state.captured)
                                                .propagate(Propagation::Direct),
                                        );
                                    } else {
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseUp(b))
                                                .target(state.hovered),
                                        );
                                    }

                                    match b {
                                        MouseButton::Left => {
                                            state.mouse.left.pos_up =
                                                (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.released = state.hovered;
                                        }

                                        MouseButton::Middle => {
                                            state.mouse.middle.pos_up =
                                                (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.released = state.hovered;
                                        }

                                        MouseButton::Right => {
                                            state.mouse.right.pos_up =
                                                (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.released = state.hovered;
                                        }

                                        _ => {}
                                    }
                                }
                            }
                        }

                        glutin::event::WindowEvent::MouseWheel {
                            device_id: _,
                            delta,
                            phase: _,
                            modifiers: _,
                        } => {
                            let (x, y) = match delta {
                                glutin::event::MouseScrollDelta::LineDelta(xx, yy) => (xx, yy),
                                _ => (0.0, 0.0),
                            };

                            if state.captured != Entity::null() {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseScroll(x, y))
                                        .target(state.captured)
                                        .propagate(Propagation::Direct),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseScroll(x, y))
                                        .target(state.hovered),
                                );
                            }
                        }

                        _ => {}
                    };
                }

                _ => {}
            }

            if should_quit {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}


fn set_not_current(windows: &HashMap<WindowId, Entity>, state: &mut State) {
    for (_, other) in windows.iter() {
        if let Some(mut window_component) = state.event_handlers.remove(other) {
            if let Some(window_widget) = window_component.downcast::<Window>() {
                if let Some(current_context_wrapper) = window_widget.handle.take() {
                    let new_windowed_context = match current_context_wrapper {
                        CurrentContextWrapper::PossiblyCurrent(windowed_context) => {
                            CurrentContextWrapper::NotCurrent(unsafe { windowed_context.make_not_current().unwrap()})
                        }

                        t => t,
                    };
                    window_widget.handle = Some(new_windowed_context);
                }
            }

            state.event_handlers.insert(*other, window_component);
        }
    } 
}

fn make_current(windows: &HashMap<WindowId, Entity>, state: &mut State, window_id: WindowId) {
    if let Some(window_entity) = windows.get(&window_id) {
        set_not_current(windows, state);
        
        if let Some(mut window_component) = state.event_handlers.remove(window_entity) {
            if let Some(window_widget) = window_component.downcast::<Window>() {
                if let Some(current_context_wrapper) = window_widget.handle.take() {
                    let new_windowed_context = match current_context_wrapper {
    
                        CurrentContextWrapper::NotCurrent(windowed_context) => {
                            let windowed_context = unsafe {windowed_context.make_current().unwrap()};
                            CurrentContextWrapper::PossiblyCurrent(windowed_context)
                        }
    
                        t => t,
                    };
                    window_widget.handle = Some(new_windowed_context);
                }
            }
    
            state.event_handlers.insert(*window_entity, window_component);
        }
    }

}


