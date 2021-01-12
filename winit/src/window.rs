use winit::dpi::{PhysicalSize};
use winit::window::WindowBuilder;
use winit::event_loop::EventLoop;
use winit::window::Icon;

use femtovg::{renderer::OpenGl, Canvas, Color};
use raw_gl_context::{GlContext, GlConfig};

use tuix_core::WindowDescription;

pub struct Window {
    pub context: GlContext,
    pub canvas: Canvas<OpenGl>,
    pub window: winit::window::Window,
}

impl Window {
    pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title(&window_description.title)
            .with_inner_size(PhysicalSize::new(window_description.inner_size.width, window_description.inner_size.height))
            .with_min_inner_size(PhysicalSize::new(window_description.min_inner_size.width, window_description.min_inner_size.height))
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

        let window = window_builder.build(&events_loop).expect("Window creation failed");

        let mut gl_config = GlConfig::default();
        gl_config.vsync = true;
        
        let context = GlContext::create(&window, gl_config).expect("OpenGL context creation failed");

        context.make_current();

        let renderer = OpenGl::new(|s| context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let dpi_factor = window.scale_factor();
        let size = window.inner_size();

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            Color::rgb(255, 80, 80),
        );

        context.make_not_current();

        Window {
            context,
            canvas,
            window,
        }
    }
}