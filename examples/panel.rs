use tuix::*;

const STYLE: &str = r#"
    panel>.header {
        flex-basis: 30px;
        background-color: red;
    }

    panel>.container1 {
        background-color: white;
    }
"#;

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.add_theme(STYLE);

        let panel = Panel::new("Panel").build(state, window, |builder| 
            builder
                .set_width(Length::Pixels(300.0))
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
