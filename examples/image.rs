/// A simple tuix application showing a widget with an image background
use tuix::*;

use image::GenericImageView;

fn main() {
    let app = Application::new(WindowDescription::new().with_title("Image"),|state, window| {
        let image = image::open("resources/icons/calculator_dark-128.png").unwrap();

        let image_id = state.add_image(image);

        Element::new().build(state, window.entity(), |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(100.0))
                .set_background_image(image_id.clone())
        });

        Element::new().build(state, window.entity(), |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(100.0))
                .set_background_image(image_id.clone())
        });
    });

    app.run();
}
