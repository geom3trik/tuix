use tuix_core::{BoundingBox, Length};
use tuix_core::{Entity, State};

use tuix_core::state::mouse::{MouseButton, MouseButtonState};

use tuix_core::events::{Event, EventManager, Propagation};

use tuix_core::state::hierarchy::IntoHierarchyIterator;

use tuix_core::state::Fonts;

use tuix_core::style::{Display, Visibility};

use tuix_core::state::style::prop::*;

use tuix_core::{WindowDescription, WindowEvent, WindowWidget};

use tuix_core::systems::*;
use baseview::{Event, EventStatus, Window, WindowHandler, WindowOpenOptions, WindowScalePolicy};

use crate::window::TuixWindow;
pub struct Application2 {
    state: State,
    event_manager: EventManager,
    context: raw_gl_context::GlContext,
}

impl Application2 {
    pub fn new<F: FnOnce(WindowDescription, &mut State, Entity) -> WindowDescription>(app: F) -> Self
    where
        F: FnMut(WindowDescription),
    {

        let mut state = State::new();
        let root = Entity::root();
        state.hierarchy.add(Entity::root(), None);


        let event_manager = EventManager::new();

        let window_description = app(WindowDescription::new(), &mut state, root);


        let window_settings = WindowOpenOptions {
            title: window_description.title.clone(),
            size: baseview::Size::new(
                window_description.inner_size.width as f64,
                window_description.inner_size.height as f64,
            ),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        


        Window::open_blocking(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {


                let regular_font = include_bytes!("../../resources/Roboto-Regular.ttf");
                let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
                let icon_font = include_bytes!("../../resources/entypo.ttf");
                let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");
        
                let fonts = Fonts {
                    regular: Some(
                        window
                            .canvas
                            .add_font_mem(regular_font)
                            .expect("Cannot add font"),
                    ),
                    bold: Some(
                        window
                            .canvas
                            .add_font_mem(bold_font)
                            .expect("Cannot add font"),
                    ),
                    icons: Some(
                        window
                            .canvas
                            .add_font_mem(icon_font)
                            .expect("Cannot add font"),
                    ),
                    emoji: Some(
                        window
                            .canvas
                            .add_font_mem(emoji_font)
                            .expect("Cannot add font"),
                    ),
                };
        
                state.fonts = fonts;
        
                state.style.width.insert(
                    Entity::root(),
                    Length::Pixels(window_description.inner_size.width as f32),
                );
                state.style.height.insert(
                    Entity::root(),
                    Length::Pixels(window_description.inner_size.height as f32),
                );
        
                state
                    .data
                    .set_width(Entity::root(), window_description.inner_size.width as f32);
                state
                    .data
                    .set_height(Entity::root(), window_description.inner_size.height as f32);
                state.data.set_opacity(Entity::root(), 1.0);
        
                let mut bounding_box = BoundingBox::default();
                bounding_box.w = window_description.inner_size.width as f32;
                bounding_box.h = window_description.inner_size.height as f32;
        
                state.data.set_clip_region(Entity::root(), bounding_box);
        
                tuix_core::WindowWidget::new().build_window(&mut state);

                TuixWindow::new(state, window_description, window)
            },
        );

        Self {
            state,
            event_manager,
        }
    }

    // pub fn parented() -> Self {

        
    //     Self {

    //     }
    // }
}
