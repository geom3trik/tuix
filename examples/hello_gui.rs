extern crate tuix;

use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::PropSet;

use tuix::Length;

use tuix::style::themes::DEFAULT_THEME;

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        let my_button = Button::with_label("Hello GUI!").build(state, window, |builder| {
            builder.set_text("Button")
        });

        my_button.mutate(state, |ctx| 
            ctx
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(100.0))
        );

        if let Some(button) = my_button.testy::<Button>(state) {
            println!("{:?}", button.text) 
        }

        //my_button.testy2::<Button,_>(state, |button| println!("{:?}", button.text));

        win_desc.with_title("Hello GUI")
    });

    app.run();
}
