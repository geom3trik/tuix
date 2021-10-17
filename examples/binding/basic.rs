use tuix::*;
use tuix::widgets::*;

#[derive(Lens)]
pub struct AppData {
    value: i32,
}

impl Model for AppData {}

fn main() {
    let app = Application::new(WindowDescription::new(), |state, window|{
        
        let app_data = AppData{value: 30}.build(state, window);

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