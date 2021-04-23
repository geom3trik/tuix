//use crate::event_manager::EventManager;
use crate::window::TuixWindow;
use crate::Renderer;
use baseview::{Window, WindowScalePolicy};
use femtovg::Canvas;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tuix_core::state::hierarchy::IntoHierarchyIterator;
use tuix_core::state::mouse::{MouseButton, MouseButtonState};
use tuix_core::state::Fonts;
use tuix_core::window::WindowWidget;
use tuix_core::{
    events::{Event, Propagation},
    window_description,
    BoundingBox
};
use tuix_core::{
    Entity, EventManager, Hierarchy, PropSet, Size, State, Units, Visibility, WindowDescription,
    WindowEvent,
};

pub struct Application<F>
where
    F: FnMut(&mut State, Entity),
    F: 'static + Send,
{
    app: F,
    window_description: WindowDescription,
}

impl<F> Application<F>
where
    F: FnMut(&mut State, Entity),
    F: 'static + Send,
{
    pub fn new(window_description: WindowDescription, app: F) -> Self {
        Self {
            app,
            window_description,
        }
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// Do **not** use this in the context of audio plugins, unless it is compiled as a
    /// standalone application.
    ///
    /// * `app` - The Tuix application builder.
    pub fn run(self) {
        TuixWindow::open_blocking(self.window_description, self.app)
    }

    /// Open a new child window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Tuix application builder.
    pub fn open_parented<P: HasRawWindowHandle>(self, parent: &P) {
        TuixWindow::open_parented(parent, self.window_description, self.app)
    }

    /// Open a new window as if it had a parent window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_as_if_parented(self) -> RawWindowHandle {
        TuixWindow::open_as_if_parented(self.window_description, self.app)
    }
}

pub(crate) struct ApplicationRunner {
    state: State,
    event_manager: EventManager,
    canvas: Canvas<Renderer>,
    hierarchy: Hierarchy,
    pos: (f32, f32),
    should_redraw: bool,
    scale_policy: WindowScalePolicy,
    scale_factor: f64,
}

