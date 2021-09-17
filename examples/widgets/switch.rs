use tuix::*;
use rand::prelude::*;

const STYLE: &str = r#"

    switch {
        width: 40px;
        height: 20px;
        background-color: #646464;
        layout-type: row;
        child-space: 2px;
        transition: background-color 0.1 0.0;
        border-radius: 9.5px;
    }
    
    switch>.front {
        left: 1px;
        width: 18px;
        height: 18px;
        background-color: white;
        border-radius: 9px;
        /* border-color: #757575;
        border-width: 1px; */
        transition: left 0.1 0.0;
    }
    
    
    switch:checked>.front {
        left: 21px;
        transition: left 0.1 0.0;
    }
    
    switch:checked {
        background-color: #ff5e1a;
        transition: background-color 0.1 0.0;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    switch: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.switch = Switch::new(false)
            .on_checked(|_, state, switch| {
                switch.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 100, 100)));
            })
            .on_unchecked(|_, state, switch|{
                switch.emit(state, CustomEvent::ChangeColor(Color::rgb(255, 255, 255)));
            })
            .build(state, container, |builder| {
                builder
                    .set_space(Stretch(1.0))
            });

        container.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, container: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    container.set_background_color(state, *color);
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Switch")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}