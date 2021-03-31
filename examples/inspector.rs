use tuix::*;

use tuix::style::themes::DEFAULT_THEME;


#[derive(Inspectable, Default, Clone)]
pub struct MyData {
    value: String,
    flag: bool,
    #[inspectable(label = "Custom i32")]
    value2: i32,
    #[inspectable(label = "Custom f32", widget = Slider)]
    value3: f32,
}

#[derive(Inspectable, Default, Clone)]
pub struct SomeData {
    name: String,
    other: String,
    more_data: MyData,
}

pub struct Inspector<T: Inspectable> {
    data: T,
}

impl<T: Inspectable> Inspector<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
        }
    }
}

impl<T> Widget for Inspector<T> 
where T: 'static + Inspectable
{
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        self.data.widget(state, entity, "");

        entity
    }
}

fn main() {
    
    

    let app = Application::new(move |state, window| {
        state.add_theme(DEFAULT_THEME);

        // let data = SomeData {
        //     name: "Testy Test".to_string(),
        // };

        window.set_title("Inspector Test");

        //let data: String = "Testy Test".to_string(); 
        let data = SomeData {
            name: "Test Name".to_string(),
            other: "Other Test Name".to_string(),
            more_data: MyData {
                value: "one".to_string(),
                flag: true,
                value2: 100,
                value3: 0.5,
            },
        };

        Inspector::new(data.clone()).build(state, window.entity(), |context| 
            context
                .set_width(Length::Pixels(300.0))
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50,50,50))
                .set_padding(Length::Pixels(10.0))
        );

        // Button::with_label("Button").build(state, root, |context| {
        //     context
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(30.0))
        //         .set_background_color(Color::from("#ff5e1a"))
        //         .set_text_justify(Justify::Center)
        // });

        
    });

    app.run();
}
