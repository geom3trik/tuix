
use tuix::*;

const STYLE: &str = r#"


    vector_edit>textbox {
        width: 1s;
        min-width: 0px;
        background-color: white;
        child-space: 1s;
        color: black;
        border-width: 1px;
        border-color: #757575;
    }

    vector_edit>dropdown {
        color: black;
        background-color: #d2d2d2;
    }

    dropdown>.container {
        top: 100%;
        width: 100%;
        border-width: 1px;
        border-color: #757575;
        outer-shadow: 2px 2px 5px #80000000;
    }

    dropdown>.header>.label {
        color: black;
        child-space: 1s;
    }

    vector_edit>dropdown .item {
        color: black;
        background-color: white;
        height: 30px;
    }

    vector_edit>dropdown .item:hover {
        background-color: #f2f2f2;
        height: 30px;
    }

    vector_edit .icon {
        display: none;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    vec_edit: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.vec_edit = VectorEdit::new()
            .with_x(255u8)
            .with_y(255u8)
            .with_z(255u8)
            .with_w(255u8)
            .on_change(|data, state, vec_edit| {
                vec_edit.emit(state, CustomEvent::ChangeColor(Color::rgba(data.x, data.y, data.z, data.w)));
                //state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(vec_edit.x, vec_edit.y, vec_edit.z, vec_edit.w))).target(entity));
            })
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });

        entity.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    entity.set_background_color(state, *color);
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Vector Edit")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}