pub mod element;
pub use element::Element;

pub mod buttons;
pub use buttons::*;

pub mod inputs;
pub use inputs::*;

pub mod popups;
pub use popups::*;

pub mod menus;
pub use menus::*;

pub mod scrollbar;
pub use scrollbar::*;

pub mod progress;
pub use progress::*;

// pub mod tab;
// pub use tab::*;

// pub mod dropdown;
// pub use dropdown::*;

pub mod scroll_container;
pub use scroll_container::*;

// pub mod value_slider;
// pub use value_slider::ValueSlider;

// pub mod length_box;
// pub use length_box::LengthBox;

pub mod panel;
pub use panel::*;

pub mod label;
pub use label::*;

pub mod containers;
pub use containers::*;

pub mod window;
pub use window::WindowWidget;

pub mod tooltip;
pub use tooltip::*;

pub mod text_area;
pub use text_area::*;

// Audio Widgets
pub mod audio_widgets;
pub use audio_widgets::*;

pub mod debug_container;
pub use debug_container::*;

pub use crate::entity::Entity;
pub use crate::events::{Event, Propagation, Widget, WindowEvent};
pub use crate::mouse::*;
pub use crate::state::State;
pub use crate::{Animation, AnimationState, PropGet, PropSet};
pub use crate::{Code, Key};
pub type Canvas = femtovg::Canvas<femtovg::renderer::OpenGl>;
pub use crate::Units::*;

#[derive(Default)]
pub struct BaseWidget {
    on_hover: Option<Event>,
}

impl BaseWidget {
    pub fn on_hover(&mut self, event: Event) -> &mut Self {
        self.on_hover = Some(event);

        self
    }
}

// pub trait BasicWidget: Sized {
//     fn get_base_widget(&mut self) -> &mut BaseWidget;

//     fn on_hover(mut self, event: Event) -> Self
//     {
//         self.get_base_widget().on_hover(event);

//         self
//     }

//     fn on_active(mut self, event: Event) -> Self
//     {
//         self
//     }
// }
