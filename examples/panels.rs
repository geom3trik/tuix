use tuix::*;

const STYLE: &str = r#"
    
    *:focus {
        border-width: 1px;
        border-color: black;
    }
    

    panel>.header {
        background-color: #ff5e1a;
    }

    panel .container1 {
        background-color: white;
    }

    panel .container2 {
        padding: 10px;
        child_left: 1s;
        child_right: 1s;
        child_top: 10px;
        child_bottom: 10px;
        child_between: 10px;
    }

    button {
        background-color: #ff5e1a;
    }

    button:hover {
        background-color: #ff7033;
    }
    
    panel.one {
        left: 10px;
        top: 10px;
        width: 300px;
        flex-direction: column;
        background-color: cyan;
    }

    panel.one>.header {
        height: 30px;
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
    let app = Application::new(WindowDescription::new().with_title("Panels"), |state, window| {
        state.add_theme(STYLE);

        let panel =
            Panel::new("Panel 1").build(state, window.entity(), |builder| builder.class("one"));

        Button::with_label("1").build(state, panel, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_background_color(Color::blue())
                .set_child_space(Stretch(1.0))
        });

        Button::with_label("2").build(state, panel, |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_background_color(Color::blue())
                .set_child_space(Stretch(1.0))
        });

        // let panel = Panel::new("Panel 2").build(state, window.entity(), |builder| builder.class("two"));

        // Button::with_label("Button").build(state, panel, |builder| {
        //     builder
        //         .set_width(Units::Pixels(100.0))
        //         .set_height(Units::Pixels(30.0))
        //         .set_text_justify(Justify::Center)
        // });
    });

    app.run();
}
