use std::hash::BuildHasher;

use winit::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget, window::WindowId};
use winit::event_loop::EventLoop;
use winit::window::Icon;
use winit::window::WindowBuilder;

use femtovg::{renderer::OpenGl, Canvas};
use raw_gl_context::{GlConfig, GlContext};

use tuix_core::*;

#[derive(Default)]
pub struct WindowWidget2 {
    pub handle: Option<winit::window::Window>,
}

impl WindowWidget2 {
    pub fn id(&self) -> winit::window::WindowId {
        self.handle.as_ref().unwrap().id()
    }
    /*
    pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
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
            .build(&events_loop)
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

        WindowWidget2 {
            context: Some(context),
            canvas: Some(canvas),
            handle: Some(handle),
        }
    }
    */
    pub fn build_window(self, state: &mut State) {
        state.build(Entity::root(), self);
    }

    pub fn create_window(&mut self, eventloop: &EventLoopWindowTarget<()>) -> (GlContext, Canvas<OpenGl>) {
        println!("Spawn a window!");
        let window_builder = WindowBuilder::new().with_inner_size(PhysicalSize::new(
            300,
            300,
        ));
        let handle = window_builder
        .build(&eventloop)
        .expect("Window creation failed");

        let mut gl_config = GlConfig::default();
        gl_config.vsync = true;

        let context =
            GlContext::create(&handle, gl_config).expect("OpenGL context creation failed");

        context.make_current();

        let renderer = OpenGl::new(|s| context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

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

        let dpi_factor = handle.scale_factor();
        let size = handle.inner_size();

        println!("size: {:?}", size);

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            300 as u32,
            300 as u32,
            femtovg::Color::rgb(255, 80, 80),
        );

        canvas.flush();

        context.swap_buffers();

        context.make_not_current();

        

        self.handle = Some(handle);

        (context, canvas)

    }
}

impl BuildHandler for WindowWidget2 {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        println!("Build a Window");
        state.insert_event(Event::new(AppEvent::AddWindow(entity)).target(entity));

        state.data.set_window(entity, entity);

        entity
            .set_position(state, Position::Absolute)
            .set_width(state, Length::Pixels(300.0))
            .set_height(state, Length::Pixels(300.0))
            //.set_background_color(state, Color::rgb(50, 200, 200))
            //.set_visibility(state, Visibility::Invisible)
    }
}

impl EventHandler for WindowWidget2 {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            match app_event {
                AppEvent::Redraw => {
                    println!("Redraw Window");
                }

                _=> {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::Restyle => {
                    apply_styles(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                }

                WindowEvent::Relayout => {
                    //println!("Relayout");
                    apply_z_ordering(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                    apply_clipping(state, &state.hierarchy.clone());
                    apply_layout(state, &state.hierarchy.clone());
                }

                WindowEvent::Redraw => {
                    if let Some(handle) = &self.handle {
                        handle.request_redraw();
                    }
                }

                _=> {}
            }
        }
    }
}

pub struct Window {
    pub context: GlContext,
    pub canvas: Canvas<OpenGl>,
    pub handle: winit::window::Window,
}

impl Window {
    pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
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
            .build(&events_loop)
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
            femtovg::Color::rgb(80, 80, 255),
        );

        context.make_not_current();

        Window {
            context,
            canvas,
            handle,
        }
    }
}
