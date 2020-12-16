extern crate tuix;

use tuix::Application;
use tuix::widgets::Button;

use tuix::events::BuildHandler;

use tuix::style::{Color, Length};
//static THEME: &'static str = include_str!("themes/hello_gui_theme.css");

fn main() {
    let mut app = Application::new(|window| window.with_title("Hello GUI"));

    let state = app.get_state();
    let window = state.root;

    //state.insert_style(THEME);

    Button::new().build(state, window, |builder| {
        builder
            .set_width(Length::Pixels(30.0))
            .set_height(Length::Pixels(30.0))
            .set_border_width(2.0)
            .set_border_color(Color::rgb(0,0,0))
            .set_background_color(Color::rgb(50,50,100))
            .set_border_radius(Length::Pixels(10.0))
    });

    app.run();
}
