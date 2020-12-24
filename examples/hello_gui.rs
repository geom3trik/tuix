extern crate tuix;

use tuix::Application;
use tuix::widgets::Button;

use tuix::events::BuildHandler;

use tuix::style::{Color, Length};

fn main() {
    let mut app = Application::new(|win_desc, state, window| {
        
        Button::new().build(state, window, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_left(Length::Pixels(30.0))
                .set_top(Length::Pixels(30.0))
                .set_border_width(1.0)
                .set_border_color(Color::rgb(0,0,0))
                .set_background_color(Color::rgb(50,50,100))
                .set_border_radius(Length::Pixels(5.0))
                .set_text("TEST")
        });        

        win_desc.with_title("Hello GUI")
    });
        
    app.run();
}
