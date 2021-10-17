use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"

    list radio {
        left: 10px;
        top: 1s;
        bottom: 1s;
    }

    radio {
        border-color: black;
        border-width: 1px;
        width: 16px;
        height: 16px;
        border-radius: 8px;
        child-space: 1s;
    }
    
    radio>.marker {
        background-color: black;
        width: 10px;
        height: 10px;
        border-radius: 5px;
    }
    
    radio>.marker {
        visibility: hidden;
        display: none;
    }
    
    radio:checked {
        border-color: black;
    }
    
    radio:checked>.marker {
        visibility: visible;
        display: flex;
    }

    label {
        child-space: 1s;
        child-left: 10px;
        color: black;
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
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.listbox = List::new()
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Auto)
                    .set_space(Stretch(1.0))
            });

        let row = Row::new().build(state, self.listbox, |builder| 
            builder
                .set_height(Pixels(30.0))
        );
        
        Radio::new()
            .set_checked(true)
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(200, 50, 50)));
            })
            .build(state, row, |builder| 
                builder
                    .set_color(Color::black())
            );
        
        Label::new("Red").build(state, row, |builder| 
            builder
        );

        let row = Row::new().build(state, self.listbox, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Radio::new()
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 200, 50)));
            })
            .build(state, row, |builder| 
                builder
                    .set_color(Color::black())
            );

        Label::new("Green").build(state, row, |builder| 
            builder
        );

        let row = Row::new().build(state, self.listbox, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Radio::new()
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 50, 200)));
            })
            .build(state, row, |builder| 
                builder
                    .set_color(Color::black())
            );

        Label::new("Blue").build(state, row, |builder| 
            builder
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
            .with_title("Radio List")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white()).set_focusable(state, false);

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}