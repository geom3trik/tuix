

use tuix::*;

extern crate rand;
use rand::Rng;


pub fn generate_nest<'a: 'b, 'b>(mut parent: &'b mut Context<'a,Entity>) -> Context<'b, Entity> {
    let random_red: u8 = rand::thread_rng().gen();
    let random_green: u8 = rand::thread_rng().gen();
    let random_blue: u8 = rand::thread_rng().gen();
    let mut first = Button::new().build(parent);
        //.set_background_color(Color::rgb(random_red, random_green, random_blue));
    let first_entity = first.entity();

    first.state().style.main_axis.insert(first_entity, Axis {
        space_before: Units::Pixels(0.0),
        size: Units::Stretch(1.0),
        space_after: Units::Pixels(0.0),
    });

    first.state().style.cross_axis.insert(first_entity, Axis {
        space_before: Units::Pixels(0.0),
        size: Units::Stretch(1.0),
        space_after: Units::Pixels(0.0),
    });

    let random_red: u8 = rand::thread_rng().gen();
    let random_green: u8 = rand::thread_rng().gen();
    let random_blue: u8 = rand::thread_rng().gen();
    let mut second = Button::new().build(parent);
        //.set_background_color(Color::rgb(random_red, random_green, random_blue));

    let entity = second.entity();

    second.state().style.main_axis.insert(entity, Axis {
        space_before: Units::Pixels(0.0),
        size: Units::Pixels(10.0),
        space_after: Units::Pixels(0.0),
    });

    second.state().style.cross_axis.insert(entity, Axis {
        space_before: Units::Pixels(0.0),
        size: Units::Stretch(1.0),
        space_after: Units::Pixels(0.0),
    });

    let mut ret = parent.borrow(first_entity);
    ret.data = first_entity;

    ret
}

const STYLE: &str = r#"

    button {
        background-color: #505050;
        border-width: 1px;
        border-color: black;
    }

    button:hover {
        background-color: white;
    }
"#;



fn main() {
    let app = Application::new(|mut context, window| {
        context.state().add_theme(STYLE);
        window.set_title("Layout Test");

        // context.state().style.main_axis_align.insert(Entity::root(), AxisAlign {
        //     space_before_first: Units::Pixels(100.0),
        //     space_between: Units::Pixels(75.0),
        //     space_after_last: Units::Pixels(50.0),
        // });

        context.state().style.grid_rows.insert(Entity::root(), Grid {
            items: vec![Units::Stretch(1.0), Units::Stretch(1.0)],
            align: AxisAlign {
                space_before_first: Units::Pixels(0.0),
                space_between: Units::Pixels(50.0),
                space_after_last: Units::Pixels(0.0),
            },
        });

        context.state().style.grid_cols.insert(Entity::root(), Grid {
            items: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)],
            align: AxisAlign {
                space_before_first: Units::Pixels(0.0),
                space_between: Units::Pixels(50.0),
                space_after_last: Units::Pixels(0.0),
            },
        });

        context.state().style.layout_type.insert(Entity::root(), LayoutType::Grid);

        let mut button = Button::with_label("Button 1")
            .build(&mut context)
            .set_background_color(Color::from("#ff5e1a"))
            .set_text_justify(Justify::Center);
        let entity = button.entity();

        button.state().style.grid_item.insert(entity, GridItem {
            row_index: 0,
            row_span: 1,
            col_index: 0,
            col_span: 1,
        });

        let mut button = Button::with_label("Button 1")
            .build(&mut context)
            .set_background_color(Color::from("#1a5eff"))
            .set_text_justify(Justify::Center);
        let entity = button.entity();

        button.state().style.grid_item.insert(entity, GridItem {
            row_index: 1,
            row_span: 1,
            col_index: 0,
            col_span: 1,
        });

        let mut button = Button::with_label("Button 1")
            .build(&mut context)
            .set_background_color(Color::from("#1a805e"))
            .set_text_justify(Justify::Center);
        let entity = button.entity();

        button.state().style.grid_item.insert(entity, GridItem {
            row_index: 0,
            row_span: 2,
            col_index: 1,
            col_span: 1,
        });

        let mut button = Button::with_label("Button 1")
            .build(&mut context)
            .set_background_color(Color::from("#1a5eff"))
            .set_text_justify(Justify::Center);
        let entity = button.entity();

        button.state().style.grid_item.insert(entity, GridItem {
            row_index: 0,
            row_span: 1,
            col_index: 2,
            col_span: 1,
        });

        let mut button = Button::with_label("Button 1")
            .build(&mut context)
            .set_background_color(Color::from("#ff5e1a"))
            .set_text_justify(Justify::Center);
        let entity = button.entity();

        button.state().style.grid_item.insert(entity, GridItem {
            row_index: 1,
            row_span: 1,
            col_index: 2,
            col_span: 1,
        });

        

        // button.state().style.main_axis.insert(entity, Axis {
        //     space_before: Units::Inherit,
        //     size: Units::Stretch(1.0),
        //     space_after: Units::Inherit,
        // });

        // button.state().style.cross_axis.insert(entity, Axis {
        //     space_before: Units::default(),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::default(),
        // });

        // let mut one = generate_nest(&mut button)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        //     let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        //     let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        //     let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);
        //     let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Column);
        // let mut one = generate_nest(&mut one)
        //     .set_flex_direction(FlexDirection::Row);


        // let button2 = Button::with_label("Button 2")
        //     .build(&mut context)
        //     .set_background_color(Color::from("#1a5eff"))
        //     .set_text_justify(Justify::Center)
        //     .entity();

        // context.state().style.main_axis.insert(button2, Axis {
        //     space_before: Units::Inherit,
        //     size: Units::Pixels(50.0),
        //     space_after: Units::Inherit,
        // });

        // context.state().style.cross_axis.insert(button2, Axis {
        //     space_before: Units::default(),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::default(),
        // });


        // let button3 = Button::with_label("Button 3")
        //     .build(&mut context)
        //     .set_background_color(Color::from("#1aff5e"))
        //     .set_text_justify(Justify::Center)
        //     .entity();

        // context.state().style.main_axis.insert(button3, Axis {
        //     space_before: Units::Pixels(20.0),
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Inherit,
        // });

        // context.state().style.cross_axis.insert(button3, Axis {
        //     space_before: Units::default(),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::default(),
        // });


    });

    app.run();
}