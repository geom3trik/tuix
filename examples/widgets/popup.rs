use tuix::*;

const STYLE: &str = r#"

    popup {
        background-color: #d2d2d2;
    }

    popup check_button {
        height: 30px;
        child-space: 1s;
        color: black;
        background-color: #d2d2d2;
    }

    popup check_button:hover {
        background-color: #e2e2e2;
    }

    popup check_button:active {
        background-color: #c2c2c2;
    }

    popup check_button:checked {
        background-color: #c2c2FF;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    popup: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        Label::new("Right click for popup.").build(state, container, |builder| 
            builder
                .set_color(Color::black())
                .set_left(Pixels(10.0))
                .set_top(Pixels(10.0))
        );

        self.popup = Popup::new()
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Auto)
            });

        let list = List::new().build(state, self.popup, |builder| 
            builder
                .set_height(Auto)
        );

        // Spacer
        Element::new().build(state, list, |builder| 
            builder
                .set_height(Pixels(5.0))
        );

        CheckButton::with_label("Red")
            .set_checked(true)
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(200, 50, 50)));
            })
            .build(state, list, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Green")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 200, 50)));
            })
            .build(state, list, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Blue")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 50, 200)));
            })
            .build(state, list, |builder| 
                builder
                    .set_color(Color::black())
            );

        // Button::with_label("Red")
        //     .on_release(|_, state, button| {
        //         button.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 0, 0)));
        //     })
        //     .build(state, list, |builder| 
        //         builder
        // );

        // Button::with_label("Green")
        //     .on_release(|_, state, button| {
        //         button.emit(state, CustomEvent::ChangeColor(Color::rgb(0, 255, 0)));
        //     })
        //     .build(state, list, |builder| 
        //         builder
        // );

        // Button::with_label("Blue")
        //     .on_release(|_, state, button| {
        //         button.emit(state, CustomEvent::ChangeColor(Color::rgb(0, 0, 255)));
        //     })
        //     .build(state, list, |builder| 
        //         builder
        // );

        // Button::with_label("Yellow")
        //     .on_release(|_, state, button| {
        //         button.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 255, 0)));
        //     })
        //     .build(state, list, |builder| 
        //         builder
        // );

        // Button::with_label("Fuchsia")
        //     .on_release(|_, state, button| {
        //         button.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 0, 255)));
        //     })
        //     .build(state, list, |builder| 
        //         builder
        // );

        // Button::with_label("Aqua")
        //     .on_release(|_, state, button| {
        //         button.emit(state, CustomEvent::ChangeColor(Color::rgb(0, 255, 255)));
        //     })
        //     .build(state, list, |builder| 
        //         builder
        // );

        // // Spacer
        // Element::new().build(state, self.popup, |builder| 
        //     builder
        //         .set_height(Pixels(5.0))
        // );

        container.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, container: Entity, event: &mut Event) {
        
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseUp(button) if *button == MouseButton::Right => {
                    container.emit_to(state, self.popup, PopupEvent::OpenAtCursor);
                }

                _=> {}
            }
        }
        
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    container.set_background_color(state, *color);
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Popup")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}