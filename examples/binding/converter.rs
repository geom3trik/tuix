use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
textbox {
    border-color: #9e9e9e;
    color: black;
    border-width: 1px;
    width: 100px;
    height: 30px;
    border-radius: 3px;
    child-space: 1s;
    child-left: 5px;
    top: 1s;
    bottom: 1s;
}

textbox:hover {
    border-color: black;
}

label {
    width: 50px;
    height: 30px;
    child-space: 1s;
    child-left: 5px;
    top: 1s;
    bottom: 1s;
    left: 0px;
    right: 1s;
    color: black;
}
"#;

#[derive(Debug, Clone, PartialEq)]
enum AppEvent {
    SetCelcius(f32),
    SetFahrenheit(f32),
}

#[derive(Default, Lens)]
pub struct AppData {
    pub temperature_celcius: f32,
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                AppEvent::SetCelcius(temp) => {
                    self.temperature_celcius = *temp;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetFahrenheit(temp) => {
                    self.temperature_celcius = (*temp - 32.0) * 5.0/9.0; 
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}


fn main() {
    let window_description = WindowDescription::new().with_title("Temperature Converter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window|{
        
        state.add_theme(STYLE);
        
        let app_data = AppData::default().build(state, window);

        let row = Row::new().build(state, app_data, |builder| 
            builder
                .set_child_space(Stretch(1.0))
        );

        Textbox::new("test")
            .on_submit(|data, state, textbox|{
                if let Ok(temp) = data.text.parse::<f32>() {
                    textbox.emit(state, AppEvent::SetCelcius(temp));
                }
            })
            .bind(AppData::temperature_celcius, |temp| temp.to_string())
            .build(state, row, |builder| builder);
        
        Label::new("°C").build(state, row, |builder| builder);

        Textbox::new("test")
            .on_submit(|data, state, textbox|{
                if let Ok(temp) = data.text.parse::<f32>() {
                    //let temp_celcius = (temp - 32.0) * 5.0/9.0;
                    textbox.emit(state, AppEvent::SetFahrenheit(temp));
                }
            })
            .bind(AppData::temperature_celcius, |temp| ((*temp * 9.0/5.0) + 32.0).to_string())
            .build(state, row, |builder| builder);

        Label::new("°F").build(state, row, |builder| builder);

    });

    app.run();
} 