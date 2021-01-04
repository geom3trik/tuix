extern crate tuix;

use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::style::themes::DEFAULT_THEME;

use tuix::style::Length;

// This example uses a custom theme defined in the 'custom_theme.css' stylesheet
static CUSTOM_THEME: &'static str = include_str!("themes/custom_theme.css");

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        // Properties defined in CUSTOM_THEME override the same properties defined in DEFAULT_THEME
        state.insert_theme(CUSTOM_THEME);

        Button::new().build(state, window, |builder| {
            builder
                // These are inline properties which cannot be overriden by a theme
                .set_left(Length::Pixels(100.0))    
                .set_top(Length::Pixels(50.0))
                .set_text("Button")
        });

        win_desc.with_title("Custom Styling")
    });

    app.run();
}
