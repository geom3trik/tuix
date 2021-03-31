use tuix::*;



fn main() {
    let app = Application::new(|state, window| {

        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button"))
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(30.0))
            .set_background_color(Color::from("#ff5e1a"))
            .set_margin(Length::Pixels(5.0));


        state.add_style_rule(style_rule);

        window.set_title("Hello GUI");

        Button::with_label("Button 1").build(state, window.entity(), |context| {
            context
                .set_text_justify(Justify::Center)
        });

        Button::with_label("Button 2").build(state, window.entity(), |context| {
            context
                .set_text_justify(Justify::Center)
        });

        
    });

    app.run();
}
