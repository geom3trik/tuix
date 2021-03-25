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
                .set_flex_basis(Length::Pixels(30.0))
                .set_text_justify(Justify::Center)
        });

        Label::new("Gallery").build(state, entity, |builder| {
            builder.set_flex_grow(1.0).set_text_justify(Justify::Center)
        });

        Button::with_label("S").build(state, entity, |builder| {
            builder
                .set_flex_basis(Length::Pixels(30.0))
                .set_text_justify(Justify::Center)
        });

        entity
            .set_flex_basis(state, Length::Pixels(30.0))
            .set_flex_direction(state, FlexDirection::Row)
            .set_background_color(state, Color::rgb(255, 94, 26))
    }
}


fn main() {
    let app = Application::new(|window, state, root| {
        state.add_theme(DEFAULT_THEME);

        window.set_background_color(state, Color::white());

        (tab_bar, tab_view) = TabManager::new().build(state, root, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_flex_basis(Length::Pixels(30.0))
        );

        Header::new().build(state, window, |builder| builder);

        let container = Element::new().build(state, window, |builder| {
            builder
                .set_flex_grow(1.0)
                .set_align_items(AlignItems::Center)
                .set_justify_content(JustifyContent::Center)
        });

        // Button::with_label("Button").build(state, container, |builder|
        //     builder
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(50.0))
        // );

        Checkbox2::new("First", true).build(state, container, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
        });
        Checkbox2::new("Second", true).build(state, container, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
        });
        Checkbox2::new("Third", true).build(state, container, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
        });

        win_desc.with_title("Widget Gallery")
    });

    app.run();
}
