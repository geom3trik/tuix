use tuix::*;

fn main() {
    let app = Application::new(|state, window| {

        // Set the window title
        window.set_title("Shared Styles");

        // Create a shared style rule for button elements
        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button"))
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(30.0))
            .set_background_color(Color::from("#ff5e1a"))
            .set_text_justify(Justify::Center)
            .set_margin(Length::Pixels(5.0));

        // Create a shared style rule for hovered buttons
        let hover_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button").set_hover())
            .set_background_color(Color::from("#ff701a"));

        // Create a shared style rule for hovered buttons
        let active_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button").set_active())
            .set_background_color(Color::from("#ee5a1d"));


        // Add the shared style rule to state
        state.add_style_rule(style_rule);
        state.add_style_rule(hover_rule);
        state.add_style_rule(active_rule);


        // Add first button
        Button::with_label("Button 1").build(state, window.entity(), |builder| builder);

        // Add second button
        Button::with_label("Button 2").build(state, window.entity(), |builder| builder);

        
    });

    app.run();
}
