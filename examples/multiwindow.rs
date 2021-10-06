use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    button {
        background-color: red;
    }

    button:hover {
        background-color: blue;
    }
"#;

enum AppEvent {
    SpawnWindow,
}

#[derive(Default)]
struct AppWidget {

}

impl Widget for AppWidget {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SpawnWindow => {
                    let window = Window::new(WindowDescription::new().with_title("Child Window")).build(state, entity, |builder| builder);
                    Button::new()

                    .build(state, window, |builder|
                        builder
                            .set_width(Pixels(100.0))
                            .set_height(Pixels(30.0))
                            .set_child_space(Stretch(1.0))
                            //.set_background_color(Color::blue())
                    );
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(WindowDescription::new(), |state, window|{
        
        state.add_theme(STYLE);
        
        let app_widget = AppWidget::default().build(state, window, |builder| builder);
        Button::with_label("Spawn Window")
        .on_press(|_, state, button|{
            button.emit(state, AppEvent::SpawnWindow);
        })
        .build(state, app_widget, |builder|
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                //.set_background_color(Color::red())
        );
    });

    app.run();
}