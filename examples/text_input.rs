

use tuix::*;
fn main() {
    let app = Application::new(|state, window| {
        
        window.set_title("Hello GUI");

        Textbox::new("ä")
            .build(state, window.entity(), |builder| {
                builder
                    .set_width(Length::Pixels(100.0))
                    .set_height(Length::Pixels(30.0))
                    .set_background_color(Color::from("#202020"))
                    .set_text_justify(Justify::Center)
        });
    });
    app.run();
}