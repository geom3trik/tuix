use winit::event_loop::{ControlFlow, EventLoop};

use crate::keyboard::{scan_to_code, vk_to_key};
use crate::window::Window;

use tuix_core::{BoundingBox, Units};
use tuix_core::{Entity, State};

use tuix_core::state::mouse::{MouseButton, MouseButtonState};

use tuix_core::events::{Event, EventManager, Propagation};

use tuix_core::state::hierarchy::IntoHierarchyIterator;

use tuix_core::state::Fonts;

use tuix_core::style::{Display, Visibility};

use tuix_core::state::style::prop::*;

use tuix_core::{WindowDescription, WindowEvent, WindowWidget};

use tuix_core::systems::*;

type WEvent<'a, T> = winit::event::Event<'a, T>;

use winit::event::VirtualKeyCode;

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

        let root = Entity::root();
        //state.hierarchy.add(Entity::root(), None);

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

        WindowWidget::new().build_window(&mut state);

        Application {
            window,
            event_loop,
            event_manager,
            state,
        }
    }

    pub fn run(self) {
        let mut state = self.state;
        let mut event_manager = self.event_manager;
        let mut window = self.window;

        let mut should_quit = false;

        let hierarchy = state.hierarchy.clone();

        //state.insert_event(Event::new(WindowEvent::Restyle));
        //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));

        let mut first_time = true;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                WEvent::LoopDestroyed => return,

                WEvent::UserEvent(_) => {}

                WEvent::MainEventsCleared => {
                    let mut needs_redraw = false;

                    if state.apply_animations() {
                        state.insert_event(
                            Event::new(WindowEvent::Relayout)
                                .target(Entity::null())
                                .origin(Entity::root()),
                        );
                        //state.insert_event(Event::new(WindowEvent::Redraw));
                        needs_redraw = true;
                    }

                    if first_time {
                        apply_styles(&mut state, &hierarchy);
                        first_time = false;
                    }

                    while !state.event_queue.is_empty() {
                        if event_manager.flush_events(&mut state) {
                            needs_redraw = true;
                        }
                    }

                    if needs_redraw {
                        window.window.request_redraw();
                    }

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
                WEvent::RedrawRequested(_) => {
                    window.context.make_current();

                    event_manager.draw(&mut state, &hierarchy, &mut window.canvas);

                    window.context.swap_buffers();
                    window.context.make_not_current();
                }

                WEvent::WindowEvent {
                    event,
                    window_id: _,
                } => {
                    match event {
                        //////////////////
                        // Close Window //
                        //////////////////
                        winit::event::WindowEvent::CloseRequested => {
                            state.insert_event(Event::new(WindowEvent::WindowClose));
                            should_quit = true;
                        }

                        //TODO
                        ///////////////////////
                        // Modifiers Changed //
                        ///////////////////////
                        winit::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                            state.modifiers.shift = modifiers_state.shift();
                            state.modifiers.ctrl = modifiers_state.ctrl();
                            state.modifiers.alt = modifiers_state.alt();
                            state.modifiers.logo = modifiers_state.logo();
                        }

                        ////////////////////
                        // Focused Window //
                        ////////////////////
                        winit::event::WindowEvent::Focused(_) => {
                            state.insert_event(
                                Event::new(WindowEvent::Restyle)
                                    .target(Entity::root())
                                    .origin(Entity::root()),
                            );
                        }

                        ////////////////////
                        // Focused Window //
                        ////////////////////
                        winit::event::WindowEvent::ReceivedCharacter(input) => {
                            state.insert_event(
                                Event::new(WindowEvent::CharInput(input))
                                    .target(state.focused)
                                    .propagate(Propagation::Down),
                            );
                        }

                        winit::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            let s = match input.state {
                                winit::event::ElementState::Pressed => MouseButtonState::Pressed,
                                winit::event::ElementState::Released => MouseButtonState::Released,
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

                        winit::event::WindowEvent::Resized(physical_size) => {
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

                            state.insert_event(
                                Event::new(WindowEvent::Restyle).origin(Entity::root()),
                            );
                            state.insert_event(
                                Event::new(WindowEvent::Relayout).target(Entity::null()),
                            );
                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }

                        winit::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            ..
                        } => {
                            let cursorx = (position.x) as f32;
                            let cursory = (position.y) as f32;

                            state.mouse.cursorx = cursorx as f32;
                            state.mouse.cursory = cursory as f32;

                            apply_hover(&mut state);

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

                        winit::event::WindowEvent::MouseInput {
                            device_id: _,
                            state: s,
                            button,
                            ..
                        } => {
                            let s = match s {
                                winit::event::ElementState::Pressed => MouseButtonState::Pressed,
                                winit::event::ElementState::Released => MouseButtonState::Released,
                            };

                            let b = match button {
                                winit::event::MouseButton::Left => MouseButton::Left,
                                winit::event::MouseButton::Right => MouseButton::Right,
                                winit::event::MouseButton::Middle => MouseButton::Middle,
                                winit::event::MouseButton::Other(id) => MouseButton::Other(id),
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

                        winit::event::WindowEvent::MouseWheel {
                            device_id: _,
                            delta,
                            phase: _,
                            ..
                        } => {
                            let (x, y) = match delta {
                                winit::event::MouseScrollDelta::LineDelta(xx, yy) => (xx, yy),
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
