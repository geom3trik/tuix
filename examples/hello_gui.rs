extern crate tuix;

use image::{buffer::Pixels};
use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::PropSet;

use tuix::Length;

use tuix::style::themes::DEFAULT_THEME;

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.insert_theme(DEFAULT_THEME);

        let test_button = Button::with_label("Hello Test").build2(state, window);
        test_button.set_width(Length::Pixels(200.0));

        let child_button = Button::with_label("Child").build3(state, &test_button);
        child_button.set_width(Length::Pixels(300.0));
        test_button.set_height(Length::Pixels(300.0));

        let my_button = Button::with_label("Test")
            .build2(state, window)
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(30.0));

        // let my_button = Button::with_label("Hello GUI!").build(state, window, |builder| {
        //     builder.set_text("Button")
        // });

        // my_button.mutate(state, |ctx|
        //     ctx
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(100.0))
        // );

        // if let Some(button) = my_button.testy::<Button>(state) {
        //     println!("{:?}", button.text)
        // }

        //my_button.testy2::<Button,_>(state, |button| println!("{:?}", button.text));

        win_desc.with_title("Hello GUI")
    });

    app.run();
}
