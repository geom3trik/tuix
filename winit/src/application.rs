use std::collections::HashMap;

use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};

use femtovg::{renderer::OpenGl, Canvas, Color};
use raw_gl_context::{GlConfig, GlContext};

use crate::keyboard::{scan_to_code, vk_to_key};
use crate::window::{Window, WindowWidget};

use tuix_core::{events::{Event, EventManager, Propagation}, window};
use tuix_core::state::hierarchy::IntoHierarchyIterator;
use tuix_core::state::mouse::{MouseButton, MouseButtonState};
use tuix_core::state::Fonts;
use tuix_core::{Entity, State};
use tuix_core::{Length, Visibility};

use tuix_core::state::style::prop::*;
use tuix_core::systems::{apply_clipping, apply_styles, apply_visibility, apply_z_ordering};
use tuix_core::{WindowDescription, WindowEvent, AppEvent};

type WEvent<'a, T> = winit::event::Event<'a, T>;
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;
use winit::window::Icon;

use winit::event::VirtualKeyCode;

pub struct Application {
    //pub window: Window,
    pub state: State,
    pub window_description: WindowDescription,
    // event_loop: EventLoop<()>,
    // pub event_manager: EventManager,
    // windows: Vec<Entity>,
}

impl Application {
    pub fn new<F: FnMut(WindowDescription, &mut State, Entity) -> WindowDescription>(
        mut app: F,
    ) -> Self {
        // let event_loop = EventLoop::new();
        let mut state = State::new();

        // let event_manager = EventManager::new();

        // let root = Entity::root();
        state.hierarchy.add(Entity::root(), None);

        // //let window_description = win(WindowDescription::new());
        let window_description = app(WindowDescription::new(), &mut state, Entity::root());

        // let mut window = WindowWidget2::new(&event_loop, &window_description);

        // let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
        // let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
        // let icon_font = include_bytes!("../../resources/entypo.ttf");
        // let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");

        // let fonts = Fonts {
        //     regular: Some(
        //         window
        //             .canvas.unwrap()
        //             .add_font_mem(regular_font)
        //             .expect("Cannot add font"),
        //     ),
        //     bold: Some(
        //         window
        //             .canvas.unwrap()
        //             .add_font_mem(bold_font)
        //             .expect("Cannot add font"),
        //     ),
        //     icons: Some(
        //         window
        //             .canvas.unwrap()
        //             .add_font_mem(icon_font)
        //             .expect("Cannot add font"),
        //     ),
        //     emoji: Some(
        //         window
        //             .canvas.unwrap()
        //             .add_font_mem(emoji_font)
        //             .expect("Cannot add font"),
        //     ),
        // };

        // state.fonts = fonts;

        // state.style.width.insert(
        //     Entity::root(),
        //     Length::Pixels(window_description.inner_size.width as f32),
        // );
        // state.style.height.insert(
        //     Entity::root(),
        //     Length::Pixels(window_description.inner_size.height as f32),
        // );

        // state
        //     .data
        //     .set_width(Entity::root(), window_description.inner_size.width as f32);
        // state
        //     .data
        //     .set_height(Entity::root(), window_description.inner_size.height as f32);
        // state.data.set_opacity(Entity::root(), 1.0);

        // window.build_window(&mut state);


        Application {
            //window,
            // event_loop,
            // event_manager,
            state,
            window_description,
            // windows: Vec::new(),
        }
    }

