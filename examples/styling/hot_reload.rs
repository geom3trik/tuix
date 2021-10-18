// Demonstrates loading a custom font, adding it to the application, and then using it from a stylesheet.

use tuix::*;
use tuix::widgets::*;


fn main() {
    let window_description = WindowDescription::new().with_title("Font");
    let app = Application::new(window_description, |state, window|{


        state.add_stylesheet("examples/styling/stylesheet.css");

        Element::new().build(state, window, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_text("Something123")
                .class("custom")
        );
    });

    app.run();
}