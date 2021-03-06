use tuix::*;

const STYLE: &str = r#"

    panel>.header {
        background-color: #ff5e1a;
    }

    panel .container1 {
        background-color: white;
    }

    panel .container2 {
        padding: 10px;
        align-items: center;
    }

    button {
        background-color: #ff5e1a;
    }

    button:hover {
        background-color: #ff7033;
    }

    panel.one {
        margin: 10px;
        width: 300px;
        flex-direction: column;
    }

    panel.one>.header {
        flex-basis: 30px;
        flex-direction: row;
    }

    panel.two {
        margin: 10px;
        height: 100px;
        flex-direction: row;
    }

    panel.two>.header {
        flex-basis: 80px;
        flex-direction: column;
    }

    panel.two>.header>label {
        text-align: start;
        text-justify: center;
    }
"#;

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.add_theme(STYLE);

        let panel = Panel::new("Panel").build(state, window, |builder| 
            builder.class("one")
        );

        Button::with_label("Button").build(state, panel, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::from("#ff5e1a"))
                .set_text_justify(Justify::Center)
        });

        let panel = Panel::new("Panel").build(state, window, |builder| 
            builder.class("two")
        );

        Button::with_label("Button").build(state, panel, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::from("#ff5e1a"))
                .set_text_justify(Justify::Center)
        });


        win_desc.with_title("Panel")
    });

    app.run();
}