    pub fn run(mut self) {
        let mut pos: (f32, f32) = (0.0, 0.0);

        let mut state = self.state;
        //state.hierarchy.add(Entity::root(), None);
        let mut event_manager = EventManager::new();
        //let mut window = self.window;

        let mut should_quit = false;

        //let hierarchy = state.hierarchy.clone();

        let event_loop = EventLoop::new();

        //let mut windows = self.windows;

        let mut num_of_windows = 1;

        let mut contexts: HashMap<Entity, (GlContext, Canvas<OpenGl>)> = HashMap::new();

        let window_description = self.window_description;

        let window_builder = WindowBuilder::new()
            .with_title(&window_description.title)
            .with_inner_size(PhysicalSize::new(
                window_description.inner_size.width,
                window_description.inner_size.height,
            ))
            .with_min_inner_size(PhysicalSize::new(
                window_description.min_inner_size.width,
                window_description.min_inner_size.height,
            ))
            .with_window_icon(if let Some(icon) = &window_description.icon {
                Some(
                    Icon::from_rgba(
                        icon.clone(),
                        window_description.icon_width,
                        window_description.icon_height,
                    )
                    .unwrap(),
                )
            } else {
                None
            });

        let handle = window_builder
            .build(&event_loop)
            .expect("Window creation failed");
        
        let mut gl_config = GlConfig::default();
        gl_config.vsync = true;

        let context =
        GlContext::create(&handle, gl_config).expect("OpenGL context creation failed");

        context.make_current();

        let renderer = OpenGl::new(|s| context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let dpi_factor = handle.scale_factor();
        let size = handle.inner_size();

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            Color::rgb(80, 80, 255),
        );

        context.make_not_current();

        let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../../resources/entypo.ttf");
        let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");

        let fonts = Fonts {
            regular: Some(
                canvas
                    .add_font_mem(regular_font)
                    .expect("Cannot add font"),
            ),
            bold: Some(
                canvas
                    .add_font_mem(bold_font)
                    .expect("Cannot add font"),
            ),
            icons: Some(
                canvas
                    .add_font_mem(icon_font)
                    .expect("Cannot add font"),
            ),
            emoji: Some(
                canvas
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

        let main_window = handle.id();

        let mut windows = HashMap::new();
        windows.insert(handle.id(), Entity::root());

        let mut window_widget = WindowWidget::default();
        window_widget.handle = Some(handle);
        window_widget.build_window(&mut state);
        contexts.insert(Entity::root(), (context, canvas));

        //let event_loop = EventLoop::new();


        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        

        let event_loop_proxy = event_loop.create_proxy();

        let mut first_time = true;

        event_loop.run(move |event, event_loop, control_flow| {
            match event {
                WEvent::LoopDestroyed => return,

                WEvent::UserEvent(_) => {
                    //window.handle.request_redraw();
                    state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    // for (window, (context, canvas)) in contexts.iter_mut() {
                    //     context.make_current();
                    //     let hierarchy = state.hierarchy.clone();
                    //     event_manager.draw(&mut state, &hierarchy, *window, canvas);
                    //     context.swap_buffers();
                    //     context.make_not_current();
                    // }
                }

                WEvent::MainEventsCleared => {
                    let mut needs_redraw = false;

                    while !state.event_queue.is_empty() {
                        //println!("Flush Events");
                        if event_manager.flush_events(&mut state, |event_handlers, app_event| {
                            match app_event {
                                AppEvent::AddWindow(entity) => {
                                    entity.testy2(event_handlers, |window_widget: &mut WindowWidget| {
                                        contexts.insert(*entity, window_widget.create_window(event_loop));
                                        windows.insert(window_widget.id(), *entity);
                                        //windows.push(*entity);
                                        num_of_windows += 1;
                                    });
                                }
                                _=> {}
                            }
                        }) {
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
                        //window.handle.request_redraw();
                    } else {
                        //println!("Wait");
                        *control_flow = ControlFlow::Wait;
                    }

                    if needs_redraw {
                        //window.handle.request_redraw();
                    }
                }

                // REDRAW
                WEvent::RedrawRequested(_) => {
                    let hierarchy = state.hierarchy.clone();

                    
                    for (window, (context, canvas)) in contexts.iter_mut() {
                        context.make_current();
                        event_manager.draw2(&mut state, &hierarchy, *window, canvas);
                        context.swap_buffers();
                        context.make_not_current();
                    }
                }

                WEvent::WindowEvent {
                    event,
                    window_id,
                } => {
                    match event {
                        //////////////////
                        // Close Window //
                        //////////////////
                        winit::event::WindowEvent::CloseRequested => {
                            println!("Close Window: {:?}", window_id);
                            if window_id == main_window {
                                should_quit = true;
                            }
                            state.insert_event(Event::new(WindowEvent::WindowClose));
                            if let Some(window_entity) = windows.get(&window_id) {
                                state.remove(*window_entity);
                                if let Some(window_entity) = windows.remove(&window_id) {
                                    contexts.remove(&window_entity);
                                }
                                
                                if windows.len() == 0 {
                                    should_quit = true;
                                }                                
                            }
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
                                        if next_focus != Entity::null() {
                                            state.focused.set_focus(&mut state, false);
                                            state.focused = next_focus;
                                            state.focused.set_focus(&mut state, true);
                                        } else {
                                            let hierarchy = state.hierarchy.clone();
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
                            let window_entity = windows.get(&window_id).unwrap();
                            state
                                .style
                                .width
                                .insert(*window_entity, Length::Pixels(physical_size.width as f32));
                            state.style.height.insert(
                                *window_entity,
                                Length::Pixels(physical_size.height as f32),
                            );

                            state
                                .data
                                .set_width(*window_entity, physical_size.width as f32);
                            state
                                .data
                                .set_height(*window_entity, physical_size.height as f32);

                            state.insert_event(
                                Event::new(WindowEvent::Restyle).target(*window_entity),
                            );
                            state.insert_event(
                                Event::new(WindowEvent::Relayout).target(*window_entity),
                            );
                            state.insert_event(Event::new(WindowEvent::Redraw).target(*window_entity));
                        }

                        winit::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            ..
                        } => {

                            let window = windows.get(&window_id).unwrap();

                            let cursorx = (position.x) as f32;
                            let cursory = (position.y) as f32;

                            //println!("cursorx: {} cursory: {} window: {:?}", cursorx, cursory, window_id);

                            state.mouse.cursorx = cursorx as f32;
                            state.mouse.cursory = cursory as f32;

                            let mut hovered_widget = Entity::root();

                            let mut draw_hierarchy = Vec::new();
                            let mut temp = Some(*window);
                            let hierarchy = state.hierarchy.clone();
                            let mut iterator = window.into_iter(&hierarchy);
                            while temp.is_some() {
                                
                                temp = iterator.next();
                                if let Some(entity) = temp {

                                    let parent_window = state.data.get_window(entity);
                                    if parent_window != *window {
                                        temp = iterator.next_branch();
                                    } else {
                                        draw_hierarchy.push(entity);
                                    }
                                    
                                }
                            }

                            // let mut draw_hierarchy: Vec<Entity> =
                            //     state.hierarchy.into_iter().collect();

                            draw_hierarchy
                                .sort_by_cached_key(|entity| state.data.get_z_order(*entity));

                            for widget in draw_hierarchy.into_iter() {
                                //println!("entity: {}", widget);
                                // Skip invisible widgets
                                if state.data.get_visibility(widget) == Visibility::Invisible {
                                    continue;
                                }

                                // This shouldn't be here but there's a bug if it isn't
                                if state.data.get_opacity(widget) == 0.0 {
                                    continue;
                                }

                                // Skip non-hoverable widgets
                                if state.data.get_hoverability(widget) != true {
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

                                let posx = state.data.get_posx(widget) - (border_width / 2.0);
                                let posy = state.data.get_posy(widget) - (border_width / 2.0);
                                let width = state.data.get_width(widget) + (border_width);
                                let height = state.data.get_height(widget) + (border_width);

                                let clip_widget = state.data.get_clip_widget(widget);

                                // let clip_posx = state.data.get_posx(clip_widget);
                                // let clip_posy = state.data.get_posy(clip_widget);
                                // let clip_width = state.data.get_width(clip_widget);
                                // let clip_height = state.data.get_height(clip_widget);

                                let clip_posx = 0.0;
                                let clip_posy = 0.0;
                                let clip_width = std::f32::MAX;
                                let clip_height = std::f32::MAX;


                                if cursorx >= posx
                                    && cursorx >= clip_posx
                                    && cursorx < (posx + width)
                                    && cursorx < (clip_posx + clip_width)
                                    && cursory >= posy
                                    && cursory >= clip_posy
                                    && cursory < (posy + height)
                                    && cursory < (clip_posy + clip_height)
                                {
                                    hovered_widget = widget;
                                    if let Some(pseudo_classes) =
                                        state.style.pseudo_classes.get_mut(hovered_widget)
                                    {
                                        pseudo_classes.set_over(true);
                                    }
                                } else {
                                    if let Some(pseudo_classes) =
                                        state.style.pseudo_classes.get_mut(hovered_widget)
                                    {
                                        pseudo_classes.set_over(false);
                                    }
                                }
                            }

                            if hovered_widget != state.hovered {
                                // Useful for debugging

                                println!(
                                    "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {}",
                                    hovered_widget,
                                    state.hierarchy.get_parent(hovered_widget),
                                    state.data.get_posx(hovered_widget),
                                    state.data.get_posy(hovered_widget),
                                    state.data.get_width(hovered_widget),
                                    state.data.get_height(hovered_widget),
                                    state.data.get_z_order(hovered_widget),
                                );

                                if let Some(pseudo_classes) =
                                    state.style.pseudo_classes.get_mut(hovered_widget)
                                {
                                    pseudo_classes.set_hover(true);
                                }

                                if let Some(pseudo_classes) =
                                    state.style.pseudo_classes.get_mut(state.hovered)
                                {
                                    pseudo_classes.set_hover(false);
                                }

                                state.insert_event(
                                    Event::new(WindowEvent::MouseOver).target(hovered_widget),
                                );
                                state.insert_event(
                                    Event::new(WindowEvent::MouseOut).target(state.hovered),
                                );

                                state.insert_event(
                                    Event::new(WindowEvent::Restyle)
                                        .origin(hovered_widget)
                                        .target(Entity::root()),
                                );
                                state.insert_event(
                                    Event::new(WindowEvent::Restyle)
                                        .origin(state.hovered)
                                        .target(Entity::root()),
                                );

                                state.hovered = hovered_widget;
                                state.active = Entity::null();

                                state.insert_event(
                                    Event::new(WindowEvent::Redraw).target(Entity::root()),
                                );
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
