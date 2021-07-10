use tuix::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    ConvertText(String),
}

#[derive(Default, Lens, Data)]
pub struct AppData {
    some_text: String,
}

impl Widget for AppData {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppMessage::ConvertText(value) => {
                    self.some_text = value.to_uppercase();
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Container {}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        Textbox::new("Type text here...")
            .on_change(|data, state, textbox| {textbox.emit(state, AppMessage::ConvertText(data.text.clone()));})
            .build(state, entity, |builder| builder.set_child_space(Stretch(1.0)));

        Label::new("")
            .bind(AppData::some_text, |value| value.to_owned())
            .build(state, entity, |builder| builder.set_child_space(Stretch(1.0)));
        
        entity
    }
}

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window|{

        let app_data = Store::new(AppData::default()).build(state, window, |builder| builder);
        
        Container::default().build(state, app_data, |builder| builder);

    });

    app.run();
}