#![feature(generic_associated_types)]

use tuix::*;

use better_any::{Tid, TidAble};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    SetData(String),
    SetOther(i32),
}

#[derive(Default, Lens, Tid)]
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
    type Data<'a> = Pair<&'a String, &'a i32>;
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

    fn on_update<'a>(&mut self, state: &mut State, entity: Entity, data: &Self::Data<'a>) {
        println!("{:?}", data.left);
        println!("{:?}", data.right);
    }
}


fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window| {
        
        let app_data = AppData::default().build(state, window);
        
        AppWidget::default()
        .bind(AppData::data.and(AppData::other))
        .build(state, app_data, |builder| builder);
    });

    app.run();
}