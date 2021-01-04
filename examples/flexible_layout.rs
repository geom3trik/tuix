extern crate tuix;

use tuix::widgets::{Element, Button};
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::style::themes::DEFAULT_THEME;

use tuix::style::{Length, Color, JustifyContent, AlignItems};

// This example uses a custom theme defined in the 'custom_theme.css' stylesheet
static CUSTOM_THEME: &'static str = include_str!("themes/custom_theme.css");

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        // Properties defined in CUSTOM_THEME override the same properties defined in DEFAULT_THEME
        state.insert_theme(CUSTOM_THEME);

        // An element is the simplest widget. It has no built in styling and doesn't handle any events.
        let first = Element::new().build(state, window, |builder| 
            builder
                // Allow the element to grow in size to fill the parent (in height)
                .set_flex_grow(1.0)
                // The flexbox way of centering the child elements
                .set_justify_content(JustifyContent::Center)
                .set_align_items(AlignItems::Center)
                
                .set_background_color(Color::rgb(100,50,50))
        );

        Element::new().build(state, window, |builder| 
            builder
                // A flex-grow of 2 rsults in a twice as large element in this case
                .set_flex_grow(2.0)
                .set_background_color(Color::rgb(50,100,50))
        );

        Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50,50,100))
        );

        // The button is now a child of the first element instead of the window
        Button::new().build(state, first, |builder| {
            builder.set_text("Button")
        });

        win_desc.with_title("Flexible Layout")
    });

    app.run();
}
