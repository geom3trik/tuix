use tuix::*;



fn main() {
    let app = Application::new(|state, window| {

        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button"))
            .property(Property::Width(Length::Pixels(100.0)))
            .property(Property::Height(Length::Pixels(30.0)))
            .property(Property::BackgroundColor(Color::from("#ff5e1a")))
            .property(Property::Margin(Length::Pixels(5.0)));


        state.add_style_rule(style_rule);

        Button::with_label("Button 1").build(state, window, |builder| {
            builder
                .set_text_justify(Justify::Center)
        });

        Button::with_label("Button 2").build(state, window, |builder| {
            builder
                .set_text_justify(Justify::Center)
        });

        win_desc.with_title("Hello GUI")
    });

    app.run();
}
