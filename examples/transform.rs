

use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    .one {
        background-color: green;
    }

    .one:hover {
        background-color: blue;
    }

    .two {
        background-color: yellow;
    }

    .two:hover {
        background-color: #909090;
    }
"#;

pub struct TransformWidget {

}

impl TransformWidget {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for TransformWidget {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let one = Element::new().build(state, entity, |builder| 
            builder
                //.set_left(Pixels(100.0))
                .set_top(Pixels(100.0))
                .set_width(Pixels(200.0))
                .set_height(Pixels(100.0))
                //.set_background_color(Color::green())
                .class("one")
        );

        one.set_scale(state, 0.5);
        one.set_rotate(state, 20.0);
        //one.set_translate(state, (30.0, 0.0));
        // one.set_translate(state, (30.0, 0.0));

        let two = Element::new().build(state, one, |builder| 
            builder
                //.set_left(Pixels(20.0))
                .set_top(Pixels(20.0))
                .set_width(Pixels(100.0))
                .set_height(Pixels(50.0))
                //.set_background_color(Color::green())
                .class("two")
        );

        entity.set_background_color(state, Color::red()).set_scale(state, 0.75).set_rotate(state, 20.0)
    }
}

fn main() {
    let app = Application::new(WindowDescription::new(),|state, window| {
        
        state.add_theme(STYLE);
        
        TransformWidget::new().build(state, window.entity(), |builder| builder);
    });

    app.run();
}