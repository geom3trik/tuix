extern crate tuix;
use tuix::*;

static THEME: &'static str = include_str!("themes/7GUIs_theme.css");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
}

struct Counter {
    value: u32,
    label: Entity,
    increment_button: Entity,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: 0,
            label: Entity::null(),
            increment_button: Entity::null(),
        }
    }
}

impl BuildHandler for Counter {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.label = Label::new("0").build(state, entity, |builder| builder);
        self.increment_button = Button::new()
            .on_press(Event::new(CounterMessage::Increment))
            .build(state, entity, |builder| builder);

        entity.set_element(state, "counter")
    }
}

impl EventHandler for Counter {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        false
    }
}

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.style.parse_theme(THEME);
        Counter::new().build(state, window, |builder| builder);
        win_desc
            .with_title("7GUI's - Counter")
            .with_inner_size(300, 50)
    });

    app.run();
}
