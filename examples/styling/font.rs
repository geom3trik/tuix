// Demonstrates loading a custom font, adding it to the application, and then using it from a stylesheet.

use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    .custom {
        background-color: #CCCCCC;
        child-space: 1s;
        child-left: 0px;
        font: "fira";
    }
"#;

static CUSTOM_FONT: &[u8] = include_bytes!("../../resources/FiraCode-Regular.ttf");

fn main() {
    let window_description = WindowDescription::new().with_title("Font");
    let app = Application::new(window_description, |state, window|{
        state.add_font_mem("fira", CUSTOM_FONT);


        state.add_theme(STYLE);

        Element::new().build(state, window, |builder|
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_text("Something123")
                //.set_font("fira")
                .class("custom")
        );
    });

    app.run();
}