extern crate tuix;

use tuix::*;

//static THEME: &'static str = include_str!("themes/basic_theme.css");

fn main() {
    // Create the app
    let app = Application::new(|win_desc, state, window| {
        match state.add_stylesheet("examples/themes/basic_theme.css") {
            Ok(_) => {}
            Err(e) => println!("Error loading stylesheet: {}", e),
        }

        window
            .set_background_color(state, Color::rgb(255, 255, 255))
            .set_align_items(state, AlignItems::FlexStart);
            // .set_flex_direction(state, FlexDirection::Row)
            // .set_align_items(state, AlignItems::FlexStart);

        //window.set_flex_direction(state, FlexDirection::Row);
        // let levels = HBox::new().build(state, window, |builder| builder.set_width(Length::Pixels(50.0)).set_background_color(Color::red()));

        // let left_channel_level = AudioLevelBar::new().build(state, levels, |builder| builder.set_flex_grow(1.0).set_background_color(Color::green()));
        // let right_channel_level = AudioLevelBar::new().build(state, levels, |builder| builder.set_flex_grow(1.0).set_background_color(Color::green()));
        //let container = Element::new().build(state, window, |builder| builder.class("container"));
        // let _one = Element::new().build(state, window, |builder| {
        //     builder.class("one").set_background_gradient(
        //         LinearGradient::new(Direction::TopToBottom)
        //             .add_stop(GradientStop::new(Length::Pixels(0.0), Color::rgb(90,90,90)))
        //             .add_stop(GradientStop::new(Length::Pixels(30.0), Color::rgb(50,50,50))),
        //     ).set_text("Button")
        // });

        let two = Element::new().build(state, window, |builder| builder.class("two"));
        Element::new().build(state, two, |builder| builder.class("three"));
        Element::new().build(state, two, |builder| builder.class("four"));
        let three = Element::new().build(state, two, |builder| builder.class("five"));
        Element::new().build(state, three, |builder| builder.class("six"));
        // let four = Element::new().build(state, one, |builder| builder.class("four"));

        // let test = VectorEdit::<f32>::new().build(state, window, |builder| builder
        //     .set_width(Length::Pixels(100.0))
        //     .set_height(Length::Pixels(30.0))
        //     .set_background_color(Color::blue())
        // );

        //let test= Spinner::new(0).build(state, window, |builder| builder);

        // let (_, _, dropdown) = Dropdown::new("Dropdown").build(state, window, |builder|
        //     builder
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(30.0))
        // );
        // Item::new("Item 1", "Item 1").build(state, dropdown, |builder| builder);
        // Item::new("Item 2", "Item 2").build(state, dropdown, |builder| builder);
        // Item::new("Item 3", "Item 3").build(state, dropdown, |builder| builder);

        // Checkbox::new(false)
        //     .build(state, footer, |builder| {
        //         builder.set_font("Icons").class("snap").set_background_color(Color::yellow())
        //     });

        // // Zoom Controls
        // let zoom_controls =
        //     Element::new().build(state, footer, |builder| builder.class("zoom_controls").set_background_color(Color::red()));

        // Button::new()
        //     .build(state, zoom_controls, |builder| {
        //         builder.set_font("Icons").class("zoom").class("first")
        //     });

        // Element::new().build(state, zoom_controls, |builder| builder.class("zoom"));

        // Button::new()
        //     .build(state, zoom_controls, |builder| {
        //         builder.set_font("Icons").class("zoom").class("last")
        //     });

        // let one = Element::new().build(state, window, |builder| {
        //     builder
        //         .class("one")
        //         .set_box_shadow_h_offset(Length::Pixels(2.5))
        //         .set_box_shadow_v_offset(Length::Pixels(2.5))
        //         .set_box_shadow_blur(Length::Pixels(10.0))
        //         .set_box_shadow_color(Color::rgba(0, 0, 0, 128))
        // });
        //let two = Element::new().build(state, one, |builder| builder.class("two"));
        //let three = Element::new().build(state, two, |builder| builder.class("three"));
        // let four = Element::new().build(state, three, |builder| builder.class("four"));
        //let five = Element::new().build(state, four, |builder| builder.class("five"));

        //let outer = ScrollContainer::new().build(state, window, |builder| builder.class("container"));
        //outer = Element::new().build(state, window, |builder| builder.class("outer").set_scaley(1.0));

        // let row = HBox::new().build(state, outer, |builder| {
        //     builder
        // });

        // Label::new("Button").build(state, row, |builder| builder);
        // Button::with_label("Press Me").build(state, row, |builder| builder);

        //let inner = Element::new().build(state, outer, |builder| builder.class("inner").set_clip_widget(outer));
        //let inner = Element::new().build(state, outer, |builder| builder.class("inner2"));
        // let _innerinner = Element::new().build(state, outer, |builder| builder.class("inner2"));

        win_desc.with_title("basic")
    });

    app.run();
}
