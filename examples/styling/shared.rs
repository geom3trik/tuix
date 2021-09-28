use tuix::*;
use tuix::widgets::*;
// Two buttons which share the same style

fn main() {
    let app = Application::new(
        WindowDescription::new().with_title("Shared Styles"),
        |state, window| {

        // Create a shared style rule for button elements
        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::element("button"))
            .set_width(Pixels(100.0))
            .set_height(Pixels(30.0))
            .set_background_color(Color::from("#ff5e1a"))
            .set_child_space(Stretch(1.0))
            .set_space(Pixels(5.0));

        // Add the shared style rule to state
        state.add_style_rule(style_rule);

        // Add first button
        Button::with_label("Button 1").build(state, window.entity(), |builder| builder);

        // Add second button
        Button::with_label("Button 2").build(state, window.entity(), |builder| builder);
    });

    app.run();
}
