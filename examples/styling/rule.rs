use tuix::*;

const style_rule: StyleRule = StyleRule::new().selector(Selector::new().element("button")).property(Property::Width(Length::Pixels(200.0)));

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.add_style_rule(style_rule);

        Button::with_label("Button")
        .build(state, window, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::from("#ff5e1a"))
                .set_text_justify(Justify::Center)
        });

        win_desc.with_title("Hello GUI")
    });

    app.run();
}