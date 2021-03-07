use tuix::*;

fn main() {
    let app = Application::new(|win_desc, state, window| {
        window
            .set_justify_content(state, JustifyContent::Center)
            .set_align_items(state, AlignItems::Center);

        Textbox::new("Some text").build(state, window, |builder| 
            builder
                .set_font("emoji")
                .set_font_size(20.0)
                .set_text_justify(Justify::Center)
                .set_width(Length::Pixels(200.0))
                .set_height(Length::Pixels(100.0))
                .set_background_color(Color::rgb(50,50,50))
        );

        // Label::new("Hello 👍").build(state, window, |builder| {
        //     builder
        //         .set_font("emoji")
        //         .set_font_size(50.0)
        //         .set_text_justify(Justify::Center)
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(100.0))
        // });

        // Label::new("Test of a line").build(state, window, |builder|
        //     builder
        //         .set_font_size(50.0)
        //         .set_text_justify(Justify::Center)
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(100.0))
        // );

        win_desc.with_inner_size(300, 300).with_title("Tic Tac Toe")
    });

    app.run();
}
