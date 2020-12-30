extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/widget_theme.css");



fn main() {

    // Create the app
    let mut app = Application::new(|window, state, root| {
        
        state.style.parse_theme(THEME);
        // let checkbox = Checkbox::new(false).build(state, root, |builder| builder.class("widget"));
        let switch = Switch::new(false).build(state, root, |builder| builder);
        
        //let dropdown = Dropdown::new()

        // let knob = ControlKnob::new().build(state, root, |builder|
        //     builder
        //         .set_width(Length::Pixels(50.0))
        //         .set_height(Length::Pixels(50.0))
        // );

        // let knob = ValueKnob::new("Dial", 0.0, 0.0, 1.0).build(state, root, |builder|
        //     builder
        //         .set_width(Length::Pixels(50.0))
        //         .set_height(Length::Pixels(75.0))
        // );

        

        window.with_title("basic").with_inner_size(600, 600)
    
    });

    

    app.run();
}