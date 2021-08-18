use tuix::*;

const STYLE: &str = r#"

    scroll_container {
        height: 1s;
        background-color: #262626;
        border-width: 0px;
        border-color: black;
    }

    scroll_container>.scrollbar {
        background-color: #464646;
        width: 10px;
    }

    scroll_container:enabled>.scrollbar {
        width: 10px;
        transition: width 0.1 0.0;
    }

    scroll_container:disabled>.scrollbar {
        width: 0px;
        transition: width 0.1 0.0;
    }

    list {
        background-color: white;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
    }

    list>check_button:hover {
        background-color: #e2e2e2;
    }

    list>check_button:active {
        background-color: #c2c2c2;
    }

    list>check_button:checked {
        background-color: #AAAAFF;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }

    scrollbar>.front {
        background-color: red;
    }

"#;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollEvent {
    Scroll(Scroll),
}


#[derive(Debug, Default, Clone, Copy, Lens)]
pub struct ScrollData {
    scroll: Scroll,
}

impl Model for ScrollData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(scroll_event) = event.message.downcast() {
            match scroll_event {
                ScrollEvent::Scroll(scroll) => {
                    self.scroll = *scroll;
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    listbox: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {


        let scroll_data = ScrollData::default().build(state, container);

        let row = Column::new().build(state, scroll_data, |builder| builder);


        Scrollbar::new(ScrollDirection::Horizontal)
        .on_scroll(|data, state, scrollbar|{
            scrollbar.emit(state, ScrollEvent::Scroll(data.scroll));
        })
        .bind(ScrollData::scroll, |scroll| *scroll)
        .build(state, row, |builder| 
            builder
                .set_width(Stretch(1.0))
                .set_height(Pixels(10.0))
                .set_background_color(Color::rgb(150, 150, 150))
        );

        let scroll = ScrollContainer::new()
        .on_scroll(|data, state, scroll_container|{
            scroll_container.emit(state, ScrollEvent::Scroll(data.scroll));
        })
        .bind(ScrollData::scroll, |scroll| *scroll)
        .build(state, row, |builder| 
            builder
                .set_height(Pixels(100.0))
                .set_width(Pixels(210.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_space(Stretch(1.0))
        );

        self.listbox = List::new()
            .build(state, scroll, |builder| {
                builder
                    .set_height(Auto)
            });
        
        CheckButton::with_label("Red")
            .set_checked(true)
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(200, 50, 50)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Green")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 200, 50)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Blue")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 50, 200)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Yellow")
            .set_checked(true)
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(200, 200, 50)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Fuchsia")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(200, 50, 200)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        CheckButton::with_label("Aqua")
            .on_checked(|_, state, button|{
                button.emit(state, CustomEvent::ChangeColor(Color::rgb(50, 200, 200)));
            })
            .build(state, self.listbox, |builder| 
                builder
                    .set_color(Color::black())
            );

        container.set_background_color(state, Color::white()).set_focusable(state, false)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    entity.set_background_color(state, *color);

                    event.consume();
                }
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
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}