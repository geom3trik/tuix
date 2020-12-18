extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/widget_theme.css");

fn main() {

    // Create the app
    let mut app = Application::new(|window| window.with_title("basic").with_inner_size(600, 600));

    // Get the state from the window
    let state = &mut app.state;

    state.style.parse_theme(THEME);

    // Get the window entity from the state
    let window = state.root;

    let checkbox = Checkbox::new(false).build(state, window, |builder| builder.class("widget"));
    

    app.run();
}