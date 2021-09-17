use tuix::*;

#[derive(Lens)]
pub struct AppData {
    value: i32,
}

#[derive(PartialEq)]
pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Increment => {
                    self.value += 1;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::Decrement => {
                    self.value -= 1;
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(WindowDescription::new(), |state, window|{

        let app_data = AppData{value: 30}.build(state, window);

        Button::with_label("Increment")
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::Increment);
            })
            .build(state, app_data, |builder|
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50,50,50))
                    .set_space(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
            );

        Button::with_label("Decrement")
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::Decrement);
            })
            .build(state, app_data, |builder|
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50,50,50))
                    .set_space(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
            );

        Label::new("")
            .bind(AppData::value, |value| value.to_string())
            .build(state, app_data, |builder|
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(100.0))
                    .set_space(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
            );
    });

    app.run();    
}