use tuix::*;

static THEME: &'static str = r#"
    temp {
        background-color: #121212;
        child-space: 1s;
    }

    textbox {
        color: #e3e3e3;
        background-color: #121212;
        border-color: #ac83df;
        border-width: 2px;
        border-radius: 3px;
        child-top: 1s;
        child-bottom: 1s;
        child-left: 5px;
        child-right: 1s;
        width: 100px;
        height: 30px;
    }

    label {
        color: #e3e3e3;
        child-top: 1s;
        child-bottom: 1s;
        child-left: 10px;
        width: 100px;
        height: 30px;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
pub enum TempEvent {
    UpdateValue(String),
}

#[derive(Default)]
struct Temp {
    temp_celcius: f32,
    textbox_celcius: Entity,
    textbox_fahrenheit: Entity,
}

impl Widget for Temp {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.textbox_celcius = Textbox::<String>::new("0")
            .on_submit(move |widget, state, textbox|{
                textbox.emit(state, entity, Event::new(TempEvent::UpdateValue(widget.text.to_owned())));
            })
            .build(state, entity, |builder| builder);

        Label::<String>::new("Celcius = ").build(state, entity, |builder| builder);

        self.textbox_fahrenheit = Textbox::<String>::new("32")
            .on_submit(move |widget, state, textbox|{
                textbox.emit(state, entity, Event::new(TempEvent::UpdateValue(widget.text.to_owned())));
            })
            .build(state, entity, |builder| builder);

        Label::<String>::new("Fahrenheit").build(state, entity, |builder| builder);
        
        entity.set_element(state, "temp").set_layout_type(state, LayoutType::Row)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(temp_event) = event.message.downcast::<TempEvent>() {
            match temp_event {
                TempEvent::UpdateValue(value) => {
                    
                    if let Ok(val) = value.parse::<f32>() {
                        if event.origin == self.textbox_celcius {
                            self.temp_celcius = val;
                        }

                        if event.origin == self.textbox_fahrenheit {
                            let celcius = (5.0/9.0) * (val - 32.0);
                            self.temp_celcius = celcius;
                        }
                    }

                    self.textbox_celcius.set_text(state, &self.temp_celcius.to_string());
                    let fahrenheit = 32.0 + (9.0/5.0) * self.temp_celcius;
                    self.textbox_fahrenheit.set_text(state, &fahrenheit.to_string());
                    
                }
            }
        }
    }
}

fn main() {

    let window_description = WindowDescription::new()
        .with_title("Temperature Converter")
        .with_inner_size(500, 100);

    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        Temp::default()
            .build(state, window, |builder| builder);
    });

    app.run();
}