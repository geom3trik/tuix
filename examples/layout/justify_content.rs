use tuix::*;
use tuix::widgets::*;

// Example showing how to emulate flexbox justify-content in tuix

fn main() {
    let app = Application::new(
        WindowDescription::new().with_title("Justify Content"),
        |state, window| {
            let window_layout_type = LayoutType::Column;

            window
                .set_background_color(state, Color::white())
                .set_layout_type(state, window_layout_type);

            // Flex Start
            let container = Element::new().build(state, window, |builder| {
                builder
                    .set_background_color(Color::rgb(50, 50, 50))
                    .set_child_space(Stretch(1.0))
                    .set_child_left(Pixels(0.0))
                    .set_layout_type(LayoutType::Row)
            });

            Label::new("Flex Start").build(state, container, |builder| {
                builder
                    .set_color(Color::white())
                    .set_position_type(PositionType::SelfDirected)
                    .set_top(Pixels(5.0))
                    .set_left(Pixels(5.0))
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(30.0))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 50, 120))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 50, 160))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 50, 200))
            });

            // Flex End
            let container = Element::new().build(state, window, |builder| {
                builder
                    .set_background_color(Color::rgb(70, 70, 70))
                    .set_child_space(Stretch(1.0))
                    .set_child_right(Pixels(0.0))
                    .set_layout_type(LayoutType::Row)
            });

            Label::new("Flex End").build(state, container, |builder| {
                builder
                    .set_color(Color::white())
                    .set_position_type(PositionType::SelfDirected)
                    .set_top(Pixels(5.0))
                    .set_left(Pixels(5.0))
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(30.0))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 120, 50))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 160, 50))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 200, 50))
            });

            // Center
            let container = Element::new().build(state, window, |builder| {
                builder
                    .set_background_color(Color::rgb(50, 50, 50))
                    .set_child_space(Stretch(1.0))
                    .set_layout_type(LayoutType::Row)
            });

            Label::new("Center").build(state, container, |builder| {
                builder
                    .set_color(Color::white())
                    .set_position_type(PositionType::SelfDirected)
                    .set_top(Pixels(5.0))
                    .set_left(Pixels(5.0))
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(30.0))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(120, 50, 50))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(160, 50, 50))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(200, 50, 50))
            });

            // Space Around
            let container = Element::new().build(state, window, |builder| {
                builder
                    .set_background_color(Color::rgb(70, 70, 70))
                    .set_child_space(Stretch(1.0))
                    .set_col_between(Stretch(2.0))
                    .set_layout_type(LayoutType::Row)
            });

            Label::new("Space Around").build(state, container, |builder| {
                builder
                    .set_color(Color::white())
                    .set_position_type(PositionType::SelfDirected)
                    .set_top(Pixels(5.0))
                    .set_left(Pixels(5.0))
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(30.0))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(120, 120, 50))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(160, 120, 50))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(200, 120, 50))
            });

            // Space Between
            let container = Element::new().build(state, window, |builder| {
                builder
                    .set_background_color(Color::rgb(50, 50, 50))
                    .set_child_top(Stretch(1.0))
                    .set_child_bottom(Stretch(1.0))
                    .set_col_between(Stretch(1.0))
                    .set_layout_type(LayoutType::Row)
            });

            Label::new("Space Between").build(state, container, |builder| {
                builder
                    .set_color(Color::white())
                    .set_position_type(PositionType::SelfDirected)
                    .set_top(Pixels(5.0))
                    .set_left(Pixels(5.0))
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(30.0))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(120, 50, 120))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(160, 50, 160))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(200, 50, 200))
            });

            // Space Evenly
            let container = Element::new().build(state, window, |builder| {
                builder
                    .set_background_color(Color::rgb(70, 70, 70))
                    .set_child_space(Stretch(1.0))
                    .set_col_between(Stretch(1.0))
                    .set_layout_type(LayoutType::Row)
            });

            Label::new("Space Evenly").build(state, container, |builder| {
                builder
                    .set_color(Color::white())
                    .set_position_type(PositionType::SelfDirected)
                    .set_top(Pixels(5.0))
                    .set_left(Pixels(5.0))
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(30.0))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 120, 120))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 160, 160))
            });

            Element::new().build(state, container, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::rgb(50, 200, 200))
            });
        },
    );

    app.run();
}
