extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/basic_theme.css");

fn main() {

    // Create the app
    let mut app = Application::new(|win_desc, state, window| {


        state.style.parse_theme(THEME);

        //let outer = ScrollContainer::new().build(state, window, |builder| builder.class("container"));
        let outer = Element::new().build(state, window, |builder| builder.class("outer"));

        // let row = HBox::new().build(state, outer, |builder| {
        //     builder
        // });

        // Label::new("Button").build(state, row, |builder| builder);
        // Button::with_label("Press Me").build(state, row, |builder| builder);
    

        let inner = Button::new().build(state, outer, |builder| builder.class("inner"));
        //let _innerinner = Button::new().build(state, inner, |builder| builder.class("innerinner"));

        win_desc.with_title("basic").with_inner_size(600, 600)
    });


    app.run();
}
