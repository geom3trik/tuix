use tuix::*;
use tuix::widgets::*;


fn main() {
    let app = Application::new(
        WindowDescription::new().with_title("Hover Style"), 
        |state, window| {

        // Create a shared style rule for button elements
        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button"))
            .set_width(Units::Pixels(100.0))
            .set_height(Units::Pixels(30.0))
            .set_background_color(Color::from("#ff5e1a"))
            .set_child_space(Stretch(1.0))
            .set_space(Pixels(5.0));

        // Create a shared style rule for hovered buttons
        let hover_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button").set(PseudoClass::HOVER))
            .set_background_color(Color::from("#ff701a"));

        // Create a shared style rule for hovered buttons
        let active_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button").set(PseudoClass::ACTIVE))
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
