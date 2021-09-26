

// This example shows how drag and drop events can be utilised within tuix. This example does not demonstrate dragging of files from outside of a tuix window.

use tuix::*;


const STYLE: &str = r#"
    button {
        width: 200px;
        height: 200px;
        child-space: 1s;
    }

    button.source {
        background-color: #208020;
    }

    button.target {
        background-color: #202080;
    }

"#;

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window| {

        state.add_theme(STYLE);

        let row = Row::new().build(state, window, |builder| 
            builder
                .set_col_between(Stretch(1.0))
                .set_child_space(Stretch(1.0))
        );

        Button::with_label("SOURCE")
        .on_press(|data, state, button|{
            state.drag(button);
        })
        .build(state, row, |builder| 
            builder
                .class("source")
        );

        Button::with_label("TARGET")
        .on_drop(|data, state, button|{
            button.set_border_radius(state, Percentage(50.0));
        })
        .build(state, row, |builder| 
            builder
                .class("target")
        );
    });

    app.run();
}