use tuix::*;

fn main() {
    let window_description = WindowDescription::new().with_title("Stack");
    let app = Application::new(window_description, |state, window|{
        // A stack is a container in which only one child is visible at a time
        let stack = Stack::new()
            .build(state, window, |builder| 
                builder.on_press(|stack: &mut Stack, state, entity|{
                    let index = if stack.get_current_index() == 0 {
                        1
                    } else {
                        0
                    };
                    stack.set_current_index(state, entity, index);
                })
            );

        let first_child = Element::new().build(state, stack, |builder| 
            builder
                .set_background_color(Color::red())
        );

        let second_child = Element::new().build(state, stack, |builder| 
            builder
                .set_background_color(Color::blue())
        );
    });

    app.run();
}