

#![allow(deprecated)]



use crate::window::{KeyboardInput, Window, WindowDescription, WindowEvent, WindowWidget};

use crate::{Entity, State};
use crate::{Length, Visibility};

use crate::state::mouse::{MouseButton, MouseButtonState};

use crate::events::{Event, EventManager, Propagation};

use crate::state::hierarchy::IntoHierarchyIterator;

use crate::state::Fonts;

use femtovg::{
    renderer::OpenGl,
    Canvas,
    Color,
};

use baseview::{WindowHandler, WindowScalePolicy};
use keyboard_types::{KeyboardEvent, KeyState};

use raw_gl_context::GlContext;

struct OpenWindowExample {
    context: GlContext,
    canvas: Canvas<OpenGl>,
    state: State,
    event_manager: EventManager,
}

impl WindowHandler for OpenWindowExample {
    fn on_frame(&mut self) {

        self.canvas.set_size(512, 512, 1.0);
        self.canvas.clear_rect(0, 0, 512, 512, Color::rgbf(0.3, 0.3, 0.32));

        let hierarchy = self.state.hierarchy.clone();

        if self.state.apply_animations() {
            self.state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()).origin(Entity::new(0, 0)));
            self.state.insert_event(Event::new(WindowEvent::Redraw));
        }

        //while !self.state.event_queue.is_empty() {
            self.event_manager.flush_events(&mut self.state);



            self.event_manager.draw(&mut self.state, &hierarchy, &mut self.canvas);
            
        //}

        self.context.make_current();

        // unsafe {
        //     gl::ClearColor(1.0, 0.0, 1.0, 1.0);
        //     gl::Clear(gl::COLOR_BUFFER_BIT);
        // }





        //draw_colorwheel(&mut self.canvas, 200.0, 200.0, 200.0, 200.0, 0.0);

        self.canvas.flush();
        self.context.swap_buffers();
    }

    fn on_event(&mut self, _window: &mut baseview::Window, event: baseview::Event) {
        match event {
            baseview::Event::Mouse(e) => {
                match e {
                    baseview::MouseEvent::CursorMoved{position} => {
                        let cursorx = (position.x) as f32;
                        let cursory = (position.y) as f32;

                        self.state.mouse.cursorx = cursorx as f32;
                        self.state.mouse.cursory = cursory as f32;

                        let mut hovered_widget = Entity::new(0, 0);

                        // This only really needs to be computed when the hierarchy changes
                        // Can be optimised
                        let mut draw_hierarchy: Vec<Entity> = self.state.hierarchy.into_iter().collect();

                        draw_hierarchy.sort_by_cached_key(|entity| self.state.transform.get_z_order(*entity));


                        for widget in draw_hierarchy.into_iter() {
                            // Skip invisible widgets
                            if self.state.transform.get_visibility(widget) == Visibility::Invisible
                            {
                                continue;
                            }
                            
                            // This shouldn't be here but there's a bug if it isn't 
                            if self.state.transform.get_opacity(widget) == 0.0 {
                                continue;
                            }
                            
                            // Skip non-hoverable widgets
                            if self.state.transform.get_hoverability(widget) != true {
                                continue;
                            }

                            let border_width = self.state
                                .style
                                .border_width
                                .get(widget)
                                .cloned()
                                .unwrap_or_default();

                            let posx = self.state.transform.get_posx(widget) - (border_width / 2.0);
                            let posy = self.state.transform.get_posy(widget) - (border_width / 2.0);
                            let width = self.state.transform.get_width(widget) + (border_width);
                            let height = self.state.transform.get_height(widget) + (border_width);

                            let clip_widget = self.state.transform.get_clip_widget(widget);


                            let clip_posx = self.state.transform.get_posx(clip_widget);
                            let clip_posy = self.state.transform.get_posy(clip_widget);
                            let clip_width = self.state.transform.get_width(clip_widget);
                            let clip_height = self.state.transform.get_height(clip_widget);

                            if cursorx >= posx && cursorx >= clip_posx
                                && cursorx < (posx + width) && cursorx < (clip_posx + clip_width)
                                && cursory >= posy && cursory >= clip_posy
                                && cursory < (posy + height) && cursory < (clip_posy + clip_height)
                            {
                                hovered_widget = widget;
                                if let Some(pseudo_classes) = self.state.style.pseudo_classes.get_mut(hovered_widget) {
                                    pseudo_classes.set_over(true);
                                }
                            } else {
                                if let Some(pseudo_classes) = self.state.style.pseudo_classes.get_mut(hovered_widget) {
                                    pseudo_classes.set_over(false);
                                }
                            }
                        }

                        if hovered_widget != self.state.hovered {

                            // Useful for debugging
                        
                            println!(
                                "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {}",
                                hovered_widget,
                                self.state.hierarchy.get_parent(hovered_widget),
                                self.state.transform.get_posx(hovered_widget),
                                self.state.transform.get_posy(hovered_widget),
                                self.state.transform.get_width(hovered_widget),
                                self.state.transform.get_height(hovered_widget),
                                self.state.transform.get_z_order(hovered_widget),
                            );

                            if let Some(pseudo_classes) = self.state.style.pseudo_classes.get_mut(hovered_widget) {
                                pseudo_classes.set_hover(true);
                            }

                            if let Some(pseudo_classes) = self.state.style.pseudo_classes.get_mut(self.state.hovered) {
                                pseudo_classes.set_hover(false);
                            }

                            self.state.insert_event(Event::new(WindowEvent::MouseOver).target(hovered_widget));
                            self.state.insert_event(Event::new(WindowEvent::MouseOut).target(self.state.hovered));

                            self.state.hovered = hovered_widget;
                            self.state.active = Entity::null();

                            self.state
                                .insert_event(Event::new(WindowEvent::Restyle));
                            self.state
                                .insert_event(Event::new(WindowEvent::Redraw));
                        }

                        if self.state.captured != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                    .target(self.state.captured)
                                    .propagate(Propagation::Direct),
                            );
                        } else if self.state.hovered != Entity::new(0, 0) {
                            self.state.insert_event(
                                Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                    .target(self.state.hovered),
                            );
                        }
                    }

                    baseview::MouseEvent::ButtonPressed(button) => {
                        let b = match button {
                            baseview::MouseButton::Left => MouseButton::Left,
                            baseview::MouseButton::Right => MouseButton::Right,
                            baseview::MouseButton::Middle => MouseButton::Middle,
                            baseview::MouseButton::Other(id) => MouseButton::Other(id),
                            _=> MouseButton::Left,
                        };

                        match b {
                            MouseButton::Left => {
                                self.state.mouse.left.state = MouseButtonState::Pressed;
                            }

                            MouseButton::Right => {
                                self.state.mouse.right.state = MouseButtonState::Pressed;
                            }

                            MouseButton::Middle => {
                                self.state.mouse.middle.state = MouseButtonState::Pressed;
                            }

                            _ => {}
                        }

                        if self.state.hovered != Entity::null()
                            && self.state.active != self.state.hovered
                        {
                            self.state.active = self.state.hovered;
                            self.state.insert_event(Event::new(WindowEvent::Restyle));
                        }

                        if self.state.captured != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::MouseDown(b))
                                    .target(self.state.captured)
                                    .propagate(Propagation::Direct),
                            );
                        } else {
                            self.state.insert_event(
                                Event::new(WindowEvent::MouseDown(b))
                                    .target(self.state.hovered),
                            );
                        }

                        match b {
                            MouseButton::Left => {
                                self.state.mouse.left.pos_down = (self.state.mouse.cursorx, self.state.mouse.cursory);
                                self.state.mouse.left.pressed = self.state.hovered;
                            }

                            MouseButton::Middle => {
                                self.state.mouse.middle.pos_down = (self.state.mouse.cursorx, self.state.mouse.cursory);
                                self.state.mouse.left.pressed = self.state.hovered;
                            }
                            
                            MouseButton::Right => {
                                self.state.mouse.right.pos_down = (self.state.mouse.cursorx, self.state.mouse.cursory);
                                self.state.mouse.left.pressed = self.state.hovered;
                            } 
                            
                            _ => {}
                        }
                            
                    }

                    baseview::MouseEvent::ButtonReleased(button) => {

                        let b = match button {
                            baseview::MouseButton::Left => MouseButton::Left,
                            baseview::MouseButton::Right => MouseButton::Right,
                            baseview::MouseButton::Middle => MouseButton::Middle,
                            baseview::MouseButton::Other(id) => MouseButton::Other(id),
                            _=> MouseButton::Left,
                        };

                        match b {
                            MouseButton::Left => {
                                self.state.mouse.left.state = MouseButtonState::Released;
                            }

                            MouseButton::Right => {
                                self.state.mouse.right.state = MouseButtonState::Released;
                            }

                            MouseButton::Middle => {
                                self.state.mouse.middle.state = MouseButtonState::Released;
                            }

                            _ => {}
                        }

                        self.state.active = Entity::null();
                        self.state.insert_event(Event::new(WindowEvent::Restyle));

                        if self.state.captured != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::MouseUp(b))
                                    .target(self.state.captured)
                                    .propagate(Propagation::Direct),
                            );
                        } else {
                            self.state.insert_event(
                                Event::new(WindowEvent::MouseUp(b))
                                    .target(self.state.hovered),
                            );
                        }

                        match b {
                            MouseButton::Left => {
                                self.state.mouse.left.pos_up = (self.state.mouse.cursorx, self.state.mouse.cursory);
                                self.state.mouse.left.released = self.state.hovered;
                            }
                            
                            MouseButton::Middle => {
                                self.state.mouse.middle.pos_up = (self.state.mouse.cursorx, self.state.mouse.cursory);
                                self.state.mouse.left.released = self.state.hovered;
                            } 
                                
                            MouseButton::Right => {
                                self.state.mouse.right.pos_up = (self.state.mouse.cursorx, self.state.mouse.cursory);
                                self.state.mouse.left.released = self.state.hovered;
                            }

                            _ => {}
                        }
                    }

                    _=> {}
                }
                //println!("Mouse event: {:?}", e)
            },
            baseview::Event::Keyboard(e) => {
                match e {
                    KeyboardEvent {state: s, key, code, location, modifiers, repeat, is_composing} => {
                        
                        
                        if s == KeyState::Down {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyInput(
                                    KeyboardInput {
                                        scancode: 0,
                                        virtual_keycode: Some(crate::VirtualKeyCode::Z),
                                        state: MouseButtonState::Pressed,
                                    }
                                ))
                                .target(self.state.hovered)
                            );
                        }

                        if s == KeyState::Up {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyInput(
                                    KeyboardInput {
                                        scancode: 0,
                                        virtual_keycode: Some(crate::VirtualKeyCode::Z),
                                        state: MouseButtonState::Released,
                                    }
                                ))
                                .target(self.state.hovered)
                            );
                        }


                        
                        
                    }
                }
                //println!("Keyboard event: {:?}", e);
            },
            
            
            
            baseview::Event::Window(e) => println!("Window event: {:?}", e),
        }
    }
}

