use glutin::dpi::*;
use glutin::event::VirtualKeyCode;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use femtovg::{renderer::OpenGl, Canvas, Color};


use tuix_core::{WindowDescription};


pub struct Window {
    pub handle: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub canvas: Canvas<OpenGl>,
}

impl Window {
    pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title(&window_description.title)
            .with_inner_size(PhysicalSize::new(window_description.inner_size.width, window_description.inner_size.height))
            .with_min_inner_size(PhysicalSize::new(window_description.min_inner_size.width, window_description.min_inner_size.height))
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

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

        println!("width: {} height: {}", size.width, size.height);

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            Color::rgb(255, 80, 80),
        );

        let height = size.height as f32;
        let width = size.width as f32;

        Window { handle, canvas }
    }
}
