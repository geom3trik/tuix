#![allow(deprecated)]

use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::dpi::*;

use crate::{keyboard::{scan_to_code, vk_to_key}, window};

use crate::window::Window;

use tuix_core::{Entity, State};
use tuix_core::{Length, Visibility, Color};

use tuix_core::state::mouse::{MouseButton, MouseButtonState};

use tuix_core::events::{Event, EventManager, Propagation};

use tuix_core::state::hierarchy::IntoHierarchyIterator;

use tuix_core::state::Fonts;

use tuix_core::state::style::prop::*;

use tuix_core::{WindowEvent, WindowDescription, WindowWidget};

use tuix_core::systems::{apply_clipping, apply_z_ordering, apply_styles, apply_visibility};

use glutin::event::VirtualKeyCode;

type GEvent<'a, T> = glutin::event::Event<'a, T>;

pub struct Application {
    pub window: Window,
    pub state: State,
    event_loop: EventLoop<()>,
    pub event_manager: EventManager,
}

impl Application {
    pub fn new<F: FnMut(WindowDescription, &mut State, Entity) -> WindowDescription>(
        mut app: F,
    ) -> Self {
        let event_loop = EventLoop::new();
        let mut state = State::new();

        let event_manager = EventManager::new();

        let root = state.root;
        state.hierarchy.add(state.root, None);

        //let window_description = win(WindowDescription::new());
        let window_description = app(WindowDescription::new(), &mut state, root);

        let mut window = Window::new(&event_loop, &window_description);

        let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../../resources/entypo.ttf");

        let fonts = Fonts {
            regular: Some(
                window
                    .canvas
                    .add_font_mem(regular_font)
                    .expect("Cannot add font"),
            ),
            bold: Some(
                window
                    .canvas
                    .add_font_mem(bold_font)
                    .expect("Cannot add font"),
            ),
            icons: Some(
                window
                    .canvas
                    .add_font_mem(icon_font)
                    .expect("Cannot add font"),
            ),
        };

        state.fonts = fonts;

        state.style.width.insert(
            state.root,
            Length::Pixels(window_description.inner_size.width as f32),
        );
        state.style.height.insert(
            state.root,
            Length::Pixels(window_description.inner_size.height as f32),
        );

        state.transform.set_width(
            state.get_root(),
            window_description.inner_size.width as f32,
        );
        state.transform.set_height(
            state.get_root(),
            window_description.inner_size.height as f32,
        );
        state.transform.set_opacity(state.get_root(), 1.0);

        WindowWidget::new().build_window(&mut state);

        Application {
            window: window,
            event_loop: event_loop,
            event_manager: event_manager,
            state: state,
        }
    }

    pub fn get_window(&self) -> Entity {
        self.state.root
    }

    pub fn get_state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn get_event_manager(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }

