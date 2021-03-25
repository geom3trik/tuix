/// A simple tuix application showing a widget with an image background

use tuix::*;

fn main() {
    let app = Application::new(|state, window| {
        
        //state.load_image();

        window.set_title("Hello GUI");

        Button::with_label("Button")
            .build(state, window, |builder| {
                builder
                    .set_width(Length::Pixels(100.0))
                    .set_height(Length::Pixels(30.0))
                    .set_background_color(Color::from("#ff5e1a"))
                    .set_text_justify(Justify::Center)
            });

    });

    app.run();
}

