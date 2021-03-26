use crate::{application::ApplicationRunner, Renderer};
use baseview::{Event, EventStatus, Window, WindowHandler, WindowOpenOptions, WindowScalePolicy};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tuix_core::{Entity, State, WindowDescription};
use raw_gl_context::{GlConfig, GlContext};
use femtovg::{renderer::OpenGl, Canvas, Color};
/// Handles an tuix_baseview application
pub(crate) struct TuixWindow {
    context: GlContext,
    canvas: Canvas<OpenGl>,
}

impl TuixWindow {
    pub(crate) fn new(state: State, win_desc: WindowDescription, window: &mut baseview::Window) -> TuixWindow {

        let mut gl_config = GlConfig::default();
        gl_config.vsync = true;

        let context =
        GlContext::create(window, gl_config).expect("OpenGL context creation failed");

        context.make_current();

        let renderer = OpenGl::new(|s| context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
        
        // canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        // canvas.clear_rect(
        //     0,
        //     0,
        //     size.width as u32,
        //     size.height as u32,
        //     Color::rgb(255, 80, 80),
        // );

        context.make_not_current();


        TuixWindow {
            context,
            canvas,
        }
    }
}

impl WindowHandler for TuixWindow {
    fn on_frame(&mut self, _window: &mut Window) {
        self.application.on_frame_update();

        self.context.make_current();

        if self.application.render() {
            self.context.swap_buffers();
        }

        self.context.make_not_current();
    }

    fn on_event(&mut self, _window: &mut Window<'_>, event: Event) -> EventStatus {
        let mut should_quit = false;
        self.application.handle_event(event, &mut should_quit);

        if should_quit {
            // TODO: Request close.
        }

        EventStatus::Ignored
    }
}

fn load_renderer(window: &Window) -> (Renderer, raw_gl_context::GlContext) {
    let mut config = raw_gl_context::GlConfig::default();
    config.vsync = true;

    let context = raw_gl_context::GlContext::create(window, config).unwrap();

    context.make_current();

    let renderer = femtovg::renderer::OpenGl::new(|s| context.get_proc_address(s) as *const _)
        .expect("Cannot create renderer");

    context.make_not_current();

    (renderer, context)
}
