#![feature(generic_associated_types)]

use tuix::*;


#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    SetData(String),
    SetOther(i32),
}

#[derive(Default, Lens)]
pub struct AppData {
    data: String,
    other: i32,
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetData(value) => {
                    self.data = value.clone();
                    println!("Do This");
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }

                AppEvent::SetOther(value) => {
                    self.other = *value;
                    println!("And This");
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }
            }
        }
    }
}

#[derive(Default)]
pub struct AppWidget {

}

impl Widget for AppWidget {
    type Ret = Entity;
    type Data = (String, i32);
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        


        Button::with_label("TEST")
        .on_press(|data, state, button|{
            println!("Send Event");
            button.emit(state, AppEvent::SetData("test".to_string()));
            button.emit(state, AppEvent::SetOther(45));
        })
        .build(state, entity, |builder|
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_background_color(Color::rgb(50, 50, 50))
                .set_child_space(Stretch(1.0))
        );
        
        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        println!("{:?}", data);
    }
}


fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window| {
        
        let app_data = AppData::default().build(state, window);
        
        AppWidget::default()
        .bind(AppData::data.and(AppData::other), &|data: (&String, &i32)| (data.0.clone(), data.1.clone()))
        .build(state, app_data, |builder| builder);
    });

    app.run();
}