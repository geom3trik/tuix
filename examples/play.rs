



extern crate tuix;

use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::*;

fn main() {
    let app = Application::new(|win_desc, state, window| {

        //state.insert_theme(DEFAULT_THEME);
        state.insert_stylesheet("examples/themes/play_theme.css");

        let row1 = HBox::new().build(state, window, |builder| builder);

        Label::new("Playhead:").build(state, row1, |builder| builder);
        Textbox::new("12").build(state, row1, |builder| builder);

        let row2 = HBox::new().build(state, window, |builder| builder);

        Button::new().build(state, row2, |builder| {
            builder.set_text("Play")
        });

        Button::new().build(state, row2, |builder| {
            builder.set_text("Stop")
        });

        win_desc.with_title("DAW").with_inner_size(300, 120)
    });

    app.run();
}
