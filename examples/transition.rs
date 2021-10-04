use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    button {
        background-color: red;
    }

    button:hover {
        background-color: blue;
        transition: background-color 1 0;
    }
"#;

fn main() {
    let app = Application::new(WindowDescription::new(), |state, window|{

        state.add_theme(STYLE);

        Button::new().build(state, window, |builder|
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(100.0))
        );
    });
}