use tuix::*;

static THEME: &'static str = r#"
    temp {
        background-color: #DDDDDD;
        child-space: 1s;
    }

    textbox {
        color: black;
        background-color: white;
        border-color: #666666;
        border-width: 1px;
        child-top: 1s;
        child-bottom: 1s;
        child-left: 5px;
        child-right: 1s;
        width: 100px;
        height: 30px;
    }

    label {
        color: black;
        child-top: 1s;
        child-bottom: 1s;
        child-left: 5px;
        width: 100px;
        height: 30px;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
pub enum DataEvent {
    Changed(std::rc::Rc<dyn Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TempEvent {
    UpdateCelcius(String),
    UpdateFahrenheit(String),
}

#[derive(Default)]
struct TempState {
    temp_celcius: f32,
}

impl Node for TempState {
    fn on_mutate(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(temp_event) = event.message.downcast() {
            match temp_event {
                TempEvent::UpdateCelcius(value) => {  
                    if let Ok(val) = value.parse::<f32>() {
                        self.temp_celcius = val;
                    } else {
                        self.temp_celcius = 0.0;
                    }
                    state.insert_event(Event::new(DataEvent::Changed(Rc::new(self))));
                }

                TempEvent::UpdateFahrenheit(value) => {
                    if let Ok(val) = value.parse::<f32>() {
                        self.temp_celcius = (5.0/9.0) * (val - 32.0);
                    } else {
                        self.temp_celcius = 0.0;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct Temp {
    textbox_celcius: Entity,
    textbox_fahrenheit: Entity,
}

impl Widget for Temp {
    type Ret = Entity;
    type Data = TempState;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.textbox_celcius = Textbox::new("0")
        .on_submit(move |widget, state, textbox|{
            state.insert_update(Event::new(TempEvent::UpdateCelcius(widget.text.to_owned())).origin(entity));
        })
        .build(state, entity, |builder| builder)
        .bind(state, data.temp_celcius);

        Label::new("Celcius = ").build(state, entity, |builder| builder);

        self.textbox_fahrenheit = Textbox::new("32")
        .on_submit(move |widget, state, textbox|{
            state.insert_update(Event::new(TempEvent::UpdateFahrenheit(widget.text.to_owned())).origin(entity));
        })
        .build(state, entity, |builder| builder);

        Label::new("Fahrenheit").build(state, entity, |builder| builder);
        
        entity.set_element(state, "temp").set_layout_type(state, LayoutType::Row)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(data_event) = event.message.downcast() {
            match data_event {
                DataEvent::Changed(node) => {

                }
            }
        }
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &dyn Node, nodes: &NodeMap) {
        
        if let Some(temp_state) = node.downcast_ref::<TempState>() {
            self.textbox_celcius.set_text(state, &temp_state.temp_celcius.to_string());
            let fahrenheit = 32.0 + (9.0/5.0) * temp_state.temp_celcius;
            self.textbox_fahrenheit.set_text(state, &fahrenheit.to_string());
        }
    }
}

fn main() {
    
    let window_description = WindowDescription::new()
        .with_title("Temperature Converter")
        .with_inner_size(500, 100);
    
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        let temp_state = TempState::default().build(state, window);

        Temp::default()
            .build(state, window, |builder| builder)
            .bind(state, temp_state);

    });

    app.run();
}