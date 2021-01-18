extern crate tuix;

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        Button::new().build(state, window, |builder| {
            builder.set_text("Button")
        });

        win_desc.with_title("Hello GUI")
    });

    app.run();
}
