use tuix::*;

const STYLE: &str = r#"
    list {
        border-width: 1px;
        border-color: #555555;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        background-color: white;
    }

    list>check_button:hover {
        background-color: #e2e2e2;
    }

    list>check_button:active {
        background-color: #c2c2c2;
    }

    list>check_button:checked {
        background-color: #AAAAFF;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    listbox: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data<'a> = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        let scroll = ScrollContainer::new().build(state, container, |builder| builder);

        self.listbox = List::new()
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Auto)
                    .set_space(Stretch(1.0))
            });
        
        CheckButton::with_label("Red")
            .set_checked(true)
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(200, 50, 50)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Green")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 200, 50)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Blue")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 50, 200)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

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
            .with_title("Spinbox")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white()).set_focusable(state, false);

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}