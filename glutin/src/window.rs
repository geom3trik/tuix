use glutin::dpi::*;
use glutin::event_loop::{EventLoop, EventLoopWindowTarget};
use glutin::window::{WindowBuilder, WindowId};
use glutin::ContextBuilder;

use femtovg::{renderer::OpenGl, Canvas, Color};

use tuix_core::*;

#[derive(Default)]
pub struct WindowWidget {
    pub handle: Option<glutin::WindowedContext<glutin::PossiblyCurrent>>,
}

impl WindowWidget {
    pub fn id(&self) -> WindowId {
        self.handle.as_ref().unwrap().window().id()
    }

    pub fn build_window(self, state: &mut State) {
        state.build(Entity::root(), self);
    }

    pub fn create_window(&mut self, eventloop: &EventLoopWindowTarget<()>) -> Canvas<OpenGl> {
        println!("Spawn a window!");
        let window_builder = WindowBuilder::new().with_inner_size(PhysicalSize::new(
            300,
            300,
        ));
        
        let handle = ContextBuilder::new()
            .with_vsync(true)
            // .with_srgb(true)
            .build_windowed(window_builder, &eventloop)
            .expect("Window context creation failed!");

        let handle = unsafe { handle.make_current().unwrap() };

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
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

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

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

        handle.swap_buffers();

        

        self.handle = Some(handle);

        canvas

    }
}

impl BuildHandler for WindowWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        println!("Build a Window");
        state.insert_event(Event::new(AppEvent::AddWindow(entity)).target(entity));

        state.data.set_window(entity, entity);

        entity
            .set_position(state, tuix_core::Position::Absolute)
            .set_width(state, Length::Pixels(300.0))
            .set_height(state, Length::Pixels(300.0))
            //.set_background_color(state, Color::rgb(50, 200, 200))
            //.set_visibility(state, Visibility::Invisible)
    }
}

impl EventHandler for WindowWidget {
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
                        handle.window().request_redraw();
                    }
                }

                _=> {}
            }
        }
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

        Window { handle, canvas }
    }
}
