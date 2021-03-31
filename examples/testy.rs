
use tuix::*;


// #[derive(Default)]
// pub struct Button {

// }

// impl Widget for Button {
//     type Ret = Entity;
//     fn on_build<'a>(&mut self, state: &mut Context) {
  
//     }
// }

#[derive(Default)]
pub struct Row {

}

impl Widget for Row {
    type Ret = ();
    fn on_build(&mut self, context: Context) -> Self::Ret {
        context.set_flex_direction(FlexDirection::Row);
    }
}

// fn main() {
//     let app = Application::new(|ctx, window| {
//         //let mut ctx = Context::new(state, window.entity());
//         Row::default().build(ctx, window.entity(), |ctx| {
//             println!("Entity1: {}", ctx.entity);
//             Button::default().build(ctx, ctx.entity(), |ctx| {
//                 println!("Entity2: {}", ctx.entity);
//             });
//             Button::default().build(ctx, ctx.entity(), |ctx| ctx.do_nothing());
//         });
//         window.set_title("Testy");
        
//         for (key, item) in ctx.state.event_handlers.iter() {
//             println!("item: {:?}", key);
//         }
        
//     });


//     app.run();
// }

fn main() {
    let app = Application::new(|mut ctx, window| {

        window.set_title("Testy");

        let mut row = Row::default().build(&mut ctx);
        
        Button::new().build(&mut row)
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(30.0))
            .set_background_color(Color::red());

        Button::new().build(&mut row)
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(30.0))
            .set_background_color(Color::blue());
    });


    // let app = Application::new(|mut ctx, window| {

    //     window.set_title("Testy");

    //     let mut row = ctx.child(Row::default())
    //         .set_width(Length::Pixels(100.0))
    //         .set_height(Length::Pixels(30.0))
    //         .set_background_color(Color::red());
        
    //     row.child(Button::new())
    //         .set_width(Length::Pixels(100.0))
    //         .set_height(Length::Pixels(30.0))
    //         .set_background_color(Color::red());

    //     row.child(Button::new())
    //         .set_width(Length::Pixels(100.0))
    //         .set_height(Length::Pixels(30.0))
    //         .set_background_color(Color::blue());
    // });



    app.run();
}
