#![allow(deprecated)]

use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::{dpi::*, window};

use crate::keyboard::{scan_to_code, vk_to_key};

use crate::window::Window;

use tuix_core::{Color, Length, Visibility};
use tuix_core::{Entity, State};

use tuix_core::state::mouse::{MouseButton, MouseButtonState};

use tuix_core::events::{Event, EventManager, Propagation};

use tuix_core::state::hierarchy::IntoHierarchyIterator;

use tuix_core::state::Fonts;

use tuix_core::state::style::prop::*;

use tuix_core::{WindowDescription, WindowEvent, WindowWidget};

use tuix_core::systems::{apply_clipping, apply_styles, apply_visibility, apply_z_ordering, apply_hover};

use glutin::event::VirtualKeyCode;

type GEvent<'a, T> = glutin::event::Event<'a, T>;

pub struct Application {
    pub window: Window,
    pub state: State,
    event_loop: EventLoop<()>,
    pub event_manager: EventManager,
}

impl Application {
    pub fn new<F: FnOnce(WindowDescription, &mut State, Entity) -> WindowDescription>(
        mut app: F,
    ) -> Self {
        let event_loop = EventLoop::new();
        let mut state = State::new();

        let event_manager = EventManager::new();

        let root = Entity::root();
        state.hierarchy.add(Entity::root(), None);

        //let window_description = win(WindowDescription::new());
        let window_description = app(WindowDescription::new(), &mut state, root);

        let mut window = Window::new(&event_loop, &window_description);

        let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../../resources/entypo.ttf");
        let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");

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
            emoji: Some(
                window
                    .canvas
                    .add_font_mem(emoji_font)
                    .expect("Cannot add font"),
            ),
        };

        state.fonts = fonts;

        state.style.width.insert(
            Entity::root(),
            Length::Pixels(window_description.inner_size.width as f32),
        );
        state.style.height.insert(
            Entity::root(),
            Length::Pixels(window_description.inner_size.height as f32),
        );

        state
            .data
            .set_width(Entity::root(), window_description.inner_size.width as f32);
        state
            .data
            .set_height(Entity::root(), window_description.inner_size.height as f32);
        state.data.set_opacity(Entity::root(), 1.0);

        WindowWidget::new().build_window(&mut state);

        Application {
            window: window,
            event_loop: event_loop,
            event_manager: event_manager,
            state: state,
        }
    }

    pub fn run(self) {
        let mut pos: (f32, f32) = (0.0, 0.0);

        let mut state = self.state;
        let mut event_manager = self.event_manager;

        let mut window = self.window;
        let mut should_quit = false;

        //let hierarchy = state.hierarchy.clone();

        let mut counter = 0;

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));

        let event_loop_proxy = self.event_loop.create_proxy();

        let mut first_time = true;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                GEvent::LoopDestroyed => return,

                GEvent::UserEvent(_) => {
                    window.handle.window().request_redraw();
                }

                GEvent::MainEventsCleared => {

                    let mut needs_redraw = false;
                    while !state.event_queue.is_empty() {
                        if event_manager.flush_events(&mut state) {
                            needs_redraw = true;
                        }
                    }

                    if state.apply_animations() {
                        //println!("Animate");
                        *control_flow = ControlFlow::Poll;
                        state.insert_event(
                            Event::new(WindowEvent::Relayout)
                                .target(Entity::root())
                                .origin(Entity::root()),
                        );
                        //state.insert_event(Event::new(WindowEvent::Redraw));
                        event_loop_proxy.send_event(());
                        window.handle.window().request_redraw();
                    } else {
                        //println!("Wait");
                        *control_flow = ControlFlow::Wait;
                    }

                    if first_time {
                        let hierarchy = state.hierarchy.clone();
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
                    let hierarchy = state.hierarchy.clone();
                    event_manager.draw(&mut state, &hierarchy, &mut window.canvas);
                    // Swap buffers
                    window
                        .handle
                        .swap_buffers()
                        .expect("Failed to swap buffers");
                }

                GEvent::WindowEvent {
                    event,
                    window_id: _,
                } => {
                    match event {
                        //////////////////
                        // Close Window //
                        //////////////////
                        glutin::event::WindowEvent::CloseRequested => {
                            state.insert_event(Event::new(WindowEvent::WindowClose));
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
                                Event::new(WindowEvent::Restyle)
                                    .target(Entity::root())
                                    .origin(Entity::root()),
                            );
                            state.insert_event(
                                Event::new(WindowEvent::Relayout)
                                    .target(Entity::root())
                                    .origin(Entity::root()),
                            );

                            state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
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

                            let code = scan_to_code(input.scancode);
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
                                    println!("Hierarchy");
                                    for entity in state.hierarchy.into_iter() {
                                        println!("Entity: {}  Parent: {:?} FC: {:?} NS: {:?}", entity, state.hierarchy.get_parent(entity), state.hierarchy.get_first_child(entity), state.hierarchy.get_next_sibling(entity));

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
                                            state.focused.set_focus(&mut state, false);
                                            state.focused = prev_focus;
                                            state.focused.set_focus(&mut state, true);
                                        } else {
                                            // TODO impliment reverse iterator for hierarchy
                                            // state.focused = match state.focused.into_iter(&state.hierarchy).next() {
                                            //     Some(val) => val,
                                            //     None => Entity::root(),
                                            // };
                                        }
                                    } else {
                                        let hierarchy = state.hierarchy.clone();
                                        if next_focus != Entity::null() {
                                            state.focused.set_focus(&mut state, false);
                                            state.focused = next_focus;
                                            state.focused.set_focus(&mut state, true);
                                        } else {
                                            state.focused.set_focus(&mut state, false);
                                            state.focused =
                                                match state.focused.into_iter(&hierarchy).next() {
                                                    Some(val) => val,
                                                    None => Entity::root(),
                                                };
                                            state.focused.set_focus(&mut state, true);
                                        }
                                    }

                                    state.insert_event(
                                        Event::new(WindowEvent::Restyle)
                                            .target(Entity::root())
                                            .origin(Entity::root()),
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

                            state
                                .style
                                .width
                                .insert(Entity::root(), Length::Pixels(physical_size.width as f32));
                            state
                                .style
                                .height
                                .insert(Entity::root(), Length::Pixels(physical_size.height as f32));

                            state
                                .data
                                .set_width(Entity::root(), physical_size.width as f32);
                            state
                                .data
                                .set_height(Entity::root(), physical_size.height as f32);

                            state.insert_event(Event::new(WindowEvent::Restyle).origin(Entity::root()).target(Entity::root()));
                            state.insert_event(
                                Event::new(WindowEvent::Relayout).target(Entity::root()),
                            );
                            state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
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

                            apply_hover(&mut state);
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
                                        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
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
