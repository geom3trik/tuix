use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.add_theme(DEFAULT_THEME);
        let debug_container = DebugContainer::new().build(state, window, |builder| builder);

        Button::with_label("Button").build(state, debug_container, |builder| {
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
