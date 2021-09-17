use tuix::*;

const STYLE: &str = r#"

    textbox {
        border-width: 1px;
        border-color: #555555;
        color: black;
    }

    dropdown {
        border-width: 1px;
        border-color: #555555;
        background-color: white;
    }
    
    dropdown .label {
        child-space: 1s;
        color: black;
    }

    dropdown .icon {
        color: #555555;
    }

    popup {
        background-color: #d2d2d2;
    }


    list {
        border-width: 1px;
        border-color: #555555;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        background-color: #d2d2d2;
    }

    list>check_button:hover {
        background-color: #e2e2e2;
    }

    list>check_button:active {
        background-color: #c2c2c2;
    }

    list>check_button:checked {
        background-color: #c2c2ff;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }

    slider>.track {
        background-color: #dfdfdf;
        border-radius: 2px;
    }

    slider>.track>.active {
        background-color: #f74c00;
        border-radius: 2px;
    }

    slider>.thumb {
        background-color: white;
        width: 0px;
        height: 1s;
    }
    
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    length_box: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.length_box = LengthBox::new("Border Width")
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(250.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });
            
        container.set_background_color(state, Color::white()).set_focusable(state, false)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    entity.set_background_color(state, *color);

                    event.consume();
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Length Box")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white()).set_focusable(state, false);

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}