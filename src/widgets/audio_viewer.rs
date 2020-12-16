use crate::component::entity::Entity;
use crate::component::state::WidgetState;
use crate::component::storage::Storage;
use crate::events::{EventHandler, EventQueue, WidgetEvent, WidgetList};
use crate::mouse::*;
use crate::widget::{Widget, WidgetBuilder};

use crate::component::style::display::*;
use crate::widget_system::WidgetSystem;

use crate::component::style::flexbox::*;
use crate::component::style::text::*;
use crate::widget::button::Button;
use crate::widget::checkbox::Checkbox;
use crate::widget::intbox::IntBox;
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};
use crate::widget::slider::Slider;

const ICON_SOUND: &str = "\u{e60d}";
const ICON_MUTE: &str = "\u{e60c}";
const ICON_HEADPHONES: &str = "\u{e6a8}";

use image::{
    DynamicImage, FilterType, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage,
};

pub struct AudioViewer {
    id: Entity,
    //audio_id: u16,
}

impl AudioViewer {
    pub fn new(state: &mut WidgetState, widget_list: &mut WidgetList, parent: Entity) -> Self {
        let id = state.add(parent).unwrap();
        id.set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column)
            .set_background(state, nanovg::Color::from_rgb(38, 38, 38));

        //id.set_background_image(state, "Dog".to_string());

        let container = state.add(id).unwrap();
        container
            .set_display(state, Display::Flexbox)
            .set_flex_grow(state, 0.0)
            .set_flex_basis(state, 50.0)
            .set_background(state, nanovg::Color::from_rgb(38, 38, 38));

        let controls = state.add(container).unwrap();
        controls
            .set_flex_grow(state, 0.0)
            .set_flex_basis(state, 200.0)
            .set_margin_right(state, 1.0)
            .set_display(state, Display::Flexbox);

        let mut mute_button = Checkbox::new(state, widget_list, controls);
        mute_button.set_icon_unchecked(state, ICON_SOUND);
        mute_button.set_icon_checked(state, ICON_MUTE);

        let mute_button = widget_list.push(mute_button);

        mute_button.set_font(state, "Icons2".to_string());

        let mut solo_button = Checkbox::new(state, widget_list, controls);
        solo_button.set_icon_unchecked(state, ICON_HEADPHONES);
        solo_button.set_icon_checked(state, ICON_HEADPHONES);

        let solo_button = widget_list.push(solo_button);
        solo_button.set_font(state, "Icons2".to_string());

        let audio_track = state.add(container).unwrap();

        AudioViewer { id: id }
    }

    pub fn create_waveform(&mut self, state: &mut WidgetState) {
        //Create an image the size of the viewer
        let width = state.transform.get_global_width(self.id) as u32;
        let height = state.transform.get_global_height(self.id) as u32;

        let mut d_image = DynamicImage::new_rgba8(200, 200);

        for (x, y, pixel) in d_image.as_mut_rgba8().unwrap().enumerate_pixels_mut() {
            if x % 2 == 0 {
                *pixel = image::Rgba([0, 0, 0, 255])
            } else {
                *pixel = image::Rgba([255, 255, 255, 255])
            }
        }

        state.resource.images.push(d_image);
    }

    pub fn update_waveform(&self, state: &mut WidgetState) {
        println!("update waveform");
        //let mut image = state.resource.images.first_mut().unwrap();

        let width = state.transform.get_global_width(self.id) as u32;
        let height = state.transform.get_global_height(self.id) as u32;

        let mut d_image = DynamicImage::new_rgba8(width, height);

        for (x, y, pixel) in d_image.as_mut_rgba8().unwrap().enumerate_pixels_mut() {
            if x % 2 == 0 {
                *pixel = image::Rgba([0, 0, 0, 255])
            } else {
                *pixel = image::Rgba([255, 255, 255, 255])
            }
        }

        //state.resource.images.clear();
        //state.resource.images.push(d_image);

        *state.resource.images.first_mut().unwrap() = d_image;

        println!("DONE");
    }

    pub fn get_entity(&self) -> Entity {
        self.id
    }

    // pub fn load_audio(&mut self, audio_id: u16, graph: &Graph) {
    //     let audio = &graph.audio;

    // }
}

impl EventHandler for AudioViewer {
    fn handle_event(
        &mut self,
        state: &mut WidgetState,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler>>,
        event_queue: &mut EventQueue,
    ) {
        match event {
            WidgetEvent::WidgetSizeChanged(entity, width, height) => {
                if *entity == self.id {
                    //println!("Size changed to: {} by {}", width, height);
                    //self.update_waveform(state);
                }
            }
            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.id
    }
}