pub struct ApplicationBV {
    //pub state: State,
    //pub event_manager: EventManager,
    pub app_runner: Option<baseview::AppRunner>
}

impl ApplicationBV {
    pub fn new<F: 'static + Send + FnMut(&mut State, Entity)>(mut win: F) -> Self {
        
        //let mut state = State::new();

        //let event_manager = EventManager::new();

        //let window_description = win(WindowDescription::new());

        let window_open_options = baseview::WindowOpenOptions {
            title: "baseview".into(),
            size: baseview::Size::new(512.0, 512.0),
            scale: WindowScalePolicy::SystemScaleFactor,
            parent: baseview::Parent::None,
        };

        let opt_app_runner = baseview::Window::open(
            window_open_options,
            move |window| {
                let context = GlContext::create(window, Default::default()).unwrap();
                context.make_current();
                gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
                let renderer = OpenGl::new(|symbol| context.get_proc_address(symbol) as *const _).expect("Cannot create renderer");
                let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    
                // let fonts = Fonts {
                //     regular: Some(canvas
                //         .add_font("examples/resources/Roboto-Regular.ttf")
                //         .expect("Cannot add font")),
                //     bold: Some(canvas
                //         .add_font("examples/resources/Roboto-Bold.ttf")
                //         .expect("Cannot add font")),
                //     icons: Some(canvas.add_font("examples/resources/entypo.ttf").expect("Cannot add font")),
                // };

                // state.fonts = fonts;

                let mut state = State::new();


                state.style.width.insert(
                    state.root,
                    Length::Pixels(512.0),
                );

                state.style.height.insert(
                    state.root,
                    Length::Pixels(512.0),
                );
        
                state.transform.set_width(
                    state.get_root(),
                    512.0,
                );
                state.transform.set_height(
                    state.get_root(),
                    512.0,
                );
                state.transform.set_opacity(state.get_root(), 1.0);
        
                WindowWidget::new().build_window(&mut state);
        
                state.hierarchy.add(state.root, None);

                let root = state.root;

                state.insert_event(Event::new(WindowEvent::Restyle));
                state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));

