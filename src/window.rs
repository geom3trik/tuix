use glutin::dpi::*;
use glutin::event::VirtualKeyCode;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use femtovg::{
    renderer::OpenGl,
    Canvas,
    Color,
};

use crate::entity::Entity;

use crate::state::mouse::*;
use crate::{apply_clipping, apply_styles, apply_visibility, layout_fun, State, apply_z_ordering};

//use crate::state::style::*;

use crate::events::{Event, EventHandler};

//use nanovg::Font;

#[derive(Debug)]
pub enum SomeError {

}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardInput {
    pub scancode: u32,
    pub virtual_keycode: Option<VirtualKeyCode>,
    pub state: MouseButtonState,
}

impl KeyboardInput {
    pub fn from_keycode(keycode: VirtualKeyCode) -> Self {
        KeyboardInput {
            scancode: 0,
            virtual_keycode: Some(keycode),
            state: MouseButtonState::Pressed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorIcon {
    Arrow,
    NResize,
    EResize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Test,
    WindowClose,
    WindowResize(f32, f32),
    MouseButton(MouseButton, MouseButtonState),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove(f32, f32),
    MouseScroll(f32, f32),
    MouseOver,
    MouseOut,
    CharInput(char),
    //KeyInput(KeyboardInput),
    KeyDown(Option<VirtualKeyCode>),
    KeyUp(Option<VirtualKeyCode>),
    SetCursor(CursorIcon),
    MouseCaptureEvent,
    MouseCaptureOutEvent,
    Redraw,
    Restyle,
    Relayout,
}

pub struct WindowDescription {
    pub title: String,
    pub inner_size: glutin::dpi::Size,
    pub min_inner_size: glutin::dpi::Size,
    // Change this to resource id when the resource manager is working
    pub icon: Option<Vec<u8>>,
    pub icon_width: u32,
    pub icon_height: u32,
}

impl WindowDescription {
    pub fn new() -> Self {
        WindowDescription {
            title: "Default".to_string(),
            inner_size: Size::new(PhysicalSize::new(800, 600)),
            min_inner_size: Size::new(PhysicalSize::new(100, 100)),
            icon: None,
            icon_width: 0,
            icon_height: 0,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.inner_size = Size::new(PhysicalSize::new(width, height));

        self
    }

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.min_inner_size = Size::new(PhysicalSize::new(width, height));

        self
    }

    pub fn with_icon(mut self, icon: Vec<u8>, width: u32, height: u32) -> Self {
        self.icon = Some(icon);
        self.icon_width = width;
        self.icon_height = height;
        self
    }
}



pub struct Window {
    pub handle: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub canvas: Canvas<OpenGl>,
}

impl Window {
    pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title(&window_description.title)
            .with_inner_size(window_description.inner_size)
            .with_min_inner_size(window_description.min_inner_size)
            .with_window_icon(if let Some(icon) = &window_description.icon {
                Some(
                    glutin::window::Icon::from_rgba(
                        icon.clone(),
                        window_description.icon_width,
                        window_description.icon_height,
                    )
                    .unwrap(),
                )
            } else {
                None
            });

        let handle = ContextBuilder::new()
            .with_vsync(true)
            // .with_srgb(true)
            .build_windowed(window_builder, &events_loop)
            .expect("Window context creation failed!");

        let handle = unsafe { handle.make_current().unwrap() };

        gl::load_with(|ptr| handle.context().get_proc_address(ptr) as *const _);

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _).expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

        println!("width: {} height: {}", size.width, size.height);

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.3, 0.3, 0.32));

        let height = size.height as f32;
        let width = size.width as f32;

        Window { handle, canvas}
    }


}

#[derive(Clone)]
pub struct WindowWidget {}

impl WindowWidget {
    pub fn new() -> Self {
        WindowWidget {}
    }

    pub fn build_window(self, state: &mut State) {
        state.build(state.root, self);
    }
}

impl EventHandler for WindowWidget {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {

                WindowEvent::WindowClose => {
                    println!("Window Close Event");
                }

                WindowEvent::Restyle => {
                    apply_styles(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                }

                WindowEvent::Relayout => {
                    apply_z_ordering(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                    apply_clipping(state, &state.hierarchy.clone());
                    layout_fun(state, &state.hierarchy.clone());
                }

                _ => {}
            }
        }

        false
    }
}
