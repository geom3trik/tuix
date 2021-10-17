use tuix::*;
use tuix::widgets::{Button, Column};

fn main() {
    let window_description = WindowDescription::new().with_title("Button");
    Application::new(window_description, |state, window|{
        let column = Column::new().build(state, window, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_row_between(Pixels(10.0))
        );

        Button::with_label("Push Button").build(state, column, |builder| builder);
        Button::with_label("Disabled Button").build(state, column, |builder| 
            builder
                .set_width(Pixels(150.0))
                .set_disabled(true)
                .set_hoverable(false)
        );

    }).run();
}