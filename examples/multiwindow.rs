extern crate tuix;
use tuix::*;

use tuix::button::Button;

use tuix_winit::window::WindowWidget2;


const STYLE: &str = r#"
knob {
    background-color: #2e2e2e;
}

knob>.back {
    background-color: #505050;
}

knob>.slider {
    background-color: #10be19;
}

knob>.tick {
    background-color: #c8c8c8;
}

textbox {
    width: 100px;
    height: 30px;
    margin: 5px;
    text-justify: center;
}

label {
    width: 100px;
    height: 30px;
    margin: 5px;
}

button {
    font-size: 16px;
    text-justify: center;
    border-radius: 5px;
    width: 100px;
    height: 30px;
}

button.increment {
    background-color: #326432;
}

button.increment:hover {
    background-color: #3c773c;
}


counter {
    flex-direction: row;
    justify-content: space-evenly;
    align-items: center;
    flex-grow: 1.0;
    padding: 5px;
}

"#;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMessage {
    AddWindow,
}

#[derive(Default)]
struct App {
}

impl App {
    pub fn new() -> Self {
        App {
 
        }
    }
}

impl BuildHandler for App {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.add_theme(STYLE);
        Button::with_label("Spawn Window")
            .on_press(Event::new(AppMessage::AddWindow))
            .build(state, entity, |builder| builder.class("increment").set_width(Length::Pixels(200.0)));
        entity.set_element(state, "counter")
    }
}

impl EventHandler for App {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<AppMessage>() {
            match counter_event {
                AppMessage::AddWindow => {
                    println!("Add Window!");
                    let window = WindowWidget2::default().build(state, entity, |builder| builder);
                    let container = Element::new().build(state, window, |builder| 
                        builder
                            .set_flex_grow(1.0)
                            .set_background_color(Color::rgb(50,50,50))
                            .set_align_items(AlignItems::Center)
                            .set_justify_content(JustifyContent::Center)
                    );

                    ValueKnob::new("Test", 0.0, 0.0, 1.0).build(state, container, |builder| 
                        builder
                        .set_width(Length::Pixels(50.0))
                        .set_height(Length::Pixels(50.0))
                        .set_align_items(AlignItems::Center)
                    );
                }
            }
        }

        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::ValueChanged(val) => {
                    let v = (*val * 255.0) as u8;
                    entity.set_background_color(state, Color::rgb(v,v,v));
                    event.consume();
                }
                _=> {}                
            }

        }
    }
}

fn main() {
    // Create the app
    let app = Application::new(|win_desc, state, window| {
        state.add_theme(STYLE);

        App::default().build(state, window, |builder| builder);

        win_desc.with_title("Multiwindow").with_inner_size(400, 100)
    });

    app.run();
}
