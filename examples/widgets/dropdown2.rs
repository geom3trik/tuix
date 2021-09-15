use tuix::*;

const STYLE: &str = r#"

    /*
    * {
        border-width: 1px;
        border-color: green;
    }
    */
    
    dropdown {
        border-width: 1px;
        border-color: #555555;
        background-color: white;
    }
    
    dropdown .label {
        child-space: 1s;
        color: black;
    }

    dropdown .icon {
        color: #555555;
    }

    popup {
        background-color: #d2d2d2;
    }


    list {
        border-width: 1px;
        border-color: #555555;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        background-color: #d2d2d2;
    }

    list>check_button:hover {
        background-color: #e2e2e2;
    }

    list>check_button:active {
        background-color: #c2c2c2;
    }

    list>check_button:checked {
        background-color: #c2c2ff;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }
    
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(ColorSelection),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorSelection {
    Red,
    Green,
    Blue,
}

#[derive(Lens)]
pub struct AppData {
    background_color: ColorSelection,
}

impl AppData {
    fn new() -> Self {
        Self {
            background_color: ColorSelection::Red,
        }
    }
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    self.background_color = *color;
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}

#[derive(Default)]
struct Container {
    dropdown: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ColorSelection;
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.dropdown = Dropdown::<()>::new("Test")
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });

        self.dropdown.set_width(state, Pixels(100.0));

        // Spacer
        Element::new().build(state, self.dropdown, |builder| 
            builder
                .set_height(Pixels(5.0))
                .set_focusable(false)
        );
        

        CheckButton::with_label("Red")
            .set_checked(true)
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(ColorSelection::Red));
            })
            .bind(AppData::background_color, |color| *color == ColorSelection::Red)
            .build(state, self.dropdown, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Green")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(ColorSelection::Green));
            })
            .bind(AppData::background_color, |color| *color == ColorSelection::Green)
            .build(state, self.dropdown, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Blue")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(ColorSelection::Blue));
            })
            .bind(AppData::background_color, |color| *color == ColorSelection::Blue)
            .build(state, self.dropdown, |builder| 
                builder
                    .set_color(Color::black())
            );

        // Spacer
        Element::new().build(state, self.dropdown, |builder| 
            builder
                .set_height(Pixels(5.0))
                .set_focusable(false)
        );

        //state.set_focus(container);

        container.set_background_color(state, Color::white()).set_focusable(state, false)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        match data {
            ColorSelection::Red => {
                entity.set_background_color(state, Color::rgb(200, 50, 50));
            }

            ColorSelection::Green => {
                entity.set_background_color(state, Color::rgb(50, 200, 50));
            }
            
            ColorSelection::Blue => {
                entity.set_background_color(state, Color::rgb(50, 50, 200));
            }
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    match code {
                        Code::KeyR => {
                            entity.emit(state, CustomEvent::ChangeColor(ColorSelection::Red));
                        }

                        Code::KeyG => {
                            entity.emit(state, CustomEvent::ChangeColor(ColorSelection::Green));
                        }

                        Code::KeyB => {
                            entity.emit(state, CustomEvent::ChangeColor(ColorSelection::Blue));
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Spinbox")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white()).set_focusable(state, false);

            state.add_theme(STYLE);

            let store = Store::new(AppData::new()).build(state, window, |builder| builder);
            
            Container::default()
                .bind(AppData::background_color, |color| *color)
                .build(state, store, |builder| builder);

        },
    );

    app.run();
}