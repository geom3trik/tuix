

// This example shows how drag and drop events can be utilised within tuix. This example does not demonstrate dragging of files from outside of a tuix window.

use tuix::*;


const STYLE: &str = r#"
    button {
        width: 200px;
        height: 200px;
        child-space: 1s;
        border-color: black;
    }

    button.source {
        background-color: #208020;
    }

    button.target {
        background-color: #202080;
    }

"#;

#[derive(PartialEq)]
pub enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
pub struct Container {

}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, container: Entity, event: &mut Event) {
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
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window| {

        state.add_theme(STYLE);

        let container = Container::default().build(state, window, |builder| builder);

        let row = Row::new().build(state, container, |builder| 
            builder
                .set_col_between(Stretch(1.0))
                .set_child_space(Stretch(1.0))
        );

        Button::with_label("SOURCE")
        .on_press(|data, state, button|{
            state.drag(button, Some(CustomEvent::ChangeColor(Color::red())));
        })
        .build(state, row, |builder| 
            builder
                .class("source")
        );

        Button::with_label("TARGET")
        .on_drop(|data, state, button|{
            button.set_border_radius(state, Percentage(50.0));
        })
        .on_drag_enter(|data, state, button|{
            button.set_border_width(state, Units::Pixels(2.0));
        })
        .on_drag_leave(|data, state, button|{
            button.set_border_width(state, Units::Pixels(0.0));
        })
        .build(state, row, |builder| 
            builder
                .class("target")
        );
    });

    app.run();
}