                let regular_font = include_bytes!("../resources/Roboto-Regular.ttf");
                let bold_font = include_bytes!("../resources/Roboto-Bold.ttf");
                let icon_font = include_bytes!("../resources/Entypo.ttf");

                let fonts = Fonts {
                    regular: Some(window.canvas
                        .add_font_mem(regular_font)
                        .expect("Cannot add font")),
                    bold: Some(window.canvas
                        .add_font_mem(bold_font)
                        .expect("Cannot add font")),
                    icons: Some(window.canvas.add_font_mem(icon_font).expect("Cannot add font")),
                };

                state.fonts = fonts;

                win(&mut state, root);

                OpenWindowExample {context, canvas, state, event_manager: EventManager::new()}
            } 
        );

 
        ApplicationBV {
            // event_manager: event_manager,
            // state: state,
            app_runner: opt_app_runner,
        }
    }



    pub fn run(self) {
        let mut pos: (f32, f32) = (0.0, 0.0);

        //let mut state = self.state;
        //let mut event_manager = self.event_manager;


        let mut should_quit = false;

        let mut should_redraw = false;
        //let hierarchy = state.hierarchy.clone();

        //state.insert_event(Event::new(WindowEvent::Restyle));
        //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));

        self.app_runner.unwrap().app_run_blocking();

        /*
        self.event_loop.run(move |event, _, control_flow|{
        

            match event {
                GEvent::LoopDestroyed => return,

                GEvent::UserEvent(_) => {

                }

                GEvent::MainEventsCleared => {
                    if state.apply_animations() {
                        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()).origin(Entity::new(0, 0)));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                    }

                    while !state.event_queue.is_empty() {
                        if event_manager.flush_events(&mut state, &mut window) {
                            window.handle.window().request_redraw();
                        }
                    }
                }

                // REDRAW
                GEvent::RedrawRequested(_) => {
                    event_manager.draw(&mut state, &hierarchy, &mut window);
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
                                Event::new(WindowEvent::Restyle).target(state.root),
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

                            if let Some(virtual_keycode) = input.virtual_keycode {


                                if virtual_keycode == VirtualKeyCode::Tab && s == MouseButtonState::Pressed {

                                    let next_focus = state.style.focus_order.get(state.focused).cloned().unwrap_or_default().next;
                                    let prev_focus = state.style.focus_order.get(state.focused).cloned().unwrap_or_default().prev;

                                    if state.modifiers.shift {
                                        if prev_focus != Entity::null() {
                                            state.focused = prev_focus;
                                        } else {
                                            // TODO impliment reverse iterator for hierarchy
                                            // state.focused = match state.focused.into_iter(&state.hierarchy).next() {
                                            //     Some(val) => val,
                                            //     None => state.root,
                                            // };
                                        }
                                    } else {
                                        if next_focus != Entity::null() {
                                            state.focused = next_focus;
                                        } else {
                                            state.focused = match state.focused.into_iter(&hierarchy).next() {
                                                Some(val) => val,
                                                None => state.root,
                                            };
                                        }
                                    }

                                    state.insert_event(
                                        Event::new(WindowEvent::Restyle).target(state.root),
                                    );

                                    
                                }
                            }

                            if state.focused != Entity::null() {
                                state.insert_event(
                                    Event::new(WindowEvent::KeyInput(
                                        KeyboardInput {
                                            scancode: input.scancode,
                                            virtual_keycode: input.virtual_keycode,
                                            state: s,
                                        }
                                    ))
                                    .target(state.focused)
                                    .propagate(Propagation::DownUp),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(WindowEvent::KeyInput(
                                        KeyboardInput {
                                            scancode: input.scancode,
                                            virtual_keycode: input.virtual_keycode,
                                            state: s,
                                        }
                                    ))
                                    .target(state.hovered)
                                    .propagate(Propagation::DownUp),
                                );
                            }
                            
                            
                        }
    
                        glutin::event::WindowEvent::Resized(logical_size) => {
                            let physical_size = logical_size;

                            state.style.width.insert(state.root, Length::Pixels(physical_size.width as f32));
                            state.style.height.insert(state.root, Length::Pixels(physical_size.height as f32));
    
                            state
                                .transform
                                .set_width(state.root, physical_size.width as f32);
                            state
                                .transform
                                .set_height(state.root, physical_size.height as f32);

    
                            state.insert_event(Event::new(WindowEvent::Restyle));
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
    
                                let border_width = state
                                    .style
                                    .border_width
                                    .get(widget)
                                    .cloned()
                                    .unwrap_or_default();
    
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
                            
                                println!(
                                    "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {}",
                                    hovered_widget,
                                    state.hierarchy.get_parent(hovered_widget),
                                    state.transform.get_posx(hovered_widget),
                                    state.transform.get_posy(hovered_widget),
                                    state.transform.get_width(hovered_widget),
                                    state.transform.get_height(hovered_widget),
                                    state.transform.get_z_order(hovered_widget),
                                );

                                if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(hovered_widget) {
                                    pseudo_classes.set_hover(true);
                                }

                                if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(state.hovered) {
                                    pseudo_classes.set_hover(false);
                                }
    
                                state.insert_event(Event::new(WindowEvent::MouseOver).target(hovered_widget));
                                state.insert_event(Event::new(WindowEvent::MouseOut).target(state.hovered));
    
                                state.hovered = hovered_widget;
                                state.active = Entity::null();
    
                                state
                                    .insert_event(Event::new(WindowEvent::Restyle));
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
        */
    }
}