impl ApplicationRunner {
    pub fn new(mut state: State, win_desc: WindowDescription, renderer: Renderer) -> Self {
        let event_manager = EventManager::new();

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        // TODO: Get scale policy from `win_desc`.
        let scale_policy = WindowScalePolicy::SystemScaleFactor;

        // Assume scale for now until there is an event with a new one.
        let scale = match scale_policy {
            WindowScalePolicy::ScaleFactor(scale) => scale,
            WindowScalePolicy::SystemScaleFactor => 1.0,
        };

        let logical_size = win_desc.inner_size;
        let physical_size = Size {
            width: (logical_size.width as f64 * scale).round() as u32,
            height: (logical_size.height as f64 * scale).round() as u32,
        };

        canvas.set_size(physical_size.width, physical_size.height, 1.0);

        let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../../resources/entypo.ttf");
        let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");

        let fonts = Fonts {
            regular: Some(canvas.add_font_mem(regular_font).expect("Cannot add font")),
            bold: Some(canvas.add_font_mem(bold_font).expect("Cannot add font")),
            icons: Some(canvas.add_font_mem(icon_font).expect("Cannot add font")),
            emoji: Some(canvas.add_font_mem(emoji_font).expect("Cannot add font")),
            arabic: Some(canvas.add_font_mem(emoji_font).expect("Cannot add font")),
        };

        state.fonts = fonts;

        canvas.scale(scale as f32, scale as f32);

        state
            .style
            .width
            .insert(Entity::root(), Units::Pixels(logical_size.width as f32));
        state
            .style
            .height
            .insert(Entity::root(), Units::Pixels(logical_size.height as f32));

        state
            .data
            .set_width(Entity::root(), physical_size.width as f32);
        state
            .data
            .set_height(Entity::root(), physical_size.height as f32);
        state.data.set_opacity(Entity::root(), 1.0);

        let mut bounding_box = BoundingBox::default();
        bounding_box.w = logical_size.width as f32;
        bounding_box.h = logical_size.height as f32;

        state.data.set_clip_region(Entity::root(), bounding_box);

        WindowWidget::new().build_window(&mut state);

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));

        let hierarchy = state.hierarchy.clone();

        //tuix_core::systems::apply_styles(&mut state, &hierarchy);

        ApplicationRunner {
            event_manager,
            state,
            canvas,
            hierarchy,
            pos: (0.0, 0.0),
            should_redraw: true,
            scale_policy,
            scale_factor: scale,
        }
    }

    /*
    pub fn get_window(&self) -> Entity {
        self.Entity::root()
    }

    pub fn get_state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn get_event_manager(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }
    */

    pub fn on_frame_update(&mut self) {

        while !self.state.event_queue.is_empty() {
            self.event_manager.flush_events(&mut self.state);
        } 

        if self.state.apply_animations() {
            self.state.insert_event(
                Event::new(WindowEvent::Relayout)
                    .target(Entity::root())
            );
            let hierarchy = self.state.hierarchy.clone();
            tuix_core::apply_layout2(&mut self.state, &hierarchy);
            //self.state.insert_event(Event::new(WindowEvent::Redraw));
            self.should_redraw = true;
        }

          

        if self.state.needs_redraw {
            // TODO - Move this to EventManager
            self.should_redraw = true;
            self.state.needs_redraw = false;
        }

    }

    pub fn render(&mut self) -> bool {
        if self.should_redraw {
            //println!("Redraw");
            let hierarchy = self.state.hierarchy.clone();
            tuix_core::apply_clipping(&mut self.state, &hierarchy);
            self.event_manager.draw(&mut self.state, &mut self.canvas);
            self.should_redraw = false;
            true
        } else {
            false
        }
    }

    pub fn handle_event(&mut self, event: baseview::Event, should_quit: &mut bool) {
        if requests_exit(&event) {
            self.state
                .insert_event(Event::new(WindowEvent::WindowClose));
            *should_quit = true;
        }

        match event {
            baseview::Event::Mouse(event) => match event {
                baseview::MouseEvent::CursorMoved { position } => {
                    let cursorx = (position.x) as f32;
                    let cursory = (position.y) as f32;

                    self.state.mouse.cursorx = cursorx;
                    self.state.mouse.cursory = cursory;

                    let mut hovered_widget = Entity::root();

                    // This only really needs to be computed when the hierarchy changes
                    // Can be optimised
                    let mut draw_hierarchy: Vec<Entity> =
                        self.state.hierarchy.into_iter().collect();

                    draw_hierarchy
                        .sort_by_cached_key(|entity| self.state.data.get_z_order(*entity));

                    for widget in draw_hierarchy.into_iter() {
                        // Skip invisible widgets
                        if self.state.data.get_visibility(widget) == Visibility::Invisible {
                            continue;
                        }

                        // This shouldn't be here but there's a bug if it isn't
                        if self.state.data.get_opacity(widget) == 0.0 {
                            continue;
                        }

                        // Skip non-hoverable widgets
                        if self.state.data.get_hoverability(widget) != true {
                            continue;
                        }

                        let border_width = match self
                            .state
                            .style
                            .border_width
                            .get(widget)
                            .cloned()
                            .unwrap_or_default()
                        {
                            Units::Pixels(val) => val,
                            //Units::Percentage(val) => parent_width * val,
                            _ => 0.0,
                        };

                        let posx = self.state.data.get_posx(widget) - (border_width / 2.0);
                        let posy = self.state.data.get_posy(widget) - (border_width / 2.0);
                        let width = self.state.data.get_width(widget) + (border_width);
                        let height = self.state.data.get_height(widget) + (border_width);

                        let clip_bounds = self.state.data.get_clip_region(widget);

                        let clip_posx = clip_bounds.x;
                        let clip_posy = clip_bounds.y;
                        let clip_width = clip_bounds.w;
                        let clip_height = clip_bounds.h;

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
                                self.state.style.pseudo_classes.get_mut(hovered_widget)
                            {
                                pseudo_classes.set_over(true);
                            }
                        } else {
                            if let Some(pseudo_classes) =
                                self.state.style.pseudo_classes.get_mut(hovered_widget)
                            {
                                pseudo_classes.set_over(false);
                            }
                        }
                    }

                    if hovered_widget != self.state.hovered {
                        // Useful for debugging

                        // println!(
                        //     "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {}",
                        //     hovered_widget,
                        //     self.state.hierarchy.get_parent(hovered_widget),
                        //     self.state.transform.get_posx(hovered_widget),
                        //     self.state.transform.get_posy(hovered_widget),
                        //     self.state.transform.get_width(hovered_widget),
                        //     self.state.transform.get_height(hovered_widget),
                        //     self.state.transform.get_z_order(hovered_widget),
                        // );

                        if let Some(pseudo_classes) =
                            self.state.style.pseudo_classes.get_mut(hovered_widget)
                        {
                            pseudo_classes.set_hover(true);
                        }

                        if let Some(pseudo_classes) =
                            self.state.style.pseudo_classes.get_mut(self.state.hovered)
                        {
                            pseudo_classes.set_hover(false);
                        }

                        self.state.insert_event(
                            Event::new(WindowEvent::MouseOver).target(hovered_widget),
                        );
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseOut).target(self.state.hovered),
                        );

                        self.state
                            .insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()).origin(hovered_widget));
                        self.state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::root()).origin(self.state.hovered),
                        );

                        self.state.hovered = hovered_widget;
                        self.state.active = Entity::null();

                        self.state.insert_event(Event::new(WindowEvent::Redraw));
                    }

                    if self.state.captured != Entity::null() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                .target(self.state.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else if self.state.hovered != Entity::root() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                .target(self.state.hovered),
                        );
                    }

                    self.pos = (cursorx, cursory);
                }
                baseview::MouseEvent::ButtonPressed(button) => {
                    let b = match button {
                        baseview::MouseButton::Left => MouseButton::Left,
                        baseview::MouseButton::Right => MouseButton::Right,
                        baseview::MouseButton::Middle => MouseButton::Middle,
                        baseview::MouseButton::Other(id) => MouseButton::Other(id as u16),
                        baseview::MouseButton::Back => MouseButton::Other(4),
                        baseview::MouseButton::Forward => MouseButton::Other(5),
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
                    };

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
                            Event::new(WindowEvent::MouseDown(b)).target(self.state.hovered),
                        );
                    }

                    match b {
                        MouseButton::Left => {
                            self.state.mouse.left.pos_down =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.pressed = self.state.hovered;
                        }

                        MouseButton::Middle => {
                            self.state.mouse.middle.pos_down =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.pressed = self.state.hovered;
                        }

                        MouseButton::Right => {
                            self.state.mouse.right.pos_down =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
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
                        baseview::MouseButton::Other(id) => MouseButton::Other(id as u16),
                        baseview::MouseButton::Back => MouseButton::Other(4),
                        baseview::MouseButton::Forward => MouseButton::Other(5),
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
                    };

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
                            Event::new(WindowEvent::MouseUp(b)).target(self.state.hovered),
                        );
                    }

                    match b {
                        MouseButton::Left => {
                            self.state.mouse.left.pos_up =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.released = self.state.hovered;
                        }

                        MouseButton::Middle => {
                            self.state.mouse.middle.pos_up =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.released = self.state.hovered;
                        }

                        MouseButton::Right => {
                            self.state.mouse.right.pos_up =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.released = self.state.hovered;
                        }

                        _ => {}
                    }
                }
                baseview::MouseEvent::WheelScrolled(scroll_delta) => {
                    let (lines_x, lines_y) = match scroll_delta {
                        baseview::ScrollDelta::Lines { x, y } => (x, y),
                        baseview::ScrollDelta::Pixels { x, y } => (
                            if x < 0.0 {
                                -1.0
                            } else if x > 1.0 {
                                1.0
                            } else {
                                0.0
                            },
                            if y < 0.0 {
                                -1.0
                            } else if y > 1.0 {
                                1.0
                            } else {
                                0.0
                            },
                        ),
                    };

                    if self.state.captured != Entity::null() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseScroll(lines_x, lines_y))
                                .target(self.state.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseScroll(lines_x, lines_y))
                                .target(self.state.hovered),
                        );
                    }
                }
                _ => {}
            },
            baseview::Event::Keyboard(event) => {
                use keyboard_types::Code;

                let (s, pressed) = match event.state {
                    keyboard_types::KeyState::Down => (MouseButtonState::Pressed, true),
                    keyboard_types::KeyState::Up => (MouseButtonState::Released, false),
                };

                match event.code {
                    Code::ShiftLeft | Code::ShiftRight => self.state.modifiers.shift = pressed,
                    Code::ControlLeft | Code::ControlRight => self.state.modifiers.ctrl = pressed,
                    Code::AltLeft | Code::AltRight => self.state.modifiers.alt = pressed,
                    Code::MetaLeft | Code::MetaRight => self.state.modifiers.logo = pressed,
                    _ => (),
                }

                if event.code == Code::F5 && s == MouseButtonState::Pressed {
                    self.state.reload_styles().unwrap();
                }

                if event.code == Code::Tab && s == MouseButtonState::Pressed {
                    let next_focus = self
                        .state
                        .style
                        .focus_order
                        .get(self.state.focused)
                        .cloned()
                        .unwrap_or_default()
                        .next;
                    let prev_focus = self
                        .state
                        .style
                        .focus_order
                        .get(self.state.focused)
                        .cloned()
                        .unwrap_or_default()
                        .prev;

                    if self.state.modifiers.shift {
                        if prev_focus != Entity::null() {
                            self.state.focused.set_focus(&mut self.state, false);
                            self.state.focused = prev_focus;
                            self.state.focused.set_focus(&mut self.state, true);
                        } else {
                            // TODO impliment reverse iterator for hierarchy
                            // state.focused = match state.focused.into_iter(&state.hierarchy).next() {
                            //     Some(val) => val,
                            //     None => Entity::root(),
                            // };
                        }
                    } else {
                        if next_focus != Entity::null() {
                            self.state.focused.set_focus(&mut self.state, false);
                            self.state.focused = next_focus;
                            self.state.focused.set_focus(&mut self.state, true);
                        } else {
                            self.state.focused.set_focus(&mut self.state, false);
                            self.state.focused =
                                match self.state.focused.into_iter(&self.hierarchy).next() {
                                    Some(val) => val,
                                    None => Entity::root(),
                                };
                            self.state.focused.set_focus(&mut self.state, true);
                        }
                    }

                    self.state.insert_event(
                        Event::new(WindowEvent::Restyle)
                            .target(Entity::root())
                            .origin(Entity::root()),
                    );
                }

                match s {
                    MouseButtonState::Pressed => {
                        if self.state.focused != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyDown(
                                    event.code,
                                    Some(event.key.clone()),
                                ))
                                .target(self.state.focused)
                                .propagate(Propagation::DownUp),
                            );
                        } else {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyDown(
                                    event.code,
                                    Some(event.key.clone()),
                                ))
                                .target(self.state.hovered)
                                .propagate(Propagation::DownUp),
                            );
                        }

                        if let keyboard_types::Key::Character(written) = &event.key {
                            for chr in written.chars() {
                                self.state.insert_event(
                                    Event::new(WindowEvent::CharInput(chr))
                                        .target(self.state.focused)
                                        .propagate(Propagation::Down),
                                );
                            }
                        }
                    }

                    MouseButtonState::Released => {
                        if self.state.focused != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyUp(event.code, Some(event.key)))
                                    .target(self.state.focused)
                                    .propagate(Propagation::DownUp),
                            );
                        } else {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyUp(event.code, Some(event.key)))
                                    .target(self.state.hovered)
                                    .propagate(Propagation::DownUp),
                            );
                        }
                    }
                }
            }
            baseview::Event::Window(event) => match event {
                baseview::WindowEvent::Focused => {
                    self.state.insert_event(
                        Event::new(WindowEvent::Restyle)
                            .target(Entity::root())
                            .origin(Entity::root()),
                    );
                }
                baseview::WindowEvent::Resized(window_info) => {
                    self.scale_factor = match self.scale_policy {
                        WindowScalePolicy::ScaleFactor(scale) => scale,
                        WindowScalePolicy::SystemScaleFactor => window_info.scale(),
                    };

                    let logical_size = (
                        (window_info.physical_size().width as f64 / self.scale_factor),
                        (window_info.physical_size().height as f64 / self.scale_factor),
                    );

                    let physical_size = (
                        window_info.physical_size().width,
                        window_info.physical_size().height,
                    );

                    self.state
                        .style
                        .width
                        .insert(Entity::root(), Units::Pixels(logical_size.0 as f32));
                    self.state
                        .style
                        .height
                        .insert(Entity::root(), Units::Pixels(logical_size.1 as f32));

                    self.state
                        .data
                        .set_width(Entity::root(), physical_size.0 as f32);
                    self.state
                        .data
                        .set_height(Entity::root(), physical_size.1 as f32);

                    self.state
                        .insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()).origin(Entity::root()));
                    self.state
                        .insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()).target(Entity::null()));
                    self.state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                }
                baseview::WindowEvent::WillClose => {
                    self.state
                        .insert_event(Event::new(WindowEvent::WindowClose));
                }
                _ => {}
            },
        }
    }
}

/// Returns true if the provided event should cause an [`Application`] to
/// exit.
pub fn requests_exit(event: &baseview::Event) -> bool {
    match event {
        baseview::Event::Window(baseview::WindowEvent::WillClose) => true,
        #[cfg(target_os = "macos")]
        baseview::Event::Keyboard(event) => {
            if event.code == keyboard_types::Code::KeyQ {
                if event.modifiers == keyboard_types::Modifiers::META {
                    if event.state == keyboard_types::KeyState::Down {
                        return true;
                    }
                }
            }

            false
        }
        _ => false,
    }
}
