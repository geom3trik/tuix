use tuix::*;

const STYLE: &str = r#"
    
    panel>.header {
        background-color: #ff5e1a;
        height: 30px;
    }

    panel .container1 {
        background-color: white;
    }

    panel .container2 {
        child-left: 1s;
        child-right: 1s;
        child-top: 10px;
        child-bottom: 10px;
        col-between: 10px;
    }
    
    panel.one {
        left: 10px;
        top: 10px;
        width: 300px;
        height: auto;
        background-color: cyan;
    }

    panel>.header>label {
        child-space: 1s;
        child-left: 0px;
    }
    
"#;

fn main() {
    let app = Application::new(WindowDescription::new().with_title("Panels"), |state, window| {
        state.add_theme(STYLE);

        let panel =
            Panel::new("Panel 1").build(state, window.entity(), |builder| builder.class("one"));


        panel.set_row_between(state, Pixels(20.0));

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
                .set_background_color(Color::red())
                .set_child_space(Stretch(1.0))
        });

        // let panel = Panel::new("Panel 2").build(state, window.entity(), |builder| builder.class("two"));

        // Button::with_label("Button").build(state, panel, |builder| {
        //     builder
        //         .set_width(Units::Pixels(100.0))
        //         .set_height(Units::Pixels(30.0))
        //         .set_child_space(Stretch(1.0))
        // });
    });

    app.run();
}
