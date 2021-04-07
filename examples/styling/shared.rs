use tuix::*;



fn main() {
    let app = Application::new(|state, window| {

        // Set the window title
        window.set_title("Shared Styles");

        // Create a shared style rule for button elements
        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button"))
            .set_width(Units::Pixels(100.0))
            .set_height(Units::Pixels(30.0))
            .set_background_color(Color::from("#ff5e1a"))
            .set_text_justify(Justify::Center)
            .set_margin(Units::Pixels(5.0));

        // Add the shared style rule to state
        state.add_style_rule(style_rule);

        // Add first button
        Button::with_label("Button 1").build(state, window.entity(), |builder| builder);

        // Add second button
        Button::with_label("Button 2").build(state, window.entity(), |builder| builder);

        
    });

    app.run();
}
