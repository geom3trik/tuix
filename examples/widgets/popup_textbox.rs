use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    popup {
        background-color: #d2d2d2;
    }
    popup textbox {
        child-space: 1s;
        color: black;
        background-color: #d2d2d2;
    }

    scroll_container>.scrollbar {
        background-color: #464646;
        width: 10px;
    }
    scroll_container:enabled>.scrollbar {
        width: 10px;
        transition: width 0.1 0.0;
    }
    scroll_container:disabled>.scrollbar {
        width: 0px;
        transition: width 0.1 0.0;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    popup: Entity,
    textbox: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
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
                    .set_height(Pixels(200.0))
            });

        // let scroll = ScrollContainer::new()
        // .build(state, self.popup, |builder| 
        //     builder
        //         //.set_top(Stretch(1.0))
        //         //.set_bottom(Stretch(1.0))
        //         //.set_space(Stretch(1.0))
        // );

        // let list = List::new().build(state, self.popup, |builder| 
        //     builder
        // );

        let list = Column::new().build(state, self.popup, |builder| builder);

        // Spacer
        Element::new().build(state, list, |builder| 
            builder
                .set_height(Pixels(5.0))
        );

        self.textbox = Textbox::new("Red")
            // .on_release(|_, state, button| {
            //     button.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 0, 0)));
            //     button.emit(state, PopupEvent::Close);
            // })
            .build(state, list, |builder| 
                builder
        );

        Textbox::new("Green")
            // .on_release(|_, state, button| {
            //     button.emit(state, CustomEvent::ChangeColor(Color::rgb(0, 255, 0)));
            //     button.emit(state, PopupEvent::Close);
            // })
            .build(state, list, |builder| 
                builder
        );

        Textbox::new("Blue")
            // .on_release(|_, state, button| {
            //     button.emit(state, CustomEvent::ChangeColor(Color::rgb(0, 0, 255)));
            //     button.emit(state, PopupEvent::Close);
            // })
            .build(state, list, |builder| 
                builder
        );

        Textbox::new("Yellow")
            // .on_release(|_, state, button| {
            //     button.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 255, 0)));
            //     button.emit(state, PopupEvent::Close);
            // })
            .build(state, list, |builder| 
                builder
        );

        Textbox::new("Fuchsia")
            // .on_release(|_, state, button| {
            //     button.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 0, 255)));
            //     button.emit(state, PopupEvent::Close);
            // })
            .build(state, list, |builder| 
                builder
        );

        Textbox::new("Aqua")
            // .on_release(|_, state, button| {
            //     button.emit(state, CustomEvent::ChangeColor(Color::rgb(0, 255, 255)));
            //     button.emit(state, PopupEvent::Close);
            // })
            .build(state, list, |builder| 
                builder
        );

        // Spacer
        Element::new().build(state, list, |builder| 
            builder
                .set_height(Pixels(5.0))
        );

        container.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, container: Entity, event: &mut Event) {
        
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseUp(button) if *button == MouseButton::Right => {
                    container.emit_to(state, self.popup, PopupEvent::OpenAtCursor);
                    self.textbox.emit(state, TextboxEvent::BeginEdit);
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