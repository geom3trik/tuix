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
use crate::widget::audio_viewer::AudioViewer;
use crate::widget::button::Button;
use crate::widget::checkbox::Checkbox;
use crate::widget::intbox::IntBox;
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};
use crate::widget::slider::Slider;

const ICON_TO_START: &str = "\u{23ee}";
const ICON_TO_END: &str = "\u{23ed}";
const ICON_STOP: &str = "\u{25a0}";
const ICON_PLAY: &str = "\u{25b6}";
const ICON_RECORD: &str = "\u{26ab}";

pub struct TrackView {
    id: Entity,
}

impl TrackView {
    pub fn new(state: &mut WidgetState, widget_list: &mut WidgetList, parent: Entity) -> Self {
        let id = state.add(parent).unwrap();
        id.set_display(state, Display::Flexbox)
            .set_background(state, nanovg::Color::from_rgb(38, 38, 38));

        let main_area = state.add(id).unwrap();
        main_area
            .set_flex_grow(state, 5.0)
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column)
            .set_background(state, nanovg::Color::from_rgb(38, 38, 38));

        //let side_bar = state.add(id).unwrap();

        let toolbar = state.add(main_area).unwrap();
        toolbar
            .set_flex_grow(state, 0.0)
            .set_flex_basis(state, 50.0)
            .set_display(state, Display::Flexbox)
            .set_justify_content(state, JustifyContent::Center)
            .set_background(state, nanovg::Color::from_rgb(56, 56, 56))
            .set_margin_bottom(state, 1.0);

        let mut viewer = AudioViewer::new(state, widget_list, main_area);
        //viewer.create_waveform(state);

        widget_list.push(viewer);

        let to_start_button = Button::new(state, toolbar);
        to_start_button
            .get_entity()
            .set_flex_basis(state, 50.0)
            .set_flex_grow(state, 0.0)
            .set_font(state, "Icons".to_string())
            .set_text(state, ICON_TO_START.to_string())
            .set_text_horizontal_align(state, HorizontalAlign::Center)
            .set_font_size(state, 40.0);

        widget_list.push(to_start_button);

        let play_button = Button::new(state, toolbar);
        play_button
            .get_entity()
            .set_flex_basis(state, 50.0)
            .set_flex_grow(state, 0.0)
            .set_font(state, "Icons".to_string())
            .set_text(state, ICON_PLAY.to_string())
            .set_text_horizontal_align(state, HorizontalAlign::Center)
            .set_font_size(state, 40.0);

        widget_list.push(play_button);

        let stop_button = Button::new(state, toolbar);
        stop_button
            .get_entity()
            .set_flex_basis(state, 50.0)
            .set_flex_grow(state, 0.0)
            .set_font(state, "Icons".to_string())
            .set_text(state, ICON_STOP.to_string())
            .set_text_horizontal_align(state, HorizontalAlign::Center)
            .set_font_size(state, 40.0);

        widget_list.push(stop_button);

        let to_end_button = Button::new(state, toolbar);
        to_end_button
            .get_entity()
            .set_flex_basis(state, 50.0)
            .set_flex_grow(state, 0.0)
            .set_font(state, "Icons".to_string())
            .set_text(state, ICON_TO_END.to_string())
            .set_text_horizontal_align(state, HorizontalAlign::Center)
            .set_font_size(state, 40.0);

        widget_list.push(to_end_button);

        let record_button = Button::new(state, toolbar);
        record_button
            .get_entity()
            .set_flex_basis(state, 50.0)
            .set_flex_grow(state, 0.0)
            .set_font(state, "Icons".to_string())
            .set_text(state, ICON_RECORD.to_string())
            .set_text_horizontal_align(state, HorizontalAlign::Center)
            .set_font_size(state, 40.0);

        widget_list.push(record_button);

        TrackView { id: id }
    }

    pub fn get_entity(&self) -> Entity {
        self.id
    }
}
