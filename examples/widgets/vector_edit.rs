use tuix::*;
use tuix::style::themes::DEFAULT_THEME;

const STYLE: &str = r#"


    vector_edit>textbox {
        width: 1s;
        min-width: 0px;
        background-color: white;
        child-space: 1s;
        color: black;
        border-width: 1px;
        border-color: #757575;
    }

    vector_edit>dropdown {
        color: black;
        background-color: #d2d2d2;
    }

    dropdown>.container {
        top: 100%;
        width: 100%;
        border-width: 1px;
        border-color: #757575;
        outer-shadow: 2px 2px 5px #80000000;
    }

    dropdown>.header>.label {
        color: black;
        child-space: 1s;
    }

    vector_edit>dropdown .item {
        color: black;
        background-color: white;
        height: 30px;
    }

    vector_edit>dropdown .item:hover {
        background-color: #f2f2f2;
        height: 30px;
    }

    vector_edit .icon {
        display: none;
    }

"#;

fn main() {
    let app = Application::new(
        WindowDescription::new()
            .with_title("Vector Edit")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            VectorEdit::new()
                .with_x(0.0)
                .with_y(0.0)
                .with_z(0.0)
                .with_w(0.0)
                .build(state, window.entity(), |builder| {
                    builder
                        .set_width(Pixels(210.0))
                        .set_height(Pixels(30.0))
                        .set_space(Stretch(1.0))
                });
        },
    );

    app.run();
}