    pub fn run(self) {
        let mut pos: (f32, f32) = (0.0, 0.0);

        let mut state = self.state;
        let mut event_manager = self.event_manager;

        let mut window = self.window;
        let mut should_quit = false;

        let hierarchy = state.hierarchy.clone();

        //state.insert_event(Event::new(WindowEvent::Restyle));
        //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));

        let event_loop_proxy = self.event_loop.create_proxy();

        let mut first_time = true;

        self.event_loop.run(move |event, _, control_flow|{
            
            *control_flow = ControlFlow::Wait;

            match event {
                GEvent::LoopDestroyed => return,

                GEvent::UserEvent(_) => {
                    
                    window.handle.window().request_redraw();
                    
                    //println!("User Event");
                    // if state.apply_animations() {
                    //     *control_flow = ControlFlow::Poll;
                    //     state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()).origin(Entity::new(0, 0)));
                    //     //state.insert_event(Event::new(WindowEvent::Redraw));
                    //     event_loop_proxy.send_event(());
                    //     window.handle.window().request_redraw();
                    // } else {
                    //     *control_flow = ControlFlow::Wait;
                    // }
                }

                GEvent::MainEventsCleared => {
                    //println!("Main Events Cleared: {}", counter);
                    //counter += 1;

                    let mut needs_redraw = false;
                    while !state.event_queue.is_empty() {
                        if event_manager.flush_events(&mut state) {
                            needs_redraw = true;
                        }
                    }

                    if state.apply_animations() {
                        //println!("Animate");
                        *control_flow = ControlFlow::Poll;
                        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()).origin(Entity::new(0, 0)));
                        //state.insert_event(Event::new(WindowEvent::Redraw));
                        event_loop_proxy.send_event(());
                        window.handle.window().request_redraw();
                    } else {
                        //println!("Wait");
                        *control_flow = ControlFlow::Wait;
                    }


                    

                    if first_time {
                        apply_styles(&mut state, &hierarchy);
                        first_time = false;
                    }

                    

                    if needs_redraw {
                        window.handle.window().request_redraw();
                    }



                    //

                    // event_manager.flush_events(&mut state);

                    
                    // apply_z_ordering(&mut state, &hierarchy);
                    // apply_visibility(&mut state, &hierarchy);
                    // apply_clipping(&mut state, &hierarchy);
                    // layout_fun(&mut state, &hierarchy);

                    // event_manager.draw(&mut state, &hierarchy, &mut window.canvas);
                    // window
                    //     .handle
                    //     .swap_buffers()
                    //     .expect("Failed to swap buffers");
                }

                // REDRAW
                GEvent::RedrawRequested(_) => {
                    event_manager.draw(&mut state, &hierarchy, &mut window.canvas);
                    // Swap buffers
                    window
                    .handle
                    .swap_buffers()
                    .expect("Failed to swap buffers");
                }

                GEvent::WindowEvent { event, window_id: _ } => {

                    match event {
                        //////////////////
                        // Close Window //
                        //////////////////
                        glutin::event::WindowEvent::CloseRequested => {
                            state
                                .insert_event(Event::new(WindowEvent::WindowClose));
                                should_quit = true;
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
                            state.insert_event(
                                Event::new(WindowEvent::Restyle).target(state.root).origin(state.root),
                            );
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

                        glutin::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                    
                            let s = match input.state {
                                glutin::event::ElementState::Pressed => MouseButtonState::Pressed,
                                glutin::event::ElementState::Released => MouseButtonState::Released,
                            };

                            let code = scan_to_code(input.scancode);
                            let key = vk_to_key(input.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert));

                            if let Some(virtual_keycode) = input.virtual_keycode {


                                if virtual_keycode == VirtualKeyCode::F5 && s == MouseButtonState::Pressed {
                                    state.reload_styles().unwrap();
                                }

                                if virtual_keycode == VirtualKeyCode::Tab && s == MouseButtonState::Pressed {

                                    let next_focus = state.style.focus_order.get(state.focused).cloned().unwrap_or_default().next;
                                    let prev_focus = state.style.focus_order.get(state.focused).cloned().unwrap_or_default().prev;

                                    if state.modifiers.shift {
                                        if prev_focus != Entity::null() {
                                            state.focused.set_focus(&mut state, false);
                                            state.focused = prev_focus;
                                            state.focused.set_focus(&mut state, true);
                                        } else {
                                            // TODO impliment reverse iterator for hierarchy
                                            // state.focused = match state.focused.into_iter(&state.hierarchy).next() {
                                            //     Some(val) => val,
                                            //     None => state.root,
                                            // };
                                        }
                                    } else {
                                        if next_focus != Entity::null() {
                                            state.focused.set_focus(&mut state, false);
                                            state.focused = next_focus;
                                            state.focused.set_focus(&mut state, true);
                                        } else {
                                            state.focused.set_focus(&mut state, false);
                                            state.focused = match state.focused.into_iter(&hierarchy).next() {
                                                Some(val) => val,
                                                None => state.root,
                                            };
                                            state.focused.set_focus(&mut state, true);
                                        }
                                    }

                                    state.insert_event(
                                        Event::new(WindowEvent::Restyle).target(state.root).origin(state.root),
                                    );

                                    
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
    
                        glutin::event::WindowEvent::Resized(physical_size) => {

                            window.handle.resize(physical_size);

                            state.style.width.insert(state.root, Length::Pixels(physical_size.width as f32));
                            state.style.height.insert(state.root, Length::Pixels(physical_size.height as f32));
    
                            state
                                .transform
                                .set_width(state.root, physical_size.width as f32);
                            state
                                .transform
                                .set_height(state.root, physical_size.height as f32);

    
                            state.insert_event(Event::new(WindowEvent::Restyle).origin(state.root));
                            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                            state.insert_event(Event::new(WindowEvent::Redraw));
    
                        }
    
                        glutin::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            modifiers: _,
                        } => {
                            let cursorx = (position.x) as f32;
                            let cursory = (position.y) as f32;
    
                            state.mouse.cursorx = cursorx as f32;
                            state.mouse.cursory = cursory as f32;
    
                            let mut hovered_widget = Entity::new(0, 0);
    
                            // This only really needs to be computed when the hierarchy changes
                            // Can be optimised
                            let mut draw_hierarchy: Vec<Entity> = state.hierarchy.into_iter().collect();
    
                            draw_hierarchy.sort_by_cached_key(|entity| state.transform.get_z_order(*entity));
    
    
                            for widget in draw_hierarchy.into_iter() {
                                // Skip invisible widgets
                                if state.transform.get_visibility(widget) == Visibility::Invisible
                                {
                                    continue;
                                }
                                
                                // This shouldn't be here but there's a bug if it isn't 
                                if state.transform.get_opacity(widget) == 0.0 {
                                    continue;
                                }
                                
                                // Skip non-hoverable widgets
                                if state.transform.get_hoverability(widget) != true {
                                    continue;
                                }

                                
    
                                let border_width = match state
                                    .style
                                    .border_width
                                    .get(widget)
                                    .cloned()
                                    .unwrap_or_default() 
                                {
                                    Length::Pixels(val) => val,
                                    //Length::Percentage(val) => parent_width * val,
                                    _ => 0.0,
                                };
    
                                let posx = state.transform.get_posx(widget) - (border_width / 2.0);
                                let posy = state.transform.get_posy(widget) - (border_width / 2.0);
                                let width = state.transform.get_width(widget) + (border_width);
                                let height = state.transform.get_height(widget) + (border_width);

                                let clip_widget = state.transform.get_clip_widget(widget);


                                let clip_posx = state.transform.get_posx(clip_widget);
                                let clip_posy = state.transform.get_posy(clip_widget);
                                let clip_width = state.transform.get_width(clip_widget);
                                let clip_height = state.transform.get_height(clip_widget);
    
                                if cursorx >= posx && cursorx >= clip_posx
                                    && cursorx < (posx + width) && cursorx < (clip_posx + clip_width)
                                    && cursory >= posy && cursory >= clip_posy
                                    && cursory < (posy + height) && cursory < (clip_posy + clip_height)
                                {
                                    hovered_widget = widget;
                                    if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(hovered_widget) {
                                        pseudo_classes.set_over(true);
                                    }
                                } else {
                                    if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(hovered_widget) {
                                        pseudo_classes.set_over(false);
                                    }
                                }
                            }
    
                            if hovered_widget != state.hovered {

                                // Useful for debugging
                            
                                // println!(
                                //     "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {}",
                                //     hovered_widget,
                                //     state.hierarchy.get_parent(hovered_widget),
                                //     state.transform.get_posx(hovered_widget),
                                //     state.transform.get_posy(hovered_widget),
                                //     state.transform.get_width(hovered_widget),
                                //     state.transform.get_height(hovered_widget),
                                //     state.transform.get_z_order(hovered_widget),
                                // );

                                if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(hovered_widget) {
                                    pseudo_classes.set_hover(true);
                                }

                                if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(state.hovered) {
                                    pseudo_classes.set_hover(false);
                                }
    
                                state.insert_event(Event::new(WindowEvent::MouseOver).target(hovered_widget));
                                state.insert_event(Event::new(WindowEvent::MouseOut).target(state.hovered));
    
                                state
                                    .insert_event(Event::new(WindowEvent::Restyle).origin(hovered_widget));
                                state
                                    .insert_event(Event::new(WindowEvent::Restyle).origin(state.hovered));
                                
                                state.hovered = hovered_widget;
                                state.active = Entity::null();
    
                                
                                state
                                    .insert_event(Event::new(WindowEvent::Redraw));
                            }
    
                            if state.captured != Entity::null() {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                        .target(state.captured)
                                        .propagate(Propagation::Direct),
                                );
                            } else if state.hovered != Entity::new(0, 0) {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                        .target(state.hovered),
                                );
                            }
    
                            pos = (cursorx, cursory);

                        }

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

                                    if state.hovered != Entity::null()
                                        && state.active != state.hovered
                                    {
                                        state.active = state.hovered;
                                        state.insert_event(Event::new(WindowEvent::Restyle));
                                    }
    
                                    if state.captured != Entity::null() {
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseDown(b))
                                                .target(state.captured)
                                                .propagate(Propagation::Direct),
                                        );
                                    } else {
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseDown(b))
                                                .target(state.hovered),
                                        );
                                    }

                                    match b {
                                        MouseButton::Left => {
                                            state.mouse.left.pos_down = (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.pressed = state.hovered;
                                        }

                                        MouseButton::Middle => {
                                            state.mouse.middle.pos_down = (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.pressed = state.hovered;
                                        }
                                        
                                        MouseButton::Right => {
                                            state.mouse.right.pos_down = (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.pressed = state.hovered;
                                        } 
                                        
                                        _ => {}
                                    }
                                }
    
                                MouseButtonState::Released => {

                            
                                    state.active = Entity::null();
                                    state.insert_event(Event::new(WindowEvent::Restyle));
    
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
                                            state.mouse.left.pos_up = (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.released = state.hovered;
                                        }
                                        
                                        MouseButton::Middle => {
                                            state.mouse.middle.pos_up = (state.mouse.cursorx, state.mouse.cursory);
                                            state.mouse.left.released = state.hovered;
                                        } 
                                          
                                        MouseButton::Right => {
                                            state.mouse.right.pos_up = (state.mouse.cursorx, state.mouse.cursory);
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
                                    Event::new(WindowEvent::MouseScroll(x, y)).target(state.captured).propagate(Propagation::Direct)
                                );
                            } else {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseScroll(x, y)).target(state.hovered)
                                );
                            }
                        }
    
                        _ => {}
                    };
                }

                _=> {}
            }

            if should_quit {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
