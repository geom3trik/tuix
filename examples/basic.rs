extern crate tuix;

use tuix::*;

//static THEME: &'static str = include_str!("themes/basic_theme.css");

fn main() {
    // Create the app
    let mut app = Application::new(|win_desc, state, window| {
        match state.insert_stylesheet("examples/themes/basic_theme.css") {
            Ok(_) => {},
            Err(e) => println!("Error loading stylesheet: {}", e)
        }

        let one = Element::new().build(state, window, |builder| builder.class("one"));
        let two = Element::new().build(state, one, |builder| builder.class("two"));
        let three = Element::new().build(state, two, |builder| builder.class("three"));
        // let four = Element::new().build(state, three, |builder| builder.class("four"));
        //let five = Element::new().build(state, four, |builder| builder.class("five"));

        //let outer = ScrollContainer::new().build(state, window, |builder| builder.class("container"));
        //outer = Element::new().build(state, window, |builder| builder.class("outer").set_scaley(1.0));

        // let row = HBox::new().build(state, outer, |builder| {
        //     builder
        // });

        // Label::new("Button").build(state, row, |builder| builder);
        // Button::with_label("Press Me").build(state, row, |builder| builder);

        //let inner = Element::new().build(state, outer, |builder| builder.class("inner").set_clip_widget(outer));
        //let inner = Element::new().build(state, outer, |builder| builder.class("inner2"));
        // let _innerinner = Element::new().build(state, outer, |builder| builder.class("inner2"));

        win_desc.with_title("basic")
    });

    app.run();
}
