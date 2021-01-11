use crate::{application::ApplicationRunner, Renderer};
use baseview::{Event, Window, WindowHandler, WindowOpenOptions, WindowScalePolicy};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tuix_core::{WindowDescription, Entity, State};

/// Handles an tuix_baseview application
pub(crate) struct TuixWindow {
    application: ApplicationRunner,
    context: raw_gl_context::GlContext,
}

impl TuixWindow {
    fn new(state: State, win_desc: WindowDescription, window: &mut baseview::Window) -> TuixWindow {
        let (renderer, context) = load_renderer(window);

        let application = ApplicationRunner::new(state, win_desc, renderer);

        TuixWindow {
            application,
            context,
        }
    }

    /// Open a new child window.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Tuix application builder.
    pub fn open_parented<P, F>(parent: &P, mut app: F)
    where
        P: HasRawWindowHandle,
        F: FnMut(WindowDescription, &mut State, Entity) -> WindowDescription,
        F: 'static,
    {
        let mut state = State::new();

        let root = state.root;
        state.hierarchy.add(state.root, None);

        let win_desc = WindowDescription::new();
        let win_desc = (app)(win_desc, &mut state, root);

        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(win_desc.inner_size.width as f64, win_desc.inner_size.height as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_parented(
            parent,
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {
                TuixWindow::new(state, win_desc, window)
            },
        )
    }

    /// Open a new window as if it had a parent window.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_as_if_parented<F>(mut app: F) -> RawWindowHandle
    where
        F: FnMut(WindowDescription, &mut State, Entity) -> WindowDescription,
        F: 'static,
    {
        let mut state = State::new();

        let root = state.root;
        state.hierarchy.add(state.root, None);

        let win_desc = WindowDescription::new();
        let win_desc = (app)(win_desc, &mut state, root);

        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(win_desc.inner_size.width as f64, win_desc.inner_size.height as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_as_if_parented(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {
                TuixWindow::new(state, win_desc, window)
            },
        )
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_blocking<F>(mut app: F)
    where
        F: FnMut(WindowDescription, &mut State, Entity) -> WindowDescription,
        F: 'static,
    {
        let mut state = State::new();

        let root = state.root;
        state.hierarchy.add(state.root, None);

        let win_desc = WindowDescription::new();
        let win_desc = (app)(win_desc, &mut state, root);

        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(win_desc.inner_size.width as f64, win_desc.inner_size.height as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_blocking(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {
                TuixWindow::new(state, win_desc, window)
            },
        )
    }
}

impl WindowHandler for TuixWindow {
    fn on_frame(&mut self) {
        self.application.on_frame_update();

        self.context.make_current();

        if self.application.render() {
            self.context.swap_buffers();
        }
        
        self.context.make_not_current();
    }

    fn on_event(&mut self, _window: &mut Window<'_>, event: Event) {
        let mut should_quit = false;
        self.application.handle_event(event, &mut should_quit);

        if should_quit {
            // TODO: Request close.
        }
    }
}

fn load_renderer(window: &Window) -> (Renderer, raw_gl_context::GlContext) {
    let mut config = raw_gl_context::GlConfig::default();
    config.vsync = true;

    let context =
        raw_gl_context::GlContext::create(window, config).unwrap();

    context.make_current();

    gl::load_with(|s| context.get_proc_address(s) as _);

    let renderer = femtovg::renderer::OpenGl::new(|s| context.get_proc_address(s) as *const _)
        .expect("Cannot create renderer");

    context.make_not_current();

    (renderer, context)
}
