use tuix::*;


const STYLE: &str = r#"
    button {
        margin: 10px;
    }
"#;

fn main() {
    let app = Application::new(|state, window| {
        
        window.set_title("Hello GUI");

        //state.style.layout_type.insert(window.entity(), LayoutType::Vertical);

        state.add_theme(STYLE);

        Button::with_label("Button")
            .build(state, window.entity(), |builder| {
                builder
                    //.set_main_size(Units::Pixels(30.0))
                    //.set_cross_size(Units::Pixels(100.0))
                    //.set_main_before(Units::Pixels(0.0))
                    //.set_cross_before(Units::Pixels(0.0))
                    .set_margin(Units::Pixels(10.0))
                    .set_width(Units::Pixels(100.0))
                    .set_height(Units::Pixels(30.0))
                    .set_background_color(Color::from("#ff5e1a"))
                    .set_text_justify(Justify::Center)
        });
    });
    app.run();
}