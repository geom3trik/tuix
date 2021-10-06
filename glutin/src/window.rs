use glutin::{ContextWrapper, NotCurrent, PossiblyCurrent, dpi::*};
use glutin::event_loop::{EventLoop, EventLoopWindowTarget};
use glutin::window::{WindowBuilder, WindowId};
use glutin::ContextBuilder;

use femtovg::{renderer::OpenGl, Canvas, Color};

use tuix_core::{Builder, Entity, Event, GenerationalId, PositionType, PropSet, State, TreeExt, Units, Widget, WindowDescription, WindowEvent, WindowWidget};

use crate::application::AppEvent;

pub enum CurrentContextWrapper {
    PossiblyCurrent(ContextWrapper<PossiblyCurrent, glutin::window::Window>),
    NotCurrent(ContextWrapper<NotCurrent, glutin::window::Window>),
}

impl CurrentContextWrapper {
    pub fn window(&self) -> &glutin::window::Window {
        match self {
            CurrentContextWrapper::PossiblyCurrent(context) => context.window(),
            CurrentContextWrapper::NotCurrent(context) => context.window(),
        }
    }
}

pub struct Window {
    window_description: WindowDescription,
    pub handle: Option<CurrentContextWrapper>,
    pub canvas: Option<Canvas<OpenGl>>,
    window_widget: WindowWidget,
}

impl Window {

    pub fn new(window_description: WindowDescription) -> Self {
        Self {
            window_description,
            handle: None,
            canvas: None,
            window_widget: WindowWidget::new(),
        }
    }

    pub fn create(&mut self, event_loop: &EventLoopWindowTarget<()>) -> WindowId {
        //Windows COM doesn't play nicely with winit's drag and drop right now
        #[cfg(target_os = "windows")]
        let mut window_builder = {
            use glutin::platform::windows::WindowBuilderExtWindows;
            WindowBuilder::new().with_drag_and_drop(false)
        };
        #[cfg(not(target_os = "windows"))]
        let mut window_builder = WindowBuilder::new();

        window_builder = window_builder
            .with_title(self.window_description.title.clone())
            .with_inner_size(PhysicalSize::new(
                self.window_description.inner_size.width,
                self.window_description.inner_size.height,
            ))
            .with_min_inner_size(PhysicalSize::new(
                self.window_description.min_inner_size.width,
                self.window_description.min_inner_size.height,
            ))
            .with_window_icon(if let Some(icon) = self.window_description.icon.clone() {
                Some(
                    glutin::window::Icon::from_rgba(
                        icon.clone(),
                        self.window_description.icon_width,
                        self.window_description.icon_height,
                    )
                    .unwrap(),
                )
            } else {
                None
            });

        let handle = ContextBuilder::new()
            .with_vsync(true)
            // .with_srgb(true)
            .build_windowed(window_builder, &event_loop)
            .expect("Window context creation failed!");

        let handle = unsafe { handle.make_current().unwrap() };

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            Color::rgb(255, 80, 80),
        );

        // let height = size.height as f32;
        // let width = size.width as f32;

        let window_id = handle.window().id();

        self.handle = Some(CurrentContextWrapper::PossiblyCurrent(handle));
        self.canvas = Some(canvas);

        window_id
    }
}

impl Widget for Window {
    type Ret = Entity;
    type Data = ();

    fn build<F>(mut self, state: &mut State, parent: impl tuix_core::AsEntity, mut builder: F) -> Self::Ret
    where
            F: FnMut(Builder<Self>) -> Builder<Self>,
            Self: std::marker::Sized + 'static, 
    {
        // Create a new entity
        let entity = state.add_window(parent.entity());

        state.insert_event(Event::new(WindowEvent::ChildAdded(entity)).direct(parent.entity()));

        // Call the on_build function of the widget
        let ret = self.on_build(state, entity);

        // Call the builder closure
        builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        ret
    }

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.emit(state, AppEvent::CreateWindow(entity));


        let window_size = self.window_description.inner_size;
        // entity
        //     .set_width(state, Units::Pixels(window_size.to_physical::<u32>(1.0).width as f32))
        //     .set_height(state, Units::Pixels(window_size.to_physical::<u32>(1.0).height as f32));

        entity
            .set_position_type(state, PositionType::SelfDirected)
            .set_left(state, Units::Pixels(0.0))
            .set_top(state, Units::Pixels(0.0))
            .set_width(state, Units::Pixels(window_size.width as f32))
            .set_height(state, Units::Pixels(window_size.height as f32));
        
        
        self.window_widget.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut tuix_core::Event) {
        self.window_widget.on_event(state, entity, event);
    }
}
