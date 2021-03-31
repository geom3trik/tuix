
use tuix::*;

fn main() {
    let app = Application::new(|mut context, window| {
        
        window.set_title("Layout Test");

        context.state().style.main_axis_align.insert(Entity::root(), AxisAlign {
            space_before_first: Units::Pixels(100.0),
            space_between: Units::Pixels(75.0),
            space_after_last: Units::Pixels(50.0),
        });

        let button = Button::with_label("Button 1")
            .build(&mut context)
            .set_background_color(Color::from("#ff5e1a"))
            .set_text_justify(Justify::Center)
            .entity();

        context.state().style.main_axis.insert(button, Axis {
            space_before: Units::Inherit,
            size: Units::Stretch(1.0),
            space_after: Units::Inherit,
        });

        context.state().style.cross_axis.insert(button, Axis {
            space_before: Units::default(),
            size: Units::Stretch(1.0),
            space_after: Units::default(),
        });

        let button2 = Button::with_label("Button 2")
            .build(&mut context)
            .set_background_color(Color::from("#1a5eff"))
            .set_text_justify(Justify::Center)
            .entity();

        context.state().style.main_axis.insert(button2, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(50.0),
            space_after: Units::Inherit,
        });

        context.state().style.cross_axis.insert(button2, Axis {
            space_before: Units::default(),
            size: Units::Stretch(1.0),
            space_after: Units::default(),
        });


    let button3 = Button::with_label("Button 3")
        .build(&mut context)
        .set_background_color(Color::from("#1aff5e"))
        .set_text_justify(Justify::Center)
        .entity();

    context.state().style.main_axis.insert(button3, Axis {
        space_before: Units::Pixels(20.0),
        size: Units::Pixels(100.0),
        space_after: Units::Inherit,
    });

    context.state().style.cross_axis.insert(button3, Axis {
        space_before: Units::default(),
        size: Units::Stretch(1.0),
        space_after: Units::default(),
    });


    });

    app.run();
}