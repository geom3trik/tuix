/*
use tuix::style::themes::DEFAULT_THEME;
use tuix::*;
pub struct Header {}

impl Header {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Header {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Button::with_label("M").build(state, entity, |builder| {
            builder
                .set_flex_basis(Units::Pixels(30.0))
                .set_text_justify(Justify::Center)
        });

        Label::new("Gallery").build(state, entity, |builder| {
            builder.set_flex_grow(1.0).set_text_justify(Justify::Center)
        });

        Button::with_label("S").build(state, entity, |builder| {
            builder
                .set_flex_basis(Units::Pixels(30.0))
                .set_text_justify(Justify::Center)
        });

        entity
            .set_flex_basis(state, Units::Pixels(30.0))
            .set_flex_direction(state, FlexDirection::Row)
            .set_background_color(state, Color::rgb(255, 94, 26))
    }
}

fn main() {
    let app = Application::new(|state, window| {
        state.add_theme(DEFAULT_THEME);

        window
            .set_title("Widget Gallery")
            .set_background_color(state, Color::white());

        let (tab_bar, tab_view) = TabManager::new().build(state, window.entity(), |builder| {
            builder
                .set_flex_grow(1.0)
                .set_flex_basis(Units::Pixels(30.0))
        });

        Header::new().build(state, window.entity(), |builder| builder);

        let container = Element::new().build(state, window.entity(), |builder| {
            builder
                .set_flex_grow(1.0)
                .set_align_items(AlignItems::Center)
                .set_justify_content(JustifyContent::Center)
        });

        // Button::with_label("Button").build(state, container, |builder|
        //     builder
        //         .set_width(Units::Pixels(100.0))
        //         .set_height(Units::Pixels(50.0))
        // );

        CheckItem::new("First", true).build(state, container, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
        });
        CheckItem::new("Second", true).build(state, container, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
        });
        CheckItem::new("Third", true).build(state, container, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
        });
    });

    app.run();
}
*/

fn main() {
    